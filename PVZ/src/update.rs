use std::{marker::PhantomData, time::Duration};

use sdl::{event::Event, grid::GridChildren};
use sdl2::{pixels::Color, rect::FRect, render::Canvas, video::Window};
use serde_json::Value;

use crate::{texts::Texts, textures::draw_string};

pub static mut UPDATE_AVAILABLE: Option<Result<bool, String>> = None;

pub fn update_available() -> Result<bool, String> {
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

pub struct Update<T, Func: Fn(&T) -> &'static Texts> {
    surface: FRect,
    parent: PhantomData<T>,
    texts: Func,
}

impl<T, Func: Fn(&T) -> &'static Texts> Update<T, Func> {
    pub fn new(texts: Func) -> Self {
        Self {
            surface: FRect::new(0., 0., 0., 0.),
            parent: PhantomData,
            texts,
        }
    }
}
impl<T, Func: Fn(&T) -> &'static Texts> GridChildren<T> for Update<T, Func> {
    fn grid_init(&mut self, _: &mut Canvas<Window>, _: &mut T) -> Result<(), String> {
        Ok(())
    }

    fn grid_init_frame(
        &mut self,
        _: &mut Canvas<Window>,
        surface: FRect,
        _: &mut T,
    ) -> Result<(), String> {
        self.surface = surface;
        Ok(())
    }

    fn grid_event(&mut self, _: &mut Canvas<Window>, _: Event, _: &mut T) -> Result<(), String> {
        Ok(())
    }

    fn grid_update(
        &mut self,
        _: &mut Canvas<Window>,
        _: Duration,
        _: &mut T,
    ) -> Result<(), String> {
        Ok(())
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, parent: &T) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_frect(self.surface)?;
        let texts = (self.texts)(parent);
        draw_string(
            canvas,
            self.surface,
            match unsafe { UPDATE_AVAILABLE.as_ref() } {
                Some(Ok(true)) => texts.update_available,
                Some(Ok(false)) => texts.up_to_date,
                Some(Err(e)) => e,
                None => texts.loading,
            },
        )
    }
}
