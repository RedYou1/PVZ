use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{BlendMode, Canvas, Texture, TextureCreator},
    ttf::{self, Font},
    video::{Window, WindowContext},
};
use std::{cell::Cell, fs};

thread_local! {
    pub static TEXTURES: Cell<Option<&'static Textures>> = const { Cell::new(None) };
}

struct Textures {
    maps: Vec<Texture<'static>>,
    plant_simple: Texture<'static>,
    plant_fire_simple: Texture<'static>,
    plant_ice_simple: Texture<'static>,
    plant_triple: Texture<'static>,
    zombie_simple: Texture<'static>,
    zombie_simple_1: Texture<'static>,
    zombie_cone: Texture<'static>,
    zombie_cone_1: Texture<'static>,
    zombie_freeze_simple: Texture<'static>,
    zombie_freeze_simple_1: Texture<'static>,
    zombie_freeze_cone: Texture<'static>,
    zombie_freeze_cone_1: Texture<'static>,
    pea: Texture<'static>,
    fire_pea: Texture<'static>,
    ice_pea: Texture<'static>,

    //font_context: &'static Sdl2TtfContext,
    font: Font<'static, 'static>,
}

pub fn plant_simple() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").plant_simple
}

pub fn plant_fire_simple() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").plant_fire_simple
}

pub fn plant_ice_simple() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").plant_ice_simple
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

pub fn zombie_cone() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").zombie_cone
}

pub fn zombie_cone_1() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").zombie_cone_1
}

pub fn zombie_freeze_simple() -> &'static Texture<'static> {
    &TEXTURES
        .get()
        .expect("Not main thread")
        .zombie_freeze_simple
}

pub fn zombie_freeze_simple_1() -> &'static Texture<'static> {
    &TEXTURES
        .get()
        .expect("Not main thread")
        .zombie_freeze_simple_1
}

pub fn zombie_freeze_cone() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").zombie_freeze_cone
}

pub fn zombie_freeze_cone_1() -> &'static Texture<'static> {
    &TEXTURES
        .get()
        .expect("Not main thread")
        .zombie_freeze_cone_1
}

pub fn pea() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").pea
}

pub fn fire_pea() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").fire_pea
}

pub fn ice_pea() -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").ice_pea
}

pub fn maps(i: usize) -> &'static Texture<'static> {
    &TEXTURES.get().expect("Not main thread").maps[i]
}

pub fn font() -> &'static Font<'static, 'static> {
    &TEXTURES.get().expect("Not main thread").font
}

pub fn draw_string(
    canvas: &mut Canvas<Window>,
    to: Rect,
    text: &str,
    color: Color,
) -> Result<(), String> {
    canvas.copy(
        &canvas
            .texture_creator()
            .create_texture_from_surface(
                font()
                    .render(text)
                    .blended(color)
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?,
        None,
        to,
    )
}

fn freezed(
    canvas: &mut Canvas<Window>,
    texture_creator: &'static TextureCreator<WindowContext>,
    texture: &Texture<'static>,
) -> Result<Texture<'static>, String> {
    let query = texture.query();
    let mut new_texture = texture_creator
        .create_texture_target(None, query.width, query.height)
        .map_err(|e| e.to_string())?;

    new_texture.set_blend_mode(BlendMode::Blend);

    canvas
        .with_texture_canvas(&mut new_texture, |texture_canvas| {
            texture_canvas
                .copy(texture, None, None)
                .expect("error while duplicating a texture");
        })
        .map_err(|e| e.to_string())?;

    new_texture.set_color_mod(100, 100, 255);

    Ok(new_texture)
}

pub fn load_textures(
    canvas: &mut Canvas<Window>,
    texture_creator: &'static TextureCreator<WindowContext>,
) -> Result<(), String> {
    let maps_count = fs::read_dir("assets/maps")
        .map_err(|e| e.to_string())?
        .count()
        / 2;
    let maps: Vec<Texture<'static>> = (0..maps_count)
        .flat_map(|i| texture_creator.load_texture(format!("assets/maps/{i}.png")))
        .collect();
    if maps.len() != maps_count {
        return Err("Not all maps could be loaded".to_owned());
    }
    let font_context = Box::leak(Box::new(ttf::init().map_err(|e| e.to_string())?));

    let zombie_simple = texture_creator.load_texture("assets/Zombies/Simple.png")?;
    let zombie_simple_1 = texture_creator.load_texture("assets/Zombies/Simple_1.png")?;
    let zombie_cone = texture_creator.load_texture("assets/Zombies/Cone.png")?;
    let zombie_cone_1 = texture_creator.load_texture("assets/Zombies/Cone_1.png")?;
    let zombie_freeze_simple = freezed(canvas, texture_creator, &zombie_simple)?;
    let zombie_freeze_simple_1 = freezed(canvas, texture_creator, &zombie_simple_1)?;
    let zombie_freeze_cone = freezed(canvas, texture_creator, &zombie_cone)?;
    let zombie_freeze_cone_1 = freezed(canvas, texture_creator, &zombie_cone_1)?;

    TEXTURES.set(Some(Box::leak(Box::new(Textures {
        maps,
        pea: texture_creator.load_texture("assets/Plants/Pea.png")?,
        fire_pea: texture_creator.load_texture("assets/Plants/Fire Pea.png")?,
        ice_pea: texture_creator.load_texture("assets/Plants/Ice Pea.png")?,
        plant_simple: texture_creator.load_texture("assets/Plants/Simple.png")?,
        plant_fire_simple: texture_creator.load_texture("assets/Plants/Fire Simple.png")?,
        plant_ice_simple: texture_creator.load_texture("assets/Plants/Ice Simple.png")?,
        plant_triple: texture_creator.load_texture("assets/Plants/Triple.png")?,
        zombie_simple,
        zombie_simple_1,
        zombie_cone,
        zombie_cone_1,
        zombie_freeze_simple,
        zombie_freeze_simple_1,
        zombie_freeze_cone,
        zombie_freeze_cone_1,
        //font_context,
        font: font_context.load_font("assets/OpenSans-Regular.ttf", 128)?,
    }))));
    Ok(())
}
