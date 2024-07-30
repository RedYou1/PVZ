use std::time::Duration;

use sdl2::{event::Event, render::Canvas, video::Window};

pub trait GameWindow {
    fn running(&mut self) -> bool;
    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String>;
    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String>;
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}
