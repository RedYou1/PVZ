use std::{marker::PhantomData, time::Duration};

use sdl::{event::Event, grid::GridChildren};
use sdl2::{mouse::MouseButton, pixels::Color, rect::FRect, render::Canvas, video::Window};

use crate::textures::draw_string;

pub struct Button<
    T,
    Text: Into<String>,
    Func: FnMut(&mut T, f32, f32, &mut Canvas<Window>) -> Result<(), String>,
    Func2: Fn(&T) -> Text,
> {
    parent: PhantomData<T>,
    action: Func,
    surface: FRect,
    text: Func2,
}
impl<
        T,
        Text: Into<String>,
        Func: FnMut(&mut T, f32, f32, &mut Canvas<Window>) -> Result<(), String>,
        Func2: Fn(&T) -> Text,
    > Button<T, Text, Func, Func2>
{
    pub fn new(action: Func, text: Func2) -> Self {
        Self {
            parent: PhantomData,
            action,
            surface: FRect::new(0., 0., 0., 0.),
            text,
        }
    }
}
impl<
        T,
        Text: Into<String>,
        Func: FnMut(&mut T, f32, f32, &mut Canvas<Window>) -> Result<(), String>,
        Func2: Fn(&T) -> Text,
    > GridChildren<T> for Button<T, Text, Func, Func2>
{
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

    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        parent: &mut T,
    ) -> Result<(), String> {
        let text = (self.text)(parent).into();
        let text = text.as_str();
        if text.is_empty() {
            return Ok(());
        }
        if let Event::MouseButtonUp {
            mouse_btn: MouseButton::Left,
            x,
            y,
            ..
        } = event
        {
            (self.action)(parent, x, y, canvas)?;
        }
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
        let text = (self.text)(parent).into();
        let text = text.as_str();
        if text.is_empty() {
            return Ok(());
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_frect(self.surface)?;
        draw_string(canvas, self.surface, text)
    }
}
