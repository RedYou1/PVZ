use sdl2::{
    image::LoadTexture,
    rect::FRect,
    render::{BlendMode, Canvas, Texture, TextureCreator},
    ttf::{self, Font},
    video::{Window, WindowContext},
};
use std::{cell::Cell, fs};

thread_local! {
    pub static TEXTURES: Cell<Option<&'static Textures>> = const { Cell::new(None) };
}

pub struct Textures {
    pub maps: Vec<Texture<'static>>,
    pub sun: Texture<'static>,
    pub plant_sunflower: Texture<'static>,
    pub plant_simple: Texture<'static>,
    pub plant_fire_simple: Texture<'static>,
    pub plant_ice_simple: Texture<'static>,
    pub plant_triple: Texture<'static>,
    pub plant_nenuphar: Texture<'static>,
    pub zombie_simple: Texture<'static>,
    pub zombie_simple_1: Texture<'static>,
    pub zombie_cone: Texture<'static>,
    pub zombie_cone_1: Texture<'static>,
    pub zombie_freeze_simple: Texture<'static>,
    pub zombie_freeze_simple_1: Texture<'static>,
    pub zombie_freeze_cone: Texture<'static>,
    pub zombie_freeze_cone_1: Texture<'static>,
    pub pea: Texture<'static>,
    pub fire_pea: Texture<'static>,
    pub ice_pea: Texture<'static>,

    //pub font_context: &'static Sdl2TtfContext,
    pub font: Font<'static, 'static>,
}

pub fn textures() -> Result<&'static Textures, String> {
    TEXTURES
        .get()
        .ok_or("Didn't loaded the textures".to_owned())
}

pub fn draw_string(canvas: &mut Canvas<Window>, to: FRect, text: &str) -> Result<(), String> {
    sdl::draw_string(canvas, &textures()?.font, to, text)
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

    let mut success = Ok(());
    canvas
        .with_texture_canvas(&mut new_texture, |texture_canvas| {
            success = texture_canvas.copy(texture, None, None);
        })
        .map_err(|e| e.to_string())?;
    success?;

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
        sun: texture_creator.load_texture("assets/Sun.png")?,
        plant_sunflower: texture_creator.load_texture("assets/Plants/Sunflower.png")?,
        pea: texture_creator.load_texture("assets/Plants/Pea.png")?,
        fire_pea: texture_creator.load_texture("assets/Plants/Fire Pea.png")?,
        ice_pea: texture_creator.load_texture("assets/Plants/Ice Pea.png")?,
        plant_simple: texture_creator.load_texture("assets/Plants/Simple.png")?,
        plant_fire_simple: texture_creator.load_texture("assets/Plants/Fire Simple.png")?,
        plant_ice_simple: texture_creator.load_texture("assets/Plants/Ice Simple.png")?,
        plant_triple: texture_creator.load_texture("assets/Plants/Triple.png")?,
        plant_nenuphar: texture_creator.load_texture("assets/Plants/Nenuphar.png")?,
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
