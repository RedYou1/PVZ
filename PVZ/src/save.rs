use std::{fs, path::Path};

use crate::texts::{texts, Lang, Texts};

pub struct SaveFile {
    langage: Lang,
}

impl SaveFile {
    pub fn load() -> Result<SaveFile, String> {
        if !Path::new("save.data").exists() {
            return Ok(SaveFile { langage: Lang::EN });
        }
        let mut data = fs::read("save.data").map_err(|e| e.to_string())?;
        let langage = match data.remove(0) {
            0 => Lang::EN,
            1 => Lang::FR,
            _ => return Err("lang not recognized".to_owned()),
        };
        Ok(SaveFile { langage })
    }

    fn save(&self) -> Result<(), String> {
        fs::write(
            "save.data",
            [match self.langage {
                Lang::EN => 0,
                Lang::FR => 1,
            }],
        )
        .map_err(|e| e.to_string())
    }

    pub const fn langage(&self) -> Lang {
        self.langage
    }

    pub fn set_langage(&mut self, lang: Lang) -> Result<(), String> {
        self.langage = lang;
        self.save()
    }

    pub fn next_lang(&mut self) -> Result<(), String> {
        self.set_langage(match self.langage {
            Lang::EN => Lang::FR,
            Lang::FR => Lang::EN,
        })
    }

    pub fn texts(&self) -> Result<&'static Texts, String> {
        texts(self.langage)
    }
}
