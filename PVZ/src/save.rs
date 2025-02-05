use std::{fs, path::Path};

use anyhow::{anyhow, Result};

use crate::texts::Lang;

pub struct SaveFile {
    pub langage: Lang,
}

impl SaveFile {
    pub fn load() -> Result<SaveFile> {
        if !Path::new("save.data").exists() {
            return Ok(SaveFile { langage: Lang::EN });
        }
        let mut data = fs::read("save.data").map_err(|e| anyhow!(e))?;
        let langage = match data.remove(0) {
            0 => Lang::EN,
            1 => Lang::FR,
            _ => return Err(anyhow!("lang not recognized")),
        };
        Ok(SaveFile { langage })
    }

    fn save(&self) -> Result<()> {
        fs::write(
            "save.data",
            [match self.langage {
                Lang::EN => 0,
                Lang::FR => 1,
            }],
        )
        .map_err(|e| anyhow!(e))
    }

    pub const fn langage(&self) -> Lang {
        self.langage
    }

    pub fn set_langage(&mut self, lang: Lang) -> Result<()> {
        self.langage = lang;
        self.save()
    }

    pub fn next_lang(&mut self) -> Result<()> {
        self.set_langage(match self.langage {
            Lang::EN => Lang::FR,
            Lang::FR => Lang::EN,
        })
    }
}
