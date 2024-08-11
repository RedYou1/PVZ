mod button;
mod level;
mod map_plant;
mod plants;
mod projectile;
mod save;
mod shop_plant;
mod sun;
mod texts;
mod textures;
mod update;
mod win;
mod zombie;

use std::thread;

use sdl::run;
use sdl2::rect::FRect;
use update::{update_available, UPDATE_AVAILABLE};
use win::Win;

pub fn main() -> Result<(), String> {
    let t = thread::spawn(|| unsafe { UPDATE_AVAILABLE = Some(update_available()) });
    run(
        "Plant Vs Zombie",
        60.,
        1280,
        720,
        |window| window.fullscreen_desktop().resizable(),
        Win::new,
    )?;
    t.join()
        .map_err(|_| "Error join update available".to_owned())
}

pub fn scale(surface: FRect, scale: FRect) -> FRect {
    FRect::new(
        scale.x() * surface.width() + surface.x(),
        scale.y() * surface.height() + surface.y(),
        scale.width() * surface.width(),
        scale.height() * surface.height(),
    )
}
