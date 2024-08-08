pub struct Texts {
    pub lang: &'static str,
    pub quit: &'static str,
    pub full_screen: &'static str,
    pub _return: &'static str,
    pub menu: &'static str,
    pub start: &'static str,
    pub win: &'static str,
    pub lost: &'static str,
}

const EN: Texts = Texts {
    lang: "English",
    quit: "Quit",
    full_screen: "Full screen",
    _return: "Return",
    menu: "Menu",
    start: "Start",
    win: "Win",
    lost: "Lost",
};

const FR: Texts = Texts {
    lang: "Français",
    quit: "Quitter",
    full_screen: "Plein écran",
    _return: "Retour",
    menu: "Menu",
    start: "Commencer",
    win: "Victoire",
    lost: "Défaite",
};

pub enum Lang {
    EN,
    FR,
}

static mut LANG: Lang = Lang::EN;

pub fn next_lang() {
    unsafe {
        LANG = match LANG {
            Lang::EN => Lang::FR,
            Lang::FR => Lang::EN,
        }
    }
}

pub fn texts() -> &'static Texts {
    unsafe {
        match LANG {
            Lang::EN => &EN,
            Lang::FR => &FR,
        }
    }
}
