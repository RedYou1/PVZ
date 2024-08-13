use sdl::missing::ui_string::UIString;
use sdl2::ttf::Font;

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
}

static mut EN: Option<Texts> = None;
static mut FR: Option<Texts> = None;

#[derive(Clone, Copy)]
pub enum Lang {
    EN,
    FR,
}

pub fn texts(lang: Lang) -> Result<&'static Texts, String> {
    unsafe {
        Ok(match lang {
            Lang::EN => EN.as_ref().ok_or("Didn't loaded the texts".to_owned())?,
            Lang::FR => FR.as_ref().ok_or("Didn't loaded the texts".to_owned())?,
        })
    }
}

pub fn load_texts(font: &'static Font<'static, 'static>) -> Result<(), String> {
    unsafe {
        EN = Some(Texts {
            lang: UIString::new(font, "English".to_owned())?.expect("Constant text"),
            quit: UIString::new(font, "Quit".to_owned())?.expect("Constant text"),
            full_screen: UIString::new(font, "Full screen".to_owned())?.expect("Constant text"),
            _return: UIString::new(font, "Return".to_owned())?.expect("Constant text"),
            menu: UIString::new(font, "Menu".to_owned())?.expect("Constant text"),
            start: UIString::new(font, "Start".to_owned())?.expect("Constant text"),
            win: UIString::new(font, "Win".to_owned())?.expect("Constant text"),
            lost: UIString::new(font, "Lost".to_owned())?.expect("Constant text"),
            update_available: UIString::new(font, "An update is available.".to_owned())?
                .expect("Constant text"),
            up_to_date: UIString::new(font, "You are up to date.".to_owned())?
                .expect("Constant text"),
            loading: UIString::new(font, "Loading...".to_owned())?.expect("Constant text"),
        });
        FR = Some(Texts {
            lang: UIString::new(font, "Français".to_owned())?.expect("Constant text"),
            quit: UIString::new(font, "Quitter".to_owned())?.expect("Constant text"),
            full_screen: UIString::new(font, "Plein écran".to_owned())?.expect("Constant text"),
            _return: UIString::new(font, "Retour".to_owned())?.expect("Constant text"),
            menu: UIString::new(font, "Menu".to_owned())?.expect("Constant text"),
            start: UIString::new(font, "Commencer".to_owned())?.expect("Constant text"),
            win: UIString::new(font, "Victoire".to_owned())?.expect("Constant text"),
            lost: UIString::new(font, "Défaite".to_owned())?.expect("Constant text"),
            update_available: UIString::new(font, "Une mise à jour est disponible.".to_owned())?
                .expect("Constant text"),
            up_to_date: UIString::new(font, "Vous êtes à jour.".to_owned())?
                .expect("Constant text"),
            loading: UIString::new(font, "Chargement...".to_owned())?.expect("Constant text"),
        });
    }
    Ok(())
}
