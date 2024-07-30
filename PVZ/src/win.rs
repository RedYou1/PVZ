use std::time::Duration;

use sdl::game_window::GameWindow;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};

use crate::{level::Level, textures::load_textures};

pub struct Win {
    running: bool,

    level: Level,
}

impl Win {
    pub fn new(canvas: &mut Canvas<Window>) -> Result<Self, String> {
        load_textures(Box::leak(Box::new(canvas.texture_creator())))?;

        Ok(Self {
            running: true,
            level: Level::load_config("levels/level0.data").map_err(|e| e.to_string())?,
        })
    }
}

impl GameWindow for Win {
    fn running(&mut self) -> bool {
        self.running
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        self.level.update(canvas, elapsed)
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.running = false,
            _ => {}
        }
        self.level.event(canvas, event.clone())
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        self.level.draw(canvas)
    }
}
