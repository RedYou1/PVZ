use std::time::Duration;

use sdl2::render::Texture;

use crate::{entity::Entity, textures};

pub struct Sun {
    pub x: i32,
    pub y: f32,
    pub dist: f32,
}

impl Sun {
    pub const fn new(x: i32, y: f32, dist: f32) -> Self {
        Self { x, y, dist }
    }
}

impl Entity for Sun {
    fn texture(&self) -> Result<&'static Texture<'static>, String> {
        Ok(&textures::textures()?.sun)
    }

    fn width(&self) -> u16 {
        60
    }

    fn height(&self) -> u16 {
        90
    }

    fn update(&mut self, playing: bool, elapsed: Duration) -> Result<(), String> {
        if !playing {
            return Ok(());
        }
        self.y = (self.y + elapsed.as_secs_f32() * 34.642944).min(self.dist);
        Ok(())
    }
}
