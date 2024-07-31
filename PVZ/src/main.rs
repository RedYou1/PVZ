mod entity;
mod level;
mod plant;
mod projectile;
mod shop;
mod textures;
mod win;
mod zombie;

use sdl::run;
use win::Win;

pub fn main() -> Result<(), String> {
    run(
        "Plant Vs Zombie",
        60.,
        1280,
        720,
        |window| window.fullscreen_desktop().resizable(),
        Win::new,
    )
}
