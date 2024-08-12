mod win;

use crate::win::Win;
use sdl::run;

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
