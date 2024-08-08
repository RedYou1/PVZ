use config::LevelConfig;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::{FPoint, Rect},
    render::Canvas,
    video::Window,
};
use std::time::Duration;

mod collision;
pub mod config;
mod draws;
mod updates;

use crate::{
    into_rect,
    plants::Plant,
    projectile::Projectile,
    save::SaveFile,
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
                let x = scale_x(x);
                let y = scale_y(y);
                for i in self
                    .suns
                    .iter()
                    .enumerate()
                    .filter_map(|(i, sun)| {
                        if sun.rect().contains_point(FPoint::new(x, y)) {
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

    pub fn draw(&self, canvas: &mut Canvas<Window>, save: &SaveFile) -> Result<(), String> {
        canvas.copy(
            &textures::textures()?.maps[self.config.map as usize],
            Some(Rect::new(
                if self.showing_zombies { 238 } else { 0 },
                0,
                762,
                429,
            )),
            Rect::new(0, 0, 1280, 720),
        )?;

        if self.showing_zombies {
            let mut t: Vec<&(u8, f32, f32)> = self.config.zombies.iter().flatten().collect();
            t.sort_by(|(_, _, y1), (_, _, y2)| y1.total_cmp(y2));
            for &(z, x, y) in t {
                let mut z = zombie_from_id(z);
                z.set_x(x);
                canvas.copy(z.texture()?, None, into_rect(z.rect(y)))?;
            }

            canvas.set_draw_color(Color::BLACK);
            canvas.fill_rect(Rect::new(1120, 10, 150, 40))?;
            draw_string(canvas, Rect::new(1120, 10, 150, 40), save.texts().menu)?;
            canvas.fill_rect(Rect::new(1070, 670, 200, 40))?;
            draw_string(canvas, Rect::new(1070, 670, 200, 40), save.texts().start)?;
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
                if end {
                    save.texts().win
                } else {
                    save.texts().lost
                },
            )?;
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(1120, 10, 150, 40))?;
        draw_string(canvas, Rect::new(1120, 10, 150, 40), save.texts().menu)?;
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
}
