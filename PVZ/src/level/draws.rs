use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::{entity::Entity, plants::nenuphar::Nenuphar, zombie::Zombie};

use super::{config::RowType, Level};

impl Level {
    pub fn draw_plants(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, ps) in self.plants.iter().enumerate() {
            for (x, plant) in ps.iter().enumerate() {
                if let Some(plant) = plant {
                    if !plant.can_go_in_water() && self.config.rows[y] == RowType::Water {
                        let nenuphar = Nenuphar::new();
                        canvas.copy(
                            nenuphar.texture(),
                            None,
                            Rect::new(
                                self.config.pos_to_coord_x(x) + 5,
                                self.config.pos_to_coord_y(y) + 5,
                                self.config.col_width() - 10,
                                self.config.row_heigth() - 10,
                            ),
                        )?;
                    }
                    canvas.copy(
                        plant.texture(),
                        None,
                        Rect::new(
                            self.config.pos_to_coord_x(x) + 5,
                            self.config.pos_to_coord_y(y) + 5,
                            self.config.col_width() - 10,
                            self.config.row_heigth() - 10,
                        ),
                    )?;
                }
            }
        }
        Ok(())
    }

    pub fn draw_zombies(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, zombies) in self.zombies.iter().enumerate() {
            let mut zombies: Vec<&dyn Zombie> =
                zombies.iter().map(|zombie| zombie.as_ref()).collect();
            zombies.sort_by(|&z1, &z2| z2.pos().total_cmp(&z1.pos()));
            for zombie in zombies {
                canvas.copy(
                    zombie.texture(),
                    None,
                    Rect::new(
                        1280 - (zombie.pos() * 1280.).floor() as i32,
                        self.config.pos_to_coord_y(y) + self.config.row_heigth() as i32
                            - zombie.height() as i32,
                        zombie.width().into(),
                        zombie.height().into(),
                    ),
                )?;
            }
        }
        Ok(())
    }

    pub fn draw_projectiles(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, projs) in self.projectiles.iter().enumerate() {
            for proj in projs {
                canvas.copy(
                    proj.texture(),
                    None,
                    Rect::new(
                        proj.x(),
                        self.config.pos_to_coord_y(y) + self.config.row_heigth() as i32 / 2
                            - proj.height() as i32 / 2,
                        proj.width().into(),
                        proj.height().into(),
                    ),
                )?;
            }
        }
        Ok(())
    }
    pub fn draw_suns(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for sun in self.suns.iter() {
            canvas.copy(
                sun.texture(),
                None,
                Rect::new(sun.x, sun.y as i32, sun.width().into(), sun.height().into()),
            )?;
        }
        Ok(())
    }
}
