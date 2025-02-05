use red_sdl::{missing::ui_string::UIString, refs::MutRef};

use crate::State;

#[derive(Default)]
pub struct Texts {
    pub lang: UIString,
    pub quit: UIString,
    pub full_screen: UIString,
    pub _return: UIString,
    pub menu: UIString,
    pub start: UIString,
    pub win: UIString,
    pub lost: UIString,

    pub update_available: UIString,
    pub up_to_date: UIString,
    pub loading: UIString,

    pub save: UIString,
}

#[derive(Clone, Copy)]
pub enum Lang {
    EN,
    FR,
}

pub fn load_texts(mut state: MutRef<State>) {
    let font = state.as_mut().textures().font();
    state.en = Texts {
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
    state.fr = Texts {
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
