use std::time::Duration;

use anyhow::Result;
use red_sdl::refs::Ref;
use sdl2::{rect::FRect, render::Texture};

use crate::State;

pub struct Sun {
    pub x: f32,
    pub y: f32,
    pub dist: f32,
}

impl Sun {
    pub const fn new(x: f32, y: f32, dist: f32) -> Self {
        Self { x, y, dist }
    }

    pub const fn texture(state: Ref<State>) -> &'static Texture<'static> {
        state.as_ref().textures().sun()
    }

    pub fn rect(&self) -> FRect {
        FRect::new(self.x, self.y, 60. / 1280., 90. / 720.)
    }

    pub fn update(&mut self, elapsed: Duration) -> Result<()> {
        self.y = (self.y + elapsed.as_secs_f32() * 34.642944 / 720.).min(self.dist);
        Ok(())
    }
}
