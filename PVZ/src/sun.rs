use std::time::Duration;

use sdl2::{rect::FRect, render::Texture};

use crate::textures;

pub struct Sun {
    pub x: f32,
    pub y: f32,
    pub dist: f32,
}

impl Sun {
    pub const fn new(x: f32, y: f32, dist: f32) -> Self {
        Self { x, y, dist }
    }

    pub fn texture(&self) -> Result<&'static Texture<'static>, String> {
        Ok(&textures::textures()?.sun)
    }

    pub fn rect(&self) -> FRect {
        FRect::new(self.x, self.y, 60. / 1280., 90. / 720.)
    }

    pub fn update(&mut self, elapsed: Duration) -> Result<(), String> {
        self.y = (self.y + elapsed.as_secs_f32() * 34.642944 / 720.).min(self.dist);
        Ok(())
    }
}
