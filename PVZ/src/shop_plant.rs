use std::time::Duration;

use anyhow::{anyhow, Result};
use red_sdl::{
    event::Event,
    missing::ui_string::UIString,
    refs::{MutRef, Ref},
    user_control::UserControl,
};
use sdl2::{mouse::MouseButton, pixels::Color, rect::FRect, render::Canvas, video::Window};

use crate::{level::Level, plants::Plant, State};

pub struct ShopPlant {
    action: fn(MutRef<Level>, Box<dyn Plant>, f32, f32),
    surface: FRect,
    plant: Box<dyn Plant>,
}
impl ShopPlant {
    pub fn new(action: fn(MutRef<Level>, Box<dyn Plant>, f32, f32), plant: Box<dyn Plant>) -> Self {
        Self {
            action,
            surface: FRect::new(0., 0., 0., 0.),
            plant,
        }
    }
}
impl UserControl<Level, State> for ShopPlant {
    fn surface(this: Ref<Self>, _: Ref<Level>, _: Ref<State>) -> FRect {
        this.surface
    }

    fn event(
        mut this: MutRef<Self>,
        _: &Canvas<Window>,
        event: Event,
        parent: MutRef<Level>,
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
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } if event.hover(this.surface) => {
                let p = this.plant.clone();
                (this.action)(parent, p, x, y);
            }
            _ => {}
        }
        Ok(())
    }

    fn update(
        _: MutRef<Self>,
        _: &Canvas<Window>,
        _: Duration,
        _: MutRef<Level>,
        _: MutRef<State>,
    ) -> Result<()> {
        Ok(())
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        parent: Ref<Level>,
        state: Ref<State>,
    ) -> Result<()> {
        canvas.set_draw_color(if parent.money >= this.plant.cost() {
            Color::RGB(0, 150, 0)
        } else {
            Color::RGB(150, 0, 0)
        });
        canvas.fill_frect(this.surface).map_err(|e| anyhow!(e))?;
        canvas
            .copy_f(
                this.as_ref().plant.texture(state),
                None,
                FRect::new(
                    this.surface.x(),
                    this.surface.y() + this.surface.height() * 10. / 106.,
                    this.surface.width(),
                    this.surface.height() - this.surface.height() * 20. / 106.,
                ),
            )
            .map_err(|e| anyhow!(e))?;
        let mut text = UIString::new(
            state.as_ref().textures().font(),
            format!("{}$", this.plant.cost()),
        )?;
        if text.is_none() {
            text = UIString::new(
                state.as_ref().textures().font(),
                format!("{}$", this.plant.cost()),
            )?;
        }
        text.ok_or(anyhow!("can't draw money"))?.draw(
            canvas,
            None,
            FRect::new(
                this.surface.x(),
                this.surface.y() + this.surface.height() * 80. / 106.,
                this.surface.width(),
                this.surface.height() * 30. / 106.,
            ),
            Color::WHITE,
        )
    }
}
