use sdl2::render::Texture;

pub trait Entity {
    fn texture(&self) -> &'static Texture<'static>;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn update(&mut self, playing: bool) -> Result<(), String>;
}
