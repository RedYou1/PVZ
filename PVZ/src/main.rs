use std::thread;

use pvz::{win::Win, UPDATE_AVAILABLE};
use red_sdl::run;
use serde_json::Value;

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

pub fn update_available() -> Result<bool, String> {
    let req = reqwest::blocking::Client::builder()
        .build()
        .map_err(|e| e.to_string())?
        .get("https://api.github.com/repos/RedYou1/PVZ/releases")
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
    Ok(first.ne("pvz_v0.1.5"))
}
