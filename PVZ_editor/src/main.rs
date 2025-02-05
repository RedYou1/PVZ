mod level_config;
mod map_config;
mod pin;
mod rows_editor;
mod win;

use std::fs;

use anyhow::{anyhow, Result};
use pvz::{
    save::SaveFile,
    texts::{Lang, Texts},
    textures::{load_textures, Textures},
};
use red_sdl::{missing::ui_string::UIString, refs::MutRef, run_event, ui_element::ui_rect::UIRect};
use sdl2::{render::Canvas, video::Window};
use win::Page;

use crate::win::Win;

pub struct State {
    textures: Textures,
    save: SaveFile,
    page_a: Page,
    page_b: Page,
    page: bool,
    pub maps_count: u8,
    pub levels_count: u8,

    en: Texts,
    fr: Texts,
}

pub fn main() -> Result<()> {
    run_event(
        "Plant Vs Zombie Editor",
        1280,
        720,
        |window| window.fullscreen_desktop().resizable(),
        |canvas| {
            let textures = load_textures(canvas, Box::leak(Box::new(canvas.texture_creator())))?;
            let maps_count = fs::read_dir("assets/maps")
                .map_err(|e| anyhow!(e))?
                .filter(|f| {
                    if let Ok(d) = f {
                        d.file_name()
                            .to_str()
                            .is_some_and(|s| s.to_lowercase().ends_with("data"))
                    } else {
                        false
                    }
                })
                .count();
            if maps_count == 0 || fs::read_dir("assets/maps").map_err(|e| anyhow!(e))?.count() > 99
            {
                return Err(anyhow!("Too much or no levels"));
            }
            let levels_count = fs::read_dir("levels").map_err(|e| anyhow!(e))?.count();
            if levels_count == 0 || fs::read_dir("levels").map_err(|e| anyhow!(e))?.count() > 99 {
                return Err(anyhow!("Too much or no levels"));
            }
            Ok(State::new(
                maps_count as u8,
                levels_count as u8,
                SaveFile::load()?,
                textures,
            ))
        },
        Win::new,
    )
}

impl State {
    pub fn new(maps_count: u8, levels_count: u8, save: SaveFile, textures: Textures) -> Self {
        Self {
            maps_count,
            levels_count,
            page_a: Page::Uninit(()),
            page_b: Page::Uninit(()),
            page: false,
            save,
            textures,
            en: Texts::default(),
            fr: Texts::default(),
        }
    }

    pub const fn get_page(&self) -> &Page {
        if self.page {
            &self.page_a
        } else {
            &self.page_b
        }
    }

    pub fn get_page_mut(&mut self) -> &mut Page {
        if self.page {
            &mut self.page_a
        } else {
            &mut self.page_b
        }
    }

    pub fn set_page(&mut self, page: Page) {
        self.page = !self.page;
        *if self.page {
            &mut self.page_a
        } else {
            &mut self.page_b
        } = page;
    }

    pub fn _return<From>(
        _: MutRef<UIRect<From, State>>,
        _: MutRef<From>,
        mut state: MutRef<State>,
        _: &Canvas<Window>,
    ) -> Result<()> {
        state.as_mut().set_page(Page::main_menu(state.into()));
        Ok(())
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

pub fn load_texts(mut state: MutRef<State>) {
    let font = state.as_ref().textures().font();
    state.as_mut().en = Texts {
        lang: UIString::new_const(font, "English"),
        quit: UIString::new_const(font, "Quit"),
        full_screen: UIString::new_const(font, "Full screen"),
        _return: UIString::new_const(font, "Return"),
        menu: UIString::new_const(font, "Menu"),
        start: UIString::new_const(font, "Start"),
        win: UIString::new_const(font, "Win"),
        lost: UIString::new_const(font, "Lost"),
        update_available: UIString::new_const(font, "An update is available."),
        up_to_date: UIString::new_const(font, "You are up to date."),
        loading: UIString::new_const(font, "Loading..."),
        save: UIString::new_const(font, "Save"),
    };
    state.as_mut().fr = Texts {
        lang: UIString::new_const(font, "Français"),
        quit: UIString::new_const(font, "Quitter"),
        full_screen: UIString::new_const(font, "Plein écran"),
        _return: UIString::new_const(font, "Retour"),
        menu: UIString::new_const(font, "Menu"),
        start: UIString::new_const(font, "Commencer"),
        win: UIString::new_const(font, "Victoire"),
        lost: UIString::new_const(font, "Défaite"),
        update_available: UIString::new_const(font, "Une mise à jour est disponible."),
        up_to_date: UIString::new_const(font, "Vous êtes à jour."),
        loading: UIString::new_const(font, "Chargement..."),
        save: UIString::new_const(font, "Sauvegarder"),
    };
}
