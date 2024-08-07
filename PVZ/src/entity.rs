use sdl2::render::Texture;
use std::time::Duration;

pub trait Entity {
    fn texture(&self) -> Result<&'static Texture<'static>, String>;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn update(&mut self, playing: bool, elapsed: Duration) -> Result<(), String>;
}
