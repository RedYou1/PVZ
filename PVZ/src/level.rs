use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, rect::Rect, render::Canvas, video::Window};

use crate::{plant::Plant, textures, zombie::Zombie};

pub trait ILevel {
    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String>;
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn update(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}

pub trait LevelConfig {
    fn probs(&self) -> &Vec<(Box<dyn Zombie>, f32)>;
}

/// Slots:
///     left: 308 + x * 97
///     top: 102 + y * 117
///     width: 80
///     heigth: 106
pub struct Level<const ROWS: usize> {
    pub showing_zombies: bool,
    pub plants: [[Option<Box<dyn Plant>>; 9]; ROWS],
    pub zombies: [Vec<Box<dyn Zombie>>; ROWS],
    pub config: Box<dyn LevelConfig>,
}

impl<const ROWS: usize> ILevel for Level<ROWS> {
    fn event(&mut self, _: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        if let Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } = event
        {
            self.showing_zombies = !self.showing_zombies
        }
        Ok(())
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
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
            for (y, (z, _)) in self.config.probs().iter().enumerate() {
                canvas.copy(
                    z.texture(),
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
        Ok(())
    }

    fn update(&mut self, _: &mut Canvas<Window>) -> Result<(), String> {
        for p in self.plants.iter_mut().flatten().flatten() {
            p.update(!self.showing_zombies)?;
        }

        for z in self.zombies.iter_mut().flatten() {
            z.update(!self.showing_zombies)?;
        }

        if !self.showing_zombies {
            let a: f32 = rand::thread_rng().gen();
            for (z, probs) in self.config.probs() {
                if a <= *probs {
                    self.zombies[rand::thread_rng().gen_range(0..5)].push(z.as_ref().clone());
                }
            }
        }

        Ok(())
    }
}

pub struct DefaultConfig {
    pub probs: Vec<(Box<dyn Zombie>, f32)>,
}
impl LevelConfig for DefaultConfig {
    fn probs(&self) -> &Vec<(Box<dyn Zombie>, f32)> {
        &self.probs
    }
}
