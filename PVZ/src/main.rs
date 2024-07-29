mod entity;
mod level;
mod plant;
mod textures;
mod win;
mod zombie;

use sdl::run;
use win::Win;

pub fn main() -> Result<(), String> {
    run("Plant Vs Zombie", 60., 1280, 720, Win::new)
}
