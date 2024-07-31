use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::{cell::Cell, fs};

thread_local! {
    pub static TEXTURES: Cell<Option<&'static Textures>> = const { Cell::new(None) };
}

struct Textures {
    maps: Vec<Texture<'static>>,
    plant_simple: Texture<'static>,
    plant_triple: Texture<'static>,
    zombie_simple: Texture<'static>,
    zombie_simple_1: Texture<'static>,
    pea: Texture<'static>,
}

pub fn plant_simple() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").plant_simple
}

pub fn plant_triple() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").plant_triple
}

pub fn zombie_simple() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").zombie_simple
}

pub fn zombie_simple_1() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").zombie_simple_1
}

pub fn pea() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").pea
}

pub fn maps(i: usize) -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").maps[i]
}

pub fn load_textures(
    texture_creator: &'static TextureCreator<WindowContext>,
) -> Result<(), String> {
    let maps_count = fs::read_dir("assets/maps")
        .map_err(|e| e.to_string())?
        .count() / 2;
    let maps: Vec<Texture<'static>> = (0..maps_count)
        .flat_map(|i| texture_creator.load_texture(format!("assets/maps/{i}.png")))
        .collect();
    if maps.len() != maps_count {
        return Err("Not all maps could be loaded".to_owned());
    }
    TEXTURES.set(Some(Box::leak(Box::new(Textures {
        maps,
        pea: texture_creator.load_texture("assets/Plants/Pea.png")?,
        plant_simple: texture_creator.load_texture("assets/Plants/Simple.png")?,
        plant_triple: texture_creator.load_texture("assets/Plants/Triple.png")?,
        zombie_simple: texture_creator.load_texture("assets/Zombies/Simple.png")?,
        zombie_simple_1: texture_creator.load_texture("assets/Zombies/Simple_1.png")?,
    }))));
    Ok(())
}
