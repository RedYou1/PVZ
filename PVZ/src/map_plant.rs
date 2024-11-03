use std::time::Duration;

use red_sdl::{event::Event, grid::GridChildren};
use sdl2::{rect::FRect, render::Canvas, video::Window};

use crate::{
    level::{config::RowType, Level},
    plants::{nenuphar::Nenuphar, Plant},
};

pub struct MapPlant {
    pub row_type: RowType,
    pub plant: *const Option<Box<dyn Plant>>,
    pub surface: FRect,
}

impl GridChildren<Level> for MapPlant {
    fn grid_init(&mut self, _: &mut Canvas<Window>, _: &mut Level) -> Result<(), String> {
        Ok(())
    }

    fn grid_init_frame(
        &mut self,
        _: &mut Canvas<Window>,
        surface: FRect,
        _: &mut Level,
    ) -> Result<(), String> {
        self.surface = surface;
        Ok(())
    }

    fn grid_event(
        &mut self,
        _: &mut Canvas<Window>,
        _: Event,
        _: &mut Level,
    ) -> Result<(), String> {
        Ok(())
    }

    fn grid_update(
        &mut self,
        _: &mut Canvas<Window>,
        _: Duration,
        _: &mut Level,
    ) -> Result<(), String> {
        Ok(())
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &Level) -> Result<(), String> {
        if let Some(plant) = unsafe { self.plant.as_ref().ok_or("unwrap ptr draw map_plant")? } {
            if !plant.can_go_in_water() && self.row_type == RowType::Water {
                let nenuphar = Nenuphar::new();
                canvas.copy_f(nenuphar.texture()?, None, self.surface)?;
            }
            canvas.copy_f(plant.texture()?, None, self.surface)?;
        }
        Ok(())
    }
}
