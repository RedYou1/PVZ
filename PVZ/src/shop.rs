use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, mouse::MouseButton, pixels::Color, rect::Rect,
    render::Canvas, video::Window,
};

use crate::plant::{Plant, Plant1};

pub struct Shop {
    pub plants: Vec<Box<dyn Plant>>,
    pub dragging: Option<(i32, i32, Box<dyn Plant>)>,
    pub money: usize,
}

impl Shop {
    pub fn new() -> Self {
        Shop {
            plants: vec![Box::new(Plant1::default())],
            dragging: None,
            money: 50,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn event(
        &mut self,
        plants: &[[Option<Box<dyn Plant>>; 9]],
        _: &mut Canvas<Window>,
        event: Event,
    ) -> Result<Option<(usize, usize, Box<dyn Plant>)>, String> {
        let mut ret = None;
        match event {
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if let Some((_, _, plant)) = self.dragging.as_ref() {
                    if self.money >= plant.cost()
                        && (308..=1181).contains(&x)
                        && (102..=687).contains(&y)
                    {
                        let x = ((x - 308) as f32 / 97.).floor() as usize;
                        let y = ((y - 102) as f32 / 117.).floor() as usize;
                        if plants[y][x].is_none() {
                            self.money -= plant.cost();
                            ret = Some((x, y, plant.as_ref().clone()));
                        }
                    }
                    self.dragging = None;
                }
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if let [plant] = self
                    .plants
                    .iter()
                    .enumerate()
                    .filter_map(|(i, plant)| {
                        if x >= i as i32 * 97 + 10
                            && x <= i as i32 * 97 + 10 + plant.width() as i32
                            && y >= 10
                            && y <= 10 + plant.height() as i32
                        {
                            Some(plant.as_ref())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<&dyn Plant>>()[..]
                {
                    if self.money >= plant.cost() {
                        self.dragging = Some((x, y, plant.clone()));
                    }
                }
            }
            Event::MouseMotion { x, y, .. } => {
                if let Some(plant) = self.dragging.as_mut() {
                    plant.0 = x;
                    plant.1 = y;
                }
            }
            _ => {}
        }
        Ok(ret)
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.fill_rect(Rect::new(0, 0, self.plants.len() as u32 * 97 + 100, 130))?;
        for (i, plant) in self.plants.iter().enumerate() {
            canvas.copy(
                plant.texture(),
                None,
                Rect::new(i as i32 * 97 + 10, 10, 80, 106),
            )?;
        }
        if let Some((x, y, plant)) = self.dragging.as_ref() {
            canvas.copy(plant.texture(), None, Rect::new(*x - 40, *y - 53, 80, 106))?;
        }
        const SCALE: i16 = 3;
        canvas.set_scale(SCALE as f32, SCALE as f32)?;
        canvas.string(
            (self.plants.len() as i16 * 97 + 10) / SCALE,
            30 / SCALE,
            format!("{}$", self.money).as_str(),
            Color::RGB(255, 255, 255),
        )?;
        canvas.set_scale(1., 1.)
    }
}
