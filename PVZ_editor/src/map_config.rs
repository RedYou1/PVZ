use std::time::Duration;

use pvz::{level::config::Map, textures::textures};
use sdl::{event::Event, grid::GridChildren, user_control::UserControl};
use sdl2::{
    rect::{FRect, Rect},
    render::Canvas,
    video::Window,
};

use crate::{pin::Pin, win::Win};

pub struct MapConfig {
    pub map: Map,
    surface: FRect,
    top_left: Pin,
    size: Pin,
}
impl MapConfig {
    pub fn new(id: u8) -> Result<Self, String> {
        Ok(Self {
            map: Map::load(id).map_err(|e| e.to_string())?,
            surface: FRect::new(0., 0., 0., 0.),
            top_left: Pin::empty(),
            size: Pin::empty(),
        })
    }

    pub fn empty() -> Self {
        Self {
            map: Map {
                id: 0,
                top: 0.,
                left: 0.,
                width: 0.,
                height: 0.,
                rows: Vec::new(),
                cols: 0,
            },
            surface: FRect::new(0., 0., 0., 0.),
            top_left: Pin::empty(),
            size: Pin::empty(),
        }
    }
}
impl GridChildren<Win> for MapConfig {
    fn grid_init(&mut self, canvas: &mut Canvas<Window>, _: &mut Win) -> Result<(), String> {
        self.top_left = Pin::new(&mut self.map, true);
        self.size = Pin::new(&mut self.map, false);
        self.top_left.init(canvas)?;
        self.size.init(canvas)
    }

    fn grid_init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        surface: FRect,
        _: &mut Win,
    ) -> Result<(), String> {
        self.surface = surface;
        self.top_left.init_frame(canvas, surface)?;
        self.size.init_frame(canvas, surface)
    }

    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        _: &mut Win,
    ) -> Result<(), String> {
        self.top_left.event(canvas, event.clone())?;
        self.size.event(canvas, event)
    }

    fn grid_update(
        &mut self,
        canvas: &mut Canvas<Window>,
        elapsed: Duration,
        _: &mut Win,
    ) -> Result<(), String> {
        self.top_left.update(canvas, elapsed)?;
        self.size.update(canvas, elapsed)
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &Win) -> Result<(), String> {
        canvas.copy_f(
            &textures()?.maps[self.map.id as usize],
            Some(Rect::new(0, 0, 762, 429)),
            self.surface,
        )?;
        self.top_left.draw(canvas)?;
        self.size.draw(canvas)
    }
}
