use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::cell::Cell;

thread_local! {
    pub static TEXTURES: Cell<Option<&'static Textures>> = const { Cell::new(None) };
}

struct Textures {
    p1: Texture<'static>,
    z1: Texture<'static>,
    map: Texture<'static>,
}

pub fn p1() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").p1
}
pub fn z1() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").z1
}

pub fn map() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").map
}

pub fn load_textures(
    texture_creator: &'static TextureCreator<WindowContext>,
) -> Result<(), String> {
    TEXTURES.set(Some(Box::leak(Box::new(Textures {
        p1: texture_creator.load_texture("assets/P1.png")?,
        z1: texture_creator.load_texture("assets/Z1.png")?,
        map: texture_creator.load_texture("assets/map.png")?,
    }))));
    Ok(())
}
