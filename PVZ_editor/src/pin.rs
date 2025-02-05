use std::time::Duration;

use anyhow::{anyhow, Result};
use pvz::level::config::Map;
use red_sdl::{
    event::Event,
    refs::{MutRef, Ref},
    user_control::UserControl,
    zero,
};
use sdl2::{
    gfx::primitives::DrawRenderer,
    mouse::MouseButton,
    pixels::Color,
    rect::{FPoint, FRect},
    render::Canvas,
    video::Window,
};

use crate::{map_config::MapConfig, State};

const HALF_SIZE: f32 = 15.;

pub struct Pin {
    map: MutRef<Map>,
    _type: bool,
    surface: FRect,
    selected: bool,
}

impl Pin {
    pub fn new(map: MutRef<Map>, _type: bool) -> Self {
        Self {
            map,
            _type,
            surface: zero(),
            selected: false,
        }
    }

    pub fn center(&self) -> (f32, f32) {
        (
            if self._type {
                self.map.left
            } else {
                self.map.left + self.map.width
            } * self.surface.width()
                + self.surface.x(),
            if self._type {
                self.map.top
            } else {
                self.map.top + self.map.height
            } * self.surface.height()
                + self.surface.y(),
        )
    }
}

impl UserControl<MapConfig, State> for Pin {
    fn surface(this: Ref<Self>, _: Ref<MapConfig>, _: Ref<State>) -> FRect {
        this.surface
    }

    fn event(
        mut this: MutRef<Self>,
        _: &Canvas<Window>,
        event: Event,
        _: MutRef<MapConfig>,
        _: MutRef<State>,
    ) -> Result<()> {
        match event {
            Event::ElementMove { x, y } => {
                this.surface.set_x(x);
                this.surface.set_y(y);
            }
            Event::ElementResize { width, height } => {
                this.surface.set_width(width);
                this.surface.set_height(height);
            }
            Event::MouseMotion {
                mousestate, x, y, ..
            } => {
                if mousestate.left() && this.selected {
                    if this._type {
                        let x = ((x - this.surface.x()) / this.surface.width())
                            .clamp(0., 1. - this.map.width);
                        let y = ((y - this.surface.y()) / this.surface.height())
                            .clamp(0., 1. - this.map.height);
                        this.map.left = x;
                        this.map.top = y;
                    } else {
                        let w = ((x - this.surface.x()) / this.surface.width())
                            .clamp(this.map.left, 1.)
                            - this.map.left;
                        let h = ((y - this.surface.y()) / this.surface.height())
                            .clamp(this.map.top, 1.)
                            - this.map.top;
                        this.map.width = w;
                        this.map.height = h;
                    }
                }
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let (c_x, c_y) = this.center();
                this.selected = FRect::new(
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
                this.selected = false;
            }
            _ => {}
        }
        Ok(())
    }

    fn update(
        _: MutRef<Self>,
        _: &Canvas<Window>,
        _: Duration,
        _: MutRef<MapConfig>,
        _: MutRef<State>,
    ) -> Result<()> {
        Ok(())
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        _: Ref<MapConfig>,
        _: Ref<State>,
    ) -> Result<()> {
        let (x, y) = this.center();
        canvas
            .filled_circle(x as i16, y as i16, HALF_SIZE as i16, Color::RED)
            .map_err(|e| anyhow!(e))?;
        canvas.set_draw_color(Color::BLACK);
        canvas
            .draw_flines([FPoint::new(x - HALF_SIZE, y), FPoint::new(x + HALF_SIZE, y)].as_ref())
            .map_err(|e| anyhow!(e))?;
        canvas
            .draw_flines([FPoint::new(x, y - HALF_SIZE), FPoint::new(x, y + HALF_SIZE)].as_ref())
            .map_err(|e| anyhow!(e))
    }
}
