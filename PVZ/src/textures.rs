use anyhow::{anyhow, Result};
use sdl2::{
    image::LoadTexture,
    render::{BlendMode, Canvas, Texture, TextureCreator},
    ttf::{self, Font},
    video::{Window, WindowContext},
};
use std::fs;

pub struct Textures {
    maps: Vec<Texture<'static>>,
    sun: Texture<'static>,
    plant_sunflower: Texture<'static>,
    plant_simple: Texture<'static>,
    plant_fire_simple: Texture<'static>,
    plant_ice_simple: Texture<'static>,
    plant_triple: Texture<'static>,
    plant_nenuphar: Texture<'static>,
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

fn freezed(
    canvas: &mut Canvas<Window>,
    texture_creator: &'static TextureCreator<WindowContext>,
    texture: &Texture<'static>,
) -> Result<Texture<'static>> {
    let query = texture.query();
    let mut new_texture = texture_creator
        .create_texture_target(None, query.width, query.height)
        .map_err(|e| anyhow!(e))?;

    new_texture.set_blend_mode(BlendMode::Blend);

    let mut success = Ok(());
    canvas
        .with_texture_canvas(&mut new_texture, |texture_canvas| {
            success = texture_canvas
                .copy(texture, None, None)
                .map_err(|e| anyhow!(e));
        })
        .map_err(|e| anyhow!(e))?;
    success?;

    new_texture.set_color_mod(100, 100, 255);

    Ok(new_texture)
}

pub fn load_textures(
    canvas: &mut Canvas<Window>,
    texture_creator: &'static TextureCreator<WindowContext>,
) -> Result<Textures> {
    let maps_count = fs::read_dir("assets/maps").map_err(|e| anyhow!(e))?.count() / 2;
    let maps: Vec<Texture<'_>> = (0..maps_count)
        .flat_map(|i| texture_creator.load_texture(format!("assets/maps/{i}.png")))
        .collect();
    if maps.len() != maps_count {
        return Err(anyhow!("Not all maps could be loaded"));
    }
    let font_context = Box::leak(Box::new(ttf::init().map_err(|e| anyhow!(e))?));

    let zombie_simple = texture_creator
        .load_texture("assets/Zombies/Simple.png")
        .map_err(|e| anyhow!(e))?;
    let zombie_simple_1 = texture_creator
        .load_texture("assets/Zombies/Simple_1.png")
        .map_err(|e| anyhow!(e))?;
    let zombie_cone = texture_creator
        .load_texture("assets/Zombies/Cone.png")
        .map_err(|e| anyhow!(e))?;
    let zombie_cone_1 = texture_creator
        .load_texture("assets/Zombies/Cone_1.png")
        .map_err(|e| anyhow!(e))?;
    let zombie_freeze_simple = freezed(canvas, texture_creator, &zombie_simple)?;
    let zombie_freeze_simple_1 = freezed(canvas, texture_creator, &zombie_simple_1)?;
    let zombie_freeze_cone = freezed(canvas, texture_creator, &zombie_cone)?;
    let zombie_freeze_cone_1 = freezed(canvas, texture_creator, &zombie_cone_1)?;

    Ok(Textures {
        maps,
        sun: texture_creator
            .load_texture("assets/Sun.png")
            .map_err(|e| anyhow!(e))?,
        plant_sunflower: texture_creator
            .load_texture("assets/Plants/Sunflower.png")
            .map_err(|e| anyhow!(e))?,
        pea: texture_creator
            .load_texture("assets/Plants/Pea.png")
            .map_err(|e| anyhow!(e))?,
        fire_pea: texture_creator
            .load_texture("assets/Plants/Fire Pea.png")
            .map_err(|e| anyhow!(e))?,
        ice_pea: texture_creator
            .load_texture("assets/Plants/Ice Pea.png")
            .map_err(|e| anyhow!(e))?,
        plant_simple: texture_creator
            .load_texture("assets/Plants/Simple.png")
            .map_err(|e| anyhow!(e))?,
        plant_fire_simple: texture_creator
            .load_texture("assets/Plants/Fire Simple.png")
            .map_err(|e| anyhow!(e))?,
        plant_ice_simple: texture_creator
            .load_texture("assets/Plants/Ice Simple.png")
            .map_err(|e| anyhow!(e))?,
        plant_triple: texture_creator
            .load_texture("assets/Plants/Triple.png")
            .map_err(|e| anyhow!(e))?,
        plant_nenuphar: texture_creator
            .load_texture("assets/Plants/Nenuphar.png")
            .map_err(|e| anyhow!(e))?,
        zombie_simple,
        zombie_simple_1,
        zombie_cone,
        zombie_cone_1,
        zombie_freeze_simple,
        zombie_freeze_simple_1,
        zombie_freeze_cone,
        zombie_freeze_cone_1,
        //font_context,
        font: font_context
            .load_font("assets/OpenSans-Regular.ttf", 128)
            .map_err(|e| anyhow!(e))?,
    })
}

impl Textures {
    pub fn map(&'static self, id: usize) -> &'static Texture<'static> {
        &self.maps[id]
    }
    pub const fn sun(&'static self) -> &'static Texture<'static> {
        &self.sun
    }
    pub const fn plant_sunflower(&'static self) -> &'static Texture<'static> {
        &self.plant_sunflower
    }
    pub const fn plant_simple(&'static self) -> &'static Texture<'static> {
        &self.plant_simple
    }
    pub const fn plant_fire_simple(&'static self) -> &'static Texture<'static> {
        &self.plant_fire_simple
    }
    pub const fn plant_ice_simple(&'static self) -> &'static Texture<'static> {
        &self.plant_ice_simple
    }
    pub const fn plant_triple(&'static self) -> &'static Texture<'static> {
        &self.plant_triple
    }
    pub const fn plant_nenuphar(&'static self) -> &'static Texture<'static> {
        &self.plant_nenuphar
    }
    pub const fn zombie_simple(&'static self) -> &'static Texture<'static> {
        &self.zombie_simple
    }
    pub const fn zombie_simple_1(&'static self) -> &'static Texture<'static> {
        &self.zombie_simple_1
    }
    pub const fn zombie_cone(&'static self) -> &'static Texture<'static> {
        &self.zombie_cone
    }
    pub const fn zombie_cone_1(&'static self) -> &'static Texture<'static> {
        &self.zombie_cone_1
    }
    pub const fn zombie_freeze_simple(&'static self) -> &'static Texture<'static> {
        &self.zombie_freeze_simple
    }
    pub const fn zombie_freeze_simple_1(&'static self) -> &'static Texture<'static> {
        &self.zombie_freeze_simple_1
    }
    pub const fn zombie_freeze_cone(&'static self) -> &'static Texture<'static> {
        &self.zombie_freeze_cone
    }
    pub const fn zombie_freeze_cone_1(&'static self) -> &'static Texture<'static> {
        &self.zombie_freeze_cone_1
    }
    pub const fn pea(&'static self) -> &'static Texture<'static> {
        &self.pea
    }
    pub const fn fire_pea(&'static self) -> &'static Texture<'static> {
        &self.fire_pea
    }
    pub const fn ice_pea(&'static self) -> &'static Texture<'static> {
        &self.ice_pea
    }

    pub const fn font(&'static self) -> &'static Font<'static, 'static> {
        &self.font
    }
}
