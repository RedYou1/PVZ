use std::{marker::PhantomData, time::Duration};

use red_sdl::{event::Event, grid::GridChildren, missing::ui_string::UIString};
use sdl2::{mouse::MouseButton, pixels::Color, rect::FRect, render::Canvas, video::Window};

use crate::{level::Level, plants::Plant, textures::textures};

pub struct ShopPlant<Func: FnMut(&mut Level, &dyn Plant, f32, f32)> {
    parent: PhantomData<Level>,
    action: Func,
    surface: FRect,
    plant: Box<dyn Plant>,
}
impl<Func: FnMut(&mut Level, &dyn Plant, f32, f32)> ShopPlant<Func> {
    pub fn new(action: Func, plant: Box<dyn Plant>) -> Self {
        Self {
            parent: PhantomData,
            action,
            surface: FRect::new(0., 0., 0., 0.),
            plant,
        }
    }
}
impl<Func: FnMut(&mut Level, &dyn Plant, f32, f32)> GridChildren<Level> for ShopPlant<Func> {
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
        event: Event,
        parent: &mut Level,
    ) -> Result<(), String> {
        if let (
            true,
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            },
        ) = (event.hover(self.surface), event)
        {
            (self.action)(parent, self.plant.as_ref(), x, y);
        }
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

    fn grid_draw(&self, canvas: &mut Canvas<Window>, parent: &Level) -> Result<(), String> {
        canvas.set_draw_color(if parent.money >= self.plant.cost() {
            Color::RGB(0, 150, 0)
        } else {
            Color::RGB(150, 0, 0)
        });
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
        let mut text = UIString::new(&textures()?.font, format!("{}$", self.plant.cost()))?;
        if text.is_none() {
            text = UIString::new(&textures()?.font, format!("{}$", self.plant.cost()))?;
        }
        text.ok_or("can't draw money".to_owned())?.draw(
            canvas,
            None,
            FRect::new(
                self.surface.x(),
                self.surface.y() + self.surface.height() * 80. / 106.,
                self.surface.width(),
                self.surface.height() * 30. / 106.,
            ),
            Color::WHITE,
        )
    }
}
