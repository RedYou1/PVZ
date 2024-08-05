use std::time::Duration;

use sdl2::render::Texture;

use crate::{entity::Entity, textures};

pub struct Sun {
    pub x: i32,
    pub y: i32,
    pub progress: f32,
}

impl Sun {
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            progress: 10.,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.progress < 0.
    }
}

impl Entity for Sun {
    fn texture(&self) -> &'static Texture<'static> {
        &textures::textures().sun
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
        if self.progress > 5. {
            self.y += (elapsed.as_secs_f32().clamp(0., self.progress - 5.) * 4.) as i32;
        }
        self.progress -= elapsed.as_secs_f32();
        Ok(())
    }
}
