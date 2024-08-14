use std::{ptr::null_mut, time::Duration};

use pvz::level::config::Map;
use sdl::{event::Event, user_control::UserControl};
use sdl2::{
    gfx::primitives::DrawRenderer,
    mouse::MouseButton,
    pixels::Color,
    rect::{FPoint, FRect},
    render::Canvas,
    video::Window,
};

const HALF_SIZE: f32 = 15.;

pub struct Pin {
    map: *mut Map,
    _type: bool,
    surface: FRect,
    selected: bool,
}

impl Pin {
    pub fn new(map: *mut Map, _type: bool) -> Self {
        Self {
            map,
            _type,
            surface: FRect::new(0., 0., 0., 0.),
            selected: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            map: null_mut(),
            _type: false,
            surface: FRect::new(0., 0., 0., 0.),
            selected: false,
        }
    }

    pub fn map(&self) -> &Map {
        unsafe { self.map.as_ref().expect("unwrap ptr") }
    }

    pub fn map_mut(&mut self) -> &mut Map {
        unsafe { self.map.as_mut().expect("unwrap ptr") }
    }

    pub fn center(&self) -> (f32, f32) {
        (
            if self._type {
                self.map().left
            } else {
                self.map().left + self.map().width
            } * self.surface.width()
                + self.surface.x(),
            if self._type {
                self.map().top
            } else {
                self.map().top + self.map().height
            } * self.surface.height()
                + self.surface.y(),
        )
    }
}

impl UserControl for Pin {
    fn init(&mut self, _: &mut Canvas<Window>) -> Result<(), String> {
        Ok(())
    }

    fn init_frame(&mut self, _: &mut Canvas<Window>, surface: FRect) -> Result<(), String> {
        self.surface = surface;
        Ok(())
    }

    fn event(&mut self, _: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        match event {
            Event::MouseMotion {
                mousestate, x, y, ..
            } => {
                if mousestate.left() && self.selected {
                    if self._type {
                        self.map_mut().left = ((x - self.surface.x()) / self.surface.width())
                            .clamp(0., 1. - self.map().width);
                        self.map_mut().top = ((y - self.surface.y()) / self.surface.height())
                            .clamp(0., 1. - self.map().height);
                    } else {
                        self.map_mut().width = ((x - self.surface.x()) / self.surface.width())
                            .clamp(self.map().left, 1.)
                            - self.map().left;
                        self.map_mut().height = ((y - self.surface.y()) / self.surface.height())
                            .clamp(self.map().top, 1.)
                            - self.map().top;
                    }
                }
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let (c_x, c_y) = self.center();
                self.selected = FRect::new(
                    c_x - HALF_SIZE,
                    c_y - HALF_SIZE,
                    HALF_SIZE * 2.,
                    HALF_SIZE * 2.,
                )
                .contains_point(FPoint::new(x, y));
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                self.selected = false;
            }
            _ => {}
        }
        Ok(())
    }

    fn update(&mut self, _: &mut Canvas<Window>, _: Duration) -> Result<(), String> {
        Ok(())
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let (x, y) = self.center();
        canvas.filled_circle(x as i16, y as i16, HALF_SIZE as i16, Color::RED)?;
        canvas.set_draw_color(Color::BLACK);
        canvas
            .draw_flines([FPoint::new(x - HALF_SIZE, y), FPoint::new(x + HALF_SIZE, y)].as_ref())?;
        canvas.draw_flines([FPoint::new(x, y - HALF_SIZE), FPoint::new(x, y + HALF_SIZE)].as_ref())
    }
}
