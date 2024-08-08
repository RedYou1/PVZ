mod level;
mod plants;
mod projectile;
mod save;
mod shop;
mod sun;
mod texts;
mod textures;
mod win;
mod zombie;

use sdl::run;
use sdl2::rect::{FRect, Rect};
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

pub fn into_rect(rect: FRect) -> Rect {
    Rect::new(
        rect.x() as i32,
        rect.y() as i32,
        rect.width() as u32,
        rect.height() as u32,
    )
}
