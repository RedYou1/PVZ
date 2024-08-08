use sdl2::{rect::FRect, render::Canvas, video::Window};

use crate::{
    into_rect,
    plants::{nenuphar::Nenuphar, Plant},
    zombie::Zombie,
};

use super::{config::RowType, Level};

impl Level {
    pub fn draw_plants(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, ps) in self.plants.iter().enumerate() {
            for (x, plant) in ps.iter().enumerate() {
                if let Some(plant) = plant {
                    let rect = into_rect(FRect::new(
                        self.config.pos_to_coord_x(x) + 5.,
                        self.config.pos_to_coord_y(y) + 5.,
                        self.config.col_width() - 10.,
                        self.config.row_heigth() - 10.,
                    ));
                    if !plant.can_go_in_water() && self.config.rows[y] == RowType::Water {
                        let nenuphar = Nenuphar::new();
                        canvas.copy(nenuphar.texture()?, None, rect)?;
                    }
                    canvas.copy(plant.texture()?, None, rect)?;
                }
            }
        }
        Ok(())
    }

    pub fn draw_zombies(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, zombies) in self.zombies.iter().enumerate() {
            let mut zombies: Vec<&dyn Zombie> =
                zombies.iter().map(|zombie| zombie.as_ref()).collect();
            zombies.sort_by(|&z1, &z2| z1.rect(0.).left().total_cmp(&z2.rect(0.).left()));
            for zombie in zombies {
                canvas.copy(
                    zombie.texture()?,
                    None,
                    into_rect(zombie.rect(
                        self.config.pos_to_coord_y(y) + self.config.row_heigth()
                            - zombie.rect(0.).height(),
                    )),
                )?;
            }
        }
        Ok(())
    }

    pub fn draw_projectiles(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, projs) in self.projectiles.iter().enumerate() {
            for proj in projs {
                canvas.copy(
                    proj.texture()?,
                    None,
                    into_rect(proj.rect(
                        self.config.pos_to_coord_y(y) + self.config.row_heigth() / 2.
                            - proj.rect(0.).height() / 2.,
                    )),
                )?;
            }
        }
        Ok(())
    }
    pub fn draw_suns(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for sun in self.suns.iter() {
            canvas.copy(sun.texture()?, None, into_rect(sun.rect()))?;
        }
        Ok(())
    }
}
