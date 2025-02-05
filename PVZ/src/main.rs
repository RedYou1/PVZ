use std::fs;

use anyhow::{anyhow, Result};
use pvz::{save::SaveFile, textures::load_textures, win::Win, State};
use red_sdl::run_game;

pub fn main() -> Result<()> {
    run_game(
        "Plant Vs Zombie",
        1280,
        720,
        |window| window.fullscreen_desktop().resizable(),
        |canvas| {
            let textures = load_textures(canvas, Box::leak(Box::new(canvas.texture_creator())))?;
            let levels_count = fs::read_dir("levels").map_err(|e| anyhow!(e))?.count();
            if levels_count == 0 || fs::read_dir("levels").map_err(|e| anyhow!(e))?.count() > 99 {
                return Err(anyhow!("Too much or no levels"));
            }
            Ok(State::new(levels_count as u8, SaveFile::load()?, textures))
        },
        Win::new,
    )
}
