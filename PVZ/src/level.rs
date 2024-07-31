use std::{
    fs,
    io::{self},
    time::Duration,
};

use rand::Rng;
use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, rect::Rect,
    render::Canvas, video::Window,
};

use crate::{
    plant::Plant,
    projectile::Projectile,
    shop::Shop,
    textures,
    win::set_scale,
    zombie::{zombie_from_id, Zombie},
};

/// Slots:
///     left: 308 + x * 97
///     top: 102 + y * 117
///     width: 80
///     heigth: 106
pub struct Level {
    pub showing_zombies: bool,
    pub plants: Vec<Vec<Option<Box<dyn Plant>>>>,
    pub zombies: Vec<Vec<Box<dyn Zombie>>>,
    pub projectiles: Vec<Vec<Box<dyn Projectile>>>,
    pub config: LevelConfig,
    pub shop: Shop,
    pub end: Option<bool>,
}

impl Level {
    pub fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        if let Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } = event
        {
            self.showing_zombies = false;
        }

        if let Some((x, y, plant)) =
            self.shop
                .event(&self.config, self.plants.as_ref(), canvas, event)?
        {
            self.plants[y][x] = Some(plant);
        }
        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.copy(
            textures::maps(self.config.map.into()),
            Some(Rect::new(
                if self.showing_zombies { 238 } else { 0 },
                0,
                762,
                429,
            )),
            Rect::new(0, 0, 1280, 720),
        )?;
        if self.showing_zombies {
            let mut t: Vec<&(u8, i32, i32)> = self.config.zombies.iter().flatten().collect();
            t.sort_by(|(_, _, y1), (_, _, y2)| y1.cmp(y2));
            for &(z, x, y) in t {
                let z = zombie_from_id(z);
                canvas.copy(
                    z.texture(),
                    None,
                    Rect::new(x, y, z.width() as u32, z.height() as u32),
                )?;
            }
            return Ok(());
        }
        for (y, ps) in self.plants.iter().enumerate() {
            for (x, p) in ps.iter().enumerate() {
                if let Some(p) = p {
                    canvas.copy(
                        p.texture(),
                        None,
                        Rect::new(
                            self.config.pos_to_coord_x(x) + 5,
                            self.config.pos_to_coord_y(y) + 5,
                            self.config.col_width() - 10,
                            self.config.row_heigth() - 10,
                        ),
                    )?;
                }
            }
        }
        for (y, zs) in self.zombies.iter().enumerate() {
            let mut zs: Vec<&dyn Zombie> = zs.iter().map(|z| z.as_ref()).collect();
            zs.sort_by(|&z1, &z2| z2.pos().total_cmp(&z1.pos()));
            for z in zs {
                canvas.copy(
                    z.texture(),
                    None,
                    Rect::new(
                        1280 - (z.pos() * 1280.).floor() as i32,
                        self.config.pos_to_coord_y(y) + self.config.row_heigth() as i32
                            - z.height() as i32,
                        z.width().into(),
                        z.height().into(),
                    ),
                )?;
            }
        }
        for (y, projs) in self.projectiles.iter().enumerate() {
            for proj in projs {
                canvas.copy(
                    proj.texture(),
                    None,
                    Rect::new(
                        proj.x(),
                        self.config.pos_to_coord_y(y) + self.config.row_heigth() as i32 / 2
                            - proj.height() as i32 / 2,
                        proj.width().into(),
                        proj.height().into(),
                    ),
                )?;
            }
        }
        if !self.showing_zombies {
            self.shop.draw(canvas, &self.config)?;
        }
        if let Some(end) = self.end {
            const SCALE: i16 = 10;
            set_scale(canvas, SCALE as f32, SCALE as f32)?;
            canvas.string(
                1280 / SCALE / 2 - 14,
                720 / SCALE / 2 - 3,
                if end { "WIN" } else { "LOSE" },
                Color::RGB(255, 255, 255),
            )?;
            set_scale(canvas, 1., 1.)?;
        }
        Ok(())
    }

    pub fn update(&mut self, _: &mut Canvas<Window>, mut elapsed: Duration) -> Result<(), String> {
        if self.end.is_some() {
            return Ok(());
        }
        if !self.zombies.iter().flatten().any(|_| true) && self.config.wait.is_empty() {
            self.end = Some(true);
            return Ok(());
        }

        for p in self.plants.iter_mut().flatten().flatten() {
            p.update(!self.showing_zombies, elapsed)?;
        }

        for z in self.zombies.iter_mut().flatten() {
            z.update(!self.showing_zombies, elapsed)?;
            if z.pos() >= 1. - self.config.left as f32 / 1280. + z.width() as f32 / 1280. {
                self.end = Some(false);
            }
        }
        if let Some(false) = self.end {
            return Ok(());
        }

        for (y, projs) in self.projectiles.iter_mut().enumerate() {
            let mut indx = Vec::new();
            for (i, proj) in projs.iter_mut().enumerate() {
                proj.update(!self.showing_zombies, elapsed)?;
                if let Some(&iz) = self.zombies[y]
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(i, z)| {
                        let zx = 1280 - (z.pos() * 1280.).floor() as i32;
                        if zx + z.width() as i32 >= proj.x() && zx <= proj.x() + proj.width() as i32
                        {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>()
                    .first()
                {
                    if self.zombies[y][iz].hit() {
                        self.zombies[y].remove(iz);
                        self.shop.money += 1;
                    }
                    indx.insert(0, i);
                }
            }
            for i in indx {
                projs.remove(i);
            }
        }

        for (y, ps) in self.plants.iter_mut().enumerate() {
            for (x, plant) in ps.iter_mut().enumerate() {
                if let Some(plant) = plant {
                    for (y, proj) in plant.should_spawn(
                        self.config.pos_to_coord_x(x) + plant.width() as i32 / 2,
                        y,
                        self.config.rows as usize - 1,
                    ) {
                        self.projectiles[y].push(proj);
                    }
                }
            }
        }

        if !self.showing_zombies && !self.config.wait.is_empty() {
            if let Some(&f) = self.config.wait.first() {
                if elapsed >= f {
                    elapsed -= f;
                    self.config.wait.remove(0);
                    if let Some(z) = self.config.zombies.first() {
                        let mut rng = rand::thread_rng();
                        let mut offsets: Vec<f32> = (0..self.config.rows).map(|_| 0.).collect();
                        for &(zombie, _, _) in z {
                            let mut z = zombie_from_id(zombie);
                            let i = rng.gen_range(0..self.config.rows) as usize;
                            z.set_pos(offsets[i]);
                            offsets[i] -= 0.006;
                            self.zombies[i].push(z);
                        }
                    }
                    self.config.zombies.remove(0);
                }
            }
            if let Some(f) = self.config.wait.first_mut() {
                *f -= elapsed;
            }
        }

        Ok(())
    }

    #[allow(clippy::unwrap_in_result)]
    pub fn load_config(path: &str) -> std::io::Result<Self> {
        let mut data = fs::read(path)?;

        let map = data.remove(0);

        let mut data2 = fs::read(format!("assets/maps/{map}.data"))?;
        let top = u16::from_le_bytes([data2.remove(0), data2.remove(0)]);
        let left = u16::from_le_bytes([data2.remove(0), data2.remove(0)]);
        let width = u16::from_le_bytes([data2.remove(0), data2.remove(0)]);
        let height = u16::from_le_bytes([data2.remove(0), data2.remove(0)]);
        let rows = data2.remove(0);
        let cols = data2.remove(0);

        let money = u32::from_le_bytes([
            data.remove(0),
            data.remove(0),
            data.remove(0),
            data.remove(0),
        ]);
        let waves = data.remove(0).into();

        let wait = data
            .chunks_exact(8)
            .take(waves)
            .map(|bytes| {
                Duration::from_millis(u64::from_le_bytes(bytes.try_into().expect("chunk")))
            })
            .collect();
        data.drain(0..waves * 8);

        let mut rng = rand::thread_rng();
        let z_rng_x1 = left + width - 305;
        let z_rng_y1 = top + height / rows as u16;
        let z_rng_y2 = top + height;
        let zombies = (0..waves)
            .map(|_| {
                let types = data.remove(0).into();

                let zombies = data
                    .chunks_exact(2)
                    .take(types)
                    .flat_map(|bytes| {
                        let z = zombie_from_id(bytes[0]);
                        (0..bytes[1])
                            .map(|_| {
                                (
                                    bytes[0],
                                    rng.gen_range((z_rng_x1 as i32)..(1280 - z.width() as i32)),
                                    rng.gen_range(
                                        (z_rng_y1 as i32 - z.height() as i32)
                                            ..(z_rng_y2 as i32 - z.height() as i32),
                                    ),
                                )
                            })
                            .collect::<Vec<(u8, i32, i32)>>()
                    })
                    .collect();

                data.drain(0..types * 2);

                zombies
            })
            .collect();

        if !data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Wrong format".to_owned(),
            ));
        }

        Ok(Self {
            showing_zombies: true,
            plants: (0..rows)
                .map(|_| (0..cols).map(|_| None).collect())
                .collect(),
            zombies: (0..rows).map(|_| Vec::with_capacity(16)).collect(),
            projectiles: (0..rows).map(|_| Vec::with_capacity(4)).collect(),
            config: LevelConfig {
                map,
                top,
                left,
                width,
                height,
                rows,
                cols,
                wait,
                zombies,
            },
            shop: Shop::new(money),
            end: None,
        })
    }
}

pub struct LevelConfig {
    map: u8,

    top: u16,
    left: u16,
    width: u16,
    height: u16,

    rows: u8,
    cols: u8,

    wait: Vec<Duration>,
    #[allow(clippy::type_complexity)]
    zombies: Vec<Vec<(u8, i32, i32)>>,
}

impl LevelConfig {
    pub const fn coord_to_pos_x(&self, x: i32) -> Option<usize> {
        if x < self.left as i32 || x > self.left as i32 + self.width as i32 {
            None
        } else {
            Some((x as usize - self.left as usize) * self.cols as usize / self.width as usize)
        }
    }
    pub const fn coord_to_pos_y(&self, y: i32) -> Option<usize> {
        if y < self.top as i32 || y > self.top as i32 + self.height as i32 {
            None
        } else {
            Some((y as usize - self.top as usize) * self.rows as usize / self.height as usize)
        }
    }

    pub const fn pos_to_coord_x(&self, x: usize) -> i32 {
        x as i32 * self.width as i32 / self.cols as i32 + self.left as i32
    }
    pub const fn pos_to_coord_y(&self, y: usize) -> i32 {
        y as i32 * self.height as i32 / self.rows as i32 + self.top as i32
    }

    pub const fn row_heigth(&self) -> u32 {
        self.height as u32 / self.rows as u32
    }

    pub const fn col_width(&self) -> u32 {
        self.width as u32 / self.cols as u32
    }
}
