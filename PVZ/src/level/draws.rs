use sdl::scale;
use sdl2::{render::Canvas, video::Window};

use crate::zombie::Zombie;

use super::Level;

impl Level {
    pub fn draw_zombies(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, zombies) in self.zombies.iter().enumerate() {
            let mut zombies: Vec<&dyn Zombie> =
                zombies.iter().map(|zombie| zombie.as_ref()).collect();
            zombies.sort_by(|&z1, &z2| z1.rect(0.).left().total_cmp(&z2.rect(0.).left()));
            for zombie in zombies {
                canvas.copy_f(
                    zombie.texture()?,
                    None,
                    scale(
                        self.surface,
                        zombie.rect(
                            self.config.pos_to_coord_y(y) + self.config.row_heigth()
                                - zombie.rect(0.).height(),
                        ),
                    ),
                )?;
            }
        }
        Ok(())
    }

    pub fn draw_projectiles(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, projs) in self.projectiles.iter().enumerate() {
            for proj in projs {
                canvas.copy_f(
                    proj.texture()?,
                    None,
                    scale(
                        self.surface,
                        proj.rect(
                            self.config.pos_to_coord_y(y) + self.config.row_heigth() / 2.
                                - proj.rect(0.).height() / 2.,
                        ),
                    ),
                )?;
            }
        }
        Ok(())
    }
    pub fn draw_suns(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for sun in self.suns.iter() {
            canvas.copy_f(sun.texture()?, None, scale(self.surface, sun.rect()))?;
        }
        Ok(())
    }
}
