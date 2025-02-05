use std::time::Duration;

use anyhow::{anyhow, Result};
use red_sdl::{
    event::Event,
    refs::{MutRef, Ref},
    user_control::UserControl,
};
use sdl2::{rect::FRect, render::Canvas, video::Window};

use crate::{
    level::{config::RowType, Level},
    plants::Plant,
    State,
};

pub struct MapPlant {
    pub row_type: RowType,
    pub plant: Option<Box<dyn Plant>>,
    pub surface: FRect,
}

impl UserControl<Level, State> for MapPlant {
    fn surface(this: Ref<Self>, _: Ref<Level>, _: Ref<State>) -> FRect {
        this.surface
    }

    fn event(
        mut this: MutRef<Self>,
        _: &Canvas<Window>,
        event: Event,
        _: MutRef<Level>,
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
        _: Ref<Level>,
        state: Ref<State>,
    ) -> Result<()> {
        if let Some(plant) = this.as_ref().plant.as_ref() {
            if !plant.can_go_in_water() && this.row_type == RowType::Water {
                canvas
                    .copy_f(
                        state.as_ref().textures().plant_nenuphar(),
                        None,
                        this.surface,
                    )
                    .map_err(|e| anyhow!(e))?;
            }
            canvas
                .copy_f(plant.texture(state), None, this.surface)
                .map_err(|e| anyhow!(e))?;
        }
        Ok(())
    }
}
