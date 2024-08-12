use std::thread;

use pvz::{
    update::{update_available, UPDATE_AVAILABLE},
    win::Win,
};
use sdl::run;

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
