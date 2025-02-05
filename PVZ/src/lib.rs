#![feature(ptr_as_ref_unchecked)]

use anyhow::Result;
use red_sdl::{functions::StateEnum, ui_element::ui_rect::UIRect};
use save::SaveFile;
use sdl2::pixels::Color;
use texts::{Lang, Texts};
use textures::Textures;

pub mod level;
pub mod map_plant;
pub mod plants;
pub mod projectile;
pub mod save;
pub mod shop_plant;
pub mod sun;
pub mod texts;
pub mod textures;
pub mod win;
pub mod zombie;

pub fn default_button<Parent, State>() -> UIRect<Parent, State> {
    UIRect::new(
        Box::new(|_, _, _| StateEnum::Enable),
        Box::new(|_, _, _| Color::BLACK),
    )
}

pub struct State {
    levels_count: u8,
    save: SaveFile,
    textures: Textures,
    update_available: Option<Result<bool>>,
    en: Texts,
    fr: Texts,
}

impl State {
    pub fn new(levels_count: u8, save: SaveFile, textures: Textures) -> Self {
        Self {
            levels_count,
            save,
            textures,
            update_available: None,
            en: Texts::default(),
            fr: Texts::default(),
        }
    }

    pub const fn textures(&self) -> &Textures {
        &self.textures
    }

    pub const fn texts(&self) -> &Texts {
        match self.save.langage {
            Lang::EN => &self.en,
            Lang::FR => &self.fr,
        }
    }
}
