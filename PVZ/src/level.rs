use std::{fs, io, time::Duration};

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
    zombie::{zombie_from_id, Zombie},
};

/// Slots:
///     left: 308 + x * 97
///     top: 102 + y * 117
///     width: 80
///     heigth: 106
pub struct Level {
    pub showing_zombies: bool,
    pub plants: Vec<[Option<Box<dyn Plant>>; 9]>,
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

        if let Some((x, y, plant)) = self.shop.event(self.plants.as_ref(), canvas, event)? {
            self.plants[y][x] = Some(plant);
        }
        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.copy(
            textures::map(),
            Some(Rect::new(
                if self.showing_zombies { 238 } else { 0 },
                0,
                762,
                429,
            )),
            Rect::new(0, 0, 1280, 720),
        )?;
        if self.showing_zombies {
            for (y, &(z, _)) in self.config.zombies.iter().flatten().enumerate() {
                canvas.copy(
                    zombie_from_id(z).texture(),
                    None,
                    Rect::new(1000, y as i32 * 141, 164 * 141 / 274, 141),
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
                        Rect::new(308 + x as i32 * 97, 102 + y as i32 * 117, 80, 106),
                    )?;
                }
            }
        }
        for (y, zs) in self.zombies.iter().enumerate() {
            for z in zs {
                canvas.copy(
                    z.texture(),
                    None,
                    Rect::new(
                        1280 - (z.pos() * 1280.).floor() as i32,
                        219 + y as i32 * 117 - z.height() as i32,
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
                        155 + y as i32 * 117 - proj.height() as i32 / 2,
                        proj.width().into(),
                        proj.height().into(),
                    ),
                )?;
            }
        }
        if !self.showing_zombies {
            self.shop.draw(canvas)?;
        }
        if let Some(end) = self.end {
            const SCALE: i16 = 10;
            canvas.set_scale(SCALE as f32, SCALE as f32)?;
            canvas.string(
                1280 / SCALE / 2 - 14,
                720 / SCALE / 2 - 3,
                if end { "WIN" } else { "LOSE" },
                Color::RGB(255, 255, 255),
            )?;
            canvas.set_scale(1., 1.)?;
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
            if z.pos() >= 1. - 200. / 1280. {
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
                    for (y, proj) in
                        plant.should_spawn(308 + x as i32 * 97 + plant.width() as i32 / 2, y)
                    {
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
                        for &(zombie, amount) in z {
                            for _ in 0..amount {
                                self.zombies[rng.gen_range(0..5)].push(zombie_from_id(zombie));
                            }
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

        let rows = data.remove(0);
        let waves = data.remove(0).into();

        let wait = data
            .chunks_exact(8)
            .take(waves)
            .map(|bytes| {
                Duration::from_millis(u64::from_le_bytes(bytes.try_into().expect("chunk")))
            })
            .collect();
        data.drain(0..waves * 8);

        let zombies = (0..waves)
            .map(|_| {
                let types = data.remove(0).into();

                let zombies = data
                    .chunks_exact(2)
                    .take(types)
                    .map(|bytes| (bytes[0], bytes[1]))
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

        const NONE_PLANT: std::option::Option<Box<dyn Plant>> = None;
        Ok(Self {
            showing_zombies: true,
            plants: (0..rows).map(|_| [NONE_PLANT; 9]).collect(),
            zombies: (0..rows).map(|_| Vec::with_capacity(16)).collect(),
            projectiles: (0..rows).map(|_| Vec::with_capacity(4)).collect(),
            config: LevelConfig { wait, zombies },
            shop: Shop::new(),
            end: None,
        })
    }
}

pub struct LevelConfig {
    wait: Vec<Duration>,
    zombies: Vec<Vec<(u8, u8)>>,
}
