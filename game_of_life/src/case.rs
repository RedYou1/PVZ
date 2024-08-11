use std::time::Duration;

use sdl::{event::Event, grid::GridChildren};
use sdl2::{
    mouse::MouseButton,
    rect::FRect,
    render::{Canvas, Texture},
    video::Window,
};

use crate::{clock, set_state, state};

pub struct Case {
    pub x: usize,
    pub y: usize,
    pub texture1: &'static Texture<'static>,
    pub texture2: &'static Texture<'static>,
    pub surface: FRect,
}

impl GridChildren<()> for Case {
    fn grid_init(&mut self, _: &mut Canvas<Window>, _: &mut ()) -> Result<(), String> {
        Ok(())
    }

    fn grid_init_frame(
        &mut self,
        _: &mut Canvas<Window>,
        surface: FRect,
        _: &mut (),
    ) -> Result<(), String> {
        self.surface = surface;
        Ok(())
    }

    fn grid_event(
        &mut self,
        _: &mut Canvas<Window>,
        event: Event,
        _: &mut (),
    ) -> Result<(), String> {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                set_state(true, self.x, self.y);
            }
            Event::MouseMotion { mousestate, .. } if mousestate.left() => {
                set_state(true, self.x, self.y);
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Right,
                ..
            } => {
                set_state(false, self.x, self.y);
            }
            Event::MouseMotion { mousestate, .. } if mousestate.right() => {
                set_state(false, self.x, self.y);
            }
            _ => {}
        }
        Ok(())
    }

    fn grid_update(
        &mut self,
        _: &mut Canvas<Window>,
        _: Duration,
        _: &mut (),
    ) -> Result<(), String> {
        Ok(())
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &()) -> Result<(), String> {
        if state(self.x, self.y) {
            canvas.copy_f(
                if clock() >= 15 {
                    self.texture1
                } else {
                    self.texture2
                },
                None,
                self.surface,
            )?;
        }
        Ok(())
    }
}
