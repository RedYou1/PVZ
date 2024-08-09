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

use std::thread;

use sdl::run;
use sdl2::rect::{FRect, Rect};
use serde_json::Value;
use win::Win;

static mut UPDATE_AVAILABLE: Option<Result<bool, String>> = None;

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

fn update_available() -> Result<bool, String> {
    let req = reqwest::blocking::Client::builder()
        .build()
        .map_err(|e| e.to_string())?
        .get("https://api.github.com/repos/RedYou1/SDL/releases")
        .header("User-Agent", "PVZ")
        .send()
        .map_err(|e| e.to_string())?;
    let text = req.text().map_err(|e| e.to_string())?;
    let json: Value = serde_json::from_str(text.as_str()).map_err(|e| e.to_string())?;
    let releases = json.as_array().ok_or("Error fetching".to_owned())?;
    let releases: Vec<&str> = releases
        .iter()
        .filter_map(|e| e["tag_name"].as_str())
        .skip_while(|e| !e.starts_with("pvz"))
        .collect();
    let first = *releases.first().ok_or("Error fetching".to_owned())?;
    Ok(first.ne("pvz_v0.1.4"))
}

pub fn into_rect(rect: FRect) -> Rect {
    Rect::new(
        rect.x() as i32,
        rect.y() as i32,
        rect.width() as u32,
        rect.height() as u32,
    )
}
