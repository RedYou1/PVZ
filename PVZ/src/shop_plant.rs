use std::{marker::PhantomData, time::Duration};

use sdl::{event::Event, grid::GridChildren};
use sdl2::{mouse::MouseButton, pixels::Color, rect::FRect, render::Canvas, video::Window};

use crate::{plants::Plant, textures::draw_string};

pub struct ShopPlant<T, Func: FnMut(&mut T, &dyn Plant, f32, f32)> {
    parent: PhantomData<T>,
    action: Func,
    surface: FRect,
    plant: Box<dyn Plant>,
}
impl<T, Func: FnMut(&mut T, &dyn Plant, f32, f32)> ShopPlant<T, Func> {
    pub fn new(action: Func, plant: Box<dyn Plant>) -> Self {
        Self {
            parent: PhantomData,
            action,
            surface: FRect::new(0., 0., 0., 0.),
            plant,
        }
    }
}
impl<T, Func: FnMut(&mut T, &dyn Plant, f32, f32)> GridChildren<T>
    for ShopPlant<T, Func>
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
        _: &mut Canvas<Window>,
        event: Event,
        parent: &mut T,
    ) -> Result<(), String> {
        if let Event::MouseButtonDown {
            mouse_btn: MouseButton::Left,
            x,
            y,
            ..
        } = event
        {
            (self.action)(parent, self.plant.as_ref(), x, y);
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

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &T) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_frect(self.surface)?;
        canvas.copy_f(
            self.plant.texture()?,
            None,
            FRect::new(
                self.surface.x(),
                self.surface.y() + self.surface.height() * 10. / 106.,
                self.surface.width(),
                self.surface.height() - self.surface.height() * 20. / 106.,
            ),
        )?;
        draw_string(
            canvas,
            FRect::new(
                self.surface.x(),
                self.surface.y() + self.surface.height() * 80. / 106.,
                self.surface.width(),
                self.surface.height() * 30. / 106.,
            ),
            format!("{}$", self.plant.cost()).as_str(),
        )
    }
}
