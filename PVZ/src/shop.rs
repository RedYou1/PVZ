use sdl2::{
    event::Event, mouse::MouseButton, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::{
    level::LevelConfig,
    plant::{PeaShooter, Plant, PlantTriple},
    projectile::DamageType,
    textures::draw_string,
};

pub struct Shop {
    pub plants: Vec<Box<dyn Plant>>,
    pub dragging: Option<(i32, i32, Box<dyn Plant>)>,
    pub money: u32,
}

impl Shop {
    pub fn new(money: u32) -> Self {
        Shop {
            plants: vec![
                Box::new(PeaShooter::new(DamageType::Normal)),
                Box::new(PeaShooter::new(DamageType::Ice)),
                Box::new(PeaShooter::new(DamageType::Fire)),
                Box::new(PlantTriple::default()),
            ],
            dragging: None,
            money,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn event(
        &mut self,
        config: &LevelConfig,
        plants: &[Vec<Option<Box<dyn Plant>>>],
        canvas: &mut Canvas<Window>,
        event: Event,
    ) -> Result<Option<(usize, usize, Box<dyn Plant>)>, String> {
        let (width, height) = canvas.output_size()?;
        let scale_x = |x: i32| x as f32 * 1280. / width as f32;
        let scale_y = |y: i32| y as f32 * 720. / height as f32;

        let mut ret = None;
        match event {
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if let Some((_, _, plant)) = self.dragging.as_ref() {
                    if self.money >= plant.cost() {
                        if let Some(x) = config.coord_to_pos_x(scale_x(x) as i32) {
                            if let Some(y) = config.coord_to_pos_y(scale_y(y) as i32) {
                                if plants[y][x].is_none() {
                                    self.money -= plant.cost();
                                    ret = Some((x, y, plant.as_ref().clone()));
                                }
                            }
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
                let x = scale_x(x) as i32;
                let y = scale_y(y) as i32;
                if self.dragging.is_none() {
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
                        self.dragging = Some((x, y, plant.clone()));
                    }
                }
            }
            Event::MouseMotion { x, y, .. } => {
                if let Some(plant) = self.dragging.as_mut() {
                    plant.0 = scale_x(x) as i32;
                    plant.1 = scale_y(y) as i32;
                }
            }
            _ => {}
        }
        Ok(ret)
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, config: &LevelConfig) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(0, 0, self.plants.len() as u32 * 97 + 100, 130))?;
        for (i, plant) in self.plants.iter().enumerate() {
            canvas.copy(
                plant.texture(),
                None,
                Rect::new(i as i32 * 97 + 10, 10, 80, 106),
            )?;
        }
        if let Some((x, y, plant)) = self.dragging.as_ref() {
            canvas.copy(
                plant.texture(),
                None,
                Rect::new(
                    *x - 40,
                    *y - 53,
                    config.col_width() - 10,
                    config.row_heigth() - 10,
                ),
            )?;
        }
        draw_string(
            canvas,
            Rect::new(self.plants.len() as i32 * 97 + 10, 42, 80, 106),
            format!("{}$", self.money).as_str(),
            Color::RGB(255, 255, 255),
        )
    }
}
