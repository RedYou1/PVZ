use config::{LevelConfig, RowType};
use rand::Rng;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};
use std::time::Duration;

pub mod config;
mod draws;

use crate::{
    entity::Entity,
    plants::{nenuphar::Nenuphar, Plant},
    projectile::{DamageType, Projectile},
    shop::Shop,
    sun::Sun,
    textures::{self, draw_string},
    zombie::{zombie_from_id, Zombie},
};

pub struct Level {
    pub showing_zombies: bool,
    pub suns: Vec<Sun>,
    pub next_sun: Duration,
    pub plants: Vec<Vec<Option<Box<dyn Plant>>>>,
    pub zombies: Vec<Vec<Box<dyn Zombie>>>,
    pub projectiles: Vec<Vec<Box<dyn Projectile>>>,
    pub config: LevelConfig,
    pub shop: Shop,
    pub end: Option<bool>,
}

impl Level {
    pub fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let scale_x = |x: i32| x as f32 * 1280. / width as f32;
        let scale_y = |y: i32| y as f32 * 720. / height as f32;

        if let Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } = event
        {
            self.showing_zombies = false;
        }

        if let Event::MouseMotion { x, y, .. } = event {
            let x = scale_x(x) as i32;
            let y = scale_y(y) as i32;
            for i in self
                .suns
                .iter()
                .enumerate()
                .filter_map(|(i, sun)| {
                    if sun.x <= x
                        && sun.x + sun.width() as i32 >= x
                        && sun.y <= y
                        && sun.y + sun.height() as i32 >= y
                    {
                        Some(i)
                    } else {
                        None
                    }
                })
                .rev()
                .collect::<Vec<usize>>()
            {
                self.shop.money += 25;
                self.suns.remove(i);
            }
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
            &textures::textures().maps[self.config.map as usize],
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
        self.draw_plants(canvas)?;
        self.draw_zombies(canvas)?;
        self.draw_projectiles(canvas)?;
        if !self.showing_zombies {
            self.shop.draw(canvas, &self.config)?;
        }
        self.draw_suns(canvas)?;
        if let Some(end) = self.end {
            draw_string(
                canvas,
                Rect::new(320, 180, 640, 540),
                if end { "Victoire" } else { "Défaite" },
                Color::RGB(255, 255, 255),
            )?;
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

        for y in 0..self.zombies.len() {
            for i in 0..self.zombies[y].len() {
                let prev_pos = self.zombies[y][i].pos();
                self.zombies[y][i].update(!self.showing_zombies, elapsed)?;
                if self.zombies[y][i].pos()
                    >= 1. - self.config.left as f32 / 1280.
                        + self.zombies[y][i].width() as f32 / 1280.
                {
                    self.end = Some(false);
                } else {
                    self.do_damage_to_plant(prev_pos, y, i, elapsed);
                }
            }
        }
        if let Some(false) = self.end {
            return Ok(());
        }

        for y in 0..self.projectiles.len() {
            let mut indx = Vec::new();
            for i in 0..self.projectiles[y].len() {
                self.projectiles[y][i].update(!self.showing_zombies, elapsed)?;

                let proj = self.projectiles[y][i].as_ref();

                if proj.to_remove() {
                    indx.insert(0, i);
                    continue;
                }

                let mut zombie_to_remove = self.do_damage_to_zombies(
                    y,
                    proj.x(),
                    proj.width() as i32,
                    proj.damage_amount(),
                    proj.damage_type(),
                );
                if zombie_to_remove.0 {
                    indx.insert(0, i);
                }
                zombie_to_remove.1.sort();
                zombie_to_remove.1.reverse();
                zombie_to_remove.1.dedup();
                for zombie_index in zombie_to_remove.1 {
                    self.zombies[y].remove(zombie_index);
                }
            }
            for i in indx {
                self.projectiles[y].remove(i);
            }
        }

        for sun in self.suns.iter_mut() {
            sun.update(!self.showing_zombies, elapsed)?;
        }
        if self.next_sun > elapsed {
            self.next_sun -= elapsed
        } else {
            self.next_sun = Duration::new(5, 0) - elapsed + self.next_sun;
            self.suns
                .push(Sun::new(rand::thread_rng().gen_range(0..1220), 40));
        }

        self.spawn_projectiles();

        if !self.showing_zombies && !self.config.wait.is_empty() {
            if let Some(&f) = self.config.wait.first() {
                if elapsed >= f {
                    elapsed -= f;
                    self.config.wait.remove(0);
                    let mut z = self.config.zombies.remove(0);
                    let mut rng = rand::thread_rng();
                    let mut offsets: Vec<f32> = (0..self.config.rows.len()).map(|_| 0.).collect();
                    while !z.is_empty() {
                        let i = rng.gen_range(0..z.len());
                        let mut z = zombie_from_id(z.remove(i).0);
                        let i = rng.gen_range(0..self.config.rows.len()) as usize;
                        z.set_pos(offsets[i]);
                        offsets[i] -= 0.006;
                        self.zombies[i].push(z);
                    }
                }
            }
            if let Some(f) = self.config.wait.first_mut() {
                *f -= elapsed;
            }
        }

        Ok(())
    }

    fn do_damage_to_plant(&mut self, prev_pos: f32, y: usize, i: usize, elapsed: Duration) {
        let z = self.zombies[y][i].as_mut();
        if let Some(x) = self
            .config
            .coord_to_pos_x((1280. - prev_pos * 1280.) as i32)
        {
            if let Some(plant) = self.plants[y][x].as_mut() {
                z.set_pos(prev_pos);
                let diff = elapsed.as_secs_f32() * if z.freezed() { 0.5 } else { 1. };
                if plant.health().as_secs_f32() < diff {
                    self.plants[y][x] =
                        if self.config.rows[y] == RowType::Water && !plant.is_nenuphar() {
                            Some(Box::new(Nenuphar::new()))
                        } else {
                            None
                        }
                } else {
                    *plant.health() -= Duration::from_secs_f32(diff);
                }
            }
        } else if let Some(x) = self.config.coord_to_pos_x((1280. - z.pos() * 1280.) as i32) {
            if let Some(p) = self.plants[y][x].as_ref() {
                let pos =
                    (self.config.pos_to_coord_x(x) as f32 + p.width() as f32 - 1280.) / -1280.;
                if z.pos() > pos {
                    z.set_pos(pos);
                }
            }
        }
    }

    fn spawn_projectiles(&mut self) {
        for (y, ps) in self.plants.iter_mut().enumerate() {
            for (x, plant) in ps.iter_mut().enumerate() {
                if let Some(plant) = plant {
                    let mut spawns = plant.should_spawn(
                        self.config.pos_to_coord_x(x) + plant.width() as i32 / 2,
                        self.config.pos_to_coord_y(y),
                        y,
                        self.config.rows.len() - 1,
                        &self.zombies,
                    );
                    self.suns.append(&mut spawns.0);
                    for (y, proj) in spawns.1 {
                        self.projectiles[y].push(proj);
                    }
                }
            }
        }
    }

    fn do_damage_to_zombies(
        &mut self,
        y: usize,
        proj_x: i32,
        proj_width: i32,
        proj_damage: usize,
        proj_type: DamageType,
    ) -> (bool, Vec<usize>) {
        let mut zombies = self.zombies[y]
            .iter_mut()
            .enumerate()
            .filter_map(|(i, z)| {
                let zx = 1280 - (z.pos() * 1280.).floor() as i32 + z.hit_box().0 as i32;
                if zx + z.hit_box().1 as i32 >= proj_x && zx <= proj_x + proj_width {
                    Some((i, z.pos()))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, f32)>>();
        zombies.sort_by(|(_, pos1), (_, pos2)| pos2.total_cmp(pos1));
        if let Some(&(iz, _)) = zombies.first() {
            (true, self.hit_zombie(y, iz, proj_damage, proj_type, false))
        } else {
            (false, Vec::new())
        }
    }

    fn hit_zombie(
        &mut self,
        y: usize,
        zombie_index: usize,
        damage_amount: usize,
        damage_type: DamageType,
        propagated: bool,
    ) -> Vec<usize> {
        let hit = self.zombies[y][zombie_index].hit(damage_amount, damage_type, propagated);
        let mut to_remove = Vec::new();
        if hit.0 {
            to_remove.push(zombie_index)
        }
        if hit.1 && !propagated {
            to_remove.extend(self.propagate(y, zombie_index, damage_amount, damage_type));
        }
        to_remove
    }

    fn propagate(
        &mut self,
        y: usize,
        zombie_index: usize,
        damage_amount: usize,
        damage_type: DamageType,
    ) -> Vec<usize> {
        let size = {
            let oz = self.zombies[y][zombie_index].as_ref();
            let zx = 1280 - (oz.pos() * 1280.).floor() as i32 + oz.hit_box().0 as i32;
            (zx, zx + oz.hit_box().1 as i32)
        };
        let mut to_remove = Vec::new();
        for zombie_index2 in self.zombies[y]
            .iter_mut()
            .enumerate()
            .filter_map(|(i, z)| {
                let zx = 1280 - (z.pos() * 1280.).floor() as i32 + z.hit_box().0 as i32;
                if zx + z.hit_box().1 as i32 >= size.0 && zx <= size.1 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>()
        {
            if zombie_index != zombie_index2 {
                to_remove.extend(self.hit_zombie(
                    y,
                    zombie_index2,
                    damage_amount,
                    damage_type,
                    true,
                ));
            }
        }
        to_remove
    }
}
