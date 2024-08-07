use collision::{do_damage_to_plant, do_damage_to_zombies};
use config::LevelConfig;
use rand::Rng;
use sdl2::{
    event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};
use std::time::Duration;

mod collision;
pub mod config;
mod draws;

use crate::{
    entity::Entity,
    plants::Plant,
    projectile::Projectile,
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
    pub fn event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        pause: &mut bool,
    ) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let scale_x = |x: i32| x as f32 * 1280. / width as f32;
        let scale_y = |y: i32| y as f32 * 720. / height as f32;

        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.showing_zombies = false;
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let x = scale_x(x);
                let y = scale_y(y);
                if (1120.0..=1260.0).contains(&x) && (10.0..=50.0).contains(&y) {
                    *pause = !*pause;
                } else if self.showing_zombies
                    && (1070.0..=1270.0).contains(&x)
                    && (670.0..=710.0).contains(&y)
                {
                    self.showing_zombies = false;
                }
            }
            Event::MouseMotion { x, y, .. } => {
                let x = scale_x(x) as i32;
                let y = scale_y(y) as i32;
                for i in self
                    .suns
                    .iter()
                    .enumerate()
                    .filter_map(|(i, sun)| {
                        if sun.x <= x
                            && sun.x + sun.width() as i32 >= x
                            && sun.y as i32 <= y
                            && sun.y as i32 + sun.height() as i32 >= y
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
            _ => {}
        }

        self.shop
            .event(&self.config, self.plants.as_mut(), canvas, event)?;
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

            canvas.set_draw_color(Color::BLACK);
            canvas.fill_rect(Rect::new(1120, 10, 150, 40))?;
            draw_string(canvas, Rect::new(1120, 10, 150, 40), "Menu")?;
            canvas.fill_rect(Rect::new(1070, 670, 200, 40))?;
            draw_string(canvas, Rect::new(1070, 670, 200, 40), "Commencer")?;
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
            )?;
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(1120, 10, 150, 40))?;
        draw_string(canvas, Rect::new(1120, 10, 150, 40), "Menu")?;
        Ok(())
    }

    pub fn update(&mut self, _: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        if self.showing_zombies {
            return Ok(());
        }
        if self.end.is_some() {
            return Ok(());
        }
        if !self.zombies.iter().flatten().any(|_| true) && self.config.waits.is_empty() {
            self.end = Some(true);
            return Ok(());
        }
        for plant in self.plants.iter_mut().flatten().flatten() {
            plant.update(!self.showing_zombies, elapsed)?;
        }
        self.update_zombies(elapsed)?;
        if let Some(false) = self.end {
            return Ok(());
        }
        self.update_projectiles(elapsed)?;
        self.update_suns(elapsed)?;
        self.spawn_projectiles();
        self.update_zombie_wave(elapsed);
        Ok(())
    }

    fn update_zombies(&mut self, elapsed: Duration) -> Result<(), String> {
        for (y, zombies) in self.zombies.iter_mut().enumerate() {
            for zombie in zombies.iter_mut() {
                let prev_pos = zombie.pos();
                zombie.update(!self.showing_zombies, elapsed)?;
                if zombie.pos()
                    >= 1. - self.config.left as f32 / 1280. + zombie.width() as f32 / 1280.
                {
                    self.end = Some(false);
                } else {
                    do_damage_to_plant(
                        zombie.as_mut(),
                        self.plants[y].as_mut(),
                        &self.config,
                        self.config.rows[y],
                        prev_pos,
                        elapsed,
                    );
                }
            }
        }
        Ok(())
    }

    fn update_projectiles(&mut self, elapsed: Duration) -> Result<(), String> {
        for (y, projs) in self.projectiles.iter_mut().enumerate() {
            let mut indx = Vec::new();
            for (i, proj) in projs.iter_mut().enumerate() {
                proj.update(!self.showing_zombies, elapsed)?;

                let proj = proj.as_ref();

                if proj.to_remove() {
                    indx.insert(0, i);
                    continue;
                }

                let mut zombie_to_remove = do_damage_to_zombies(
                    self.zombies[y].as_mut(),
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
                projs.remove(i);
            }
        }
        Ok(())
    }

    fn update_suns(&mut self, elapsed: Duration) -> Result<(), String> {
        for sun in self.suns.iter_mut() {
            sun.update(!self.showing_zombies, elapsed)?;
        }
        if self.next_sun > elapsed {
            self.next_sun -= elapsed
        } else {
            self.next_sun = Duration::new(5, 0) - elapsed + self.next_sun;
            self.suns.push(Sun::new(
                rand::thread_rng().gen_range(0..1220),
                0.,
                rand::thread_rng().gen_range(200.0..420.),
            ));
        }
        Ok(())
    }

    fn update_zombie_wave(&mut self, mut elapsed: Duration) {
        if !self.config.waits.is_empty() {
            if let Some(&f) = self.config.waits.first() {
                if elapsed >= f {
                    elapsed -= f;
                    self.config.waits.remove(0);
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
            if let Some(f) = self.config.waits.first_mut() {
                *f -= elapsed;
            }
        }
    }
}
