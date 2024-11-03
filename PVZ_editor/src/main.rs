mod level_config;
mod map_config;
mod pin;
mod rows_editor;
mod win;

use crate::win::Win;
use red_sdl::run;

pub fn main() -> Result<(), String> {
    run(
        "Plant Vs Zombie Editor",
        30.,
        1280,
        720,
        |window| window.fullscreen_desktop().resizable(),
        Win::new,
    )
}
