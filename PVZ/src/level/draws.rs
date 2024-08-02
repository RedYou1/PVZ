use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::zombie::Zombie;

use super::Level;

impl Level {
    pub fn draw_plants(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, ps) in self.plants.iter().enumerate() {
            for (x, p) in ps.iter().enumerate() {
                if let Some(p) = p {
                    canvas.copy(
                        p.texture(),
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
        for (y, zs) in self.zombies.iter().enumerate() {
            let mut zs: Vec<&dyn Zombie> = zs.iter().map(|z| z.as_ref()).collect();
            zs.sort_by(|&z1, &z2| z2.pos().total_cmp(&z1.pos()));
            for z in zs {
                canvas.copy(
                    z.texture(),
                    None,
                    Rect::new(
                        1280 - (z.pos() * 1280.).floor() as i32,
                        self.config.pos_to_coord_y(y) + self.config.row_heigth() as i32
                            - z.height() as i32,
                        z.width().into(),
                        z.height().into(),
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
}
