use anyhow::{anyhow, Result};
use red_sdl::{missing::rect::scale, refs::Ref};
use sdl2::{render::Canvas, video::Window};

use crate::{sun::Sun, zombie::Zombie, State};

use super::Level;

impl Level {
    pub fn draw_zombies(
        &'static self,
        canvas: &mut Canvas<Window>,
        state: &'static State,
    ) -> Result<()> {
        for (y, zombies) in self.zombies.iter().enumerate() {
            let mut zombies: Vec<&dyn Zombie> =
                zombies.iter().map(|zombie| zombie.as_ref()).collect();
            zombies.sort_by(|&z1, &z2| z1.rect(0.).left().total_cmp(&z2.rect(0.).left()));
            for zombie in zombies {
                canvas
                    .copy_f(
                        zombie.texture(state.textures()),
                        None,
                        scale(
                            self.surface,
                            zombie.rect(
                                self.map.pos_to_coord_y(y) + self.map.row_heigth()
                                    - zombie.rect(0.).height(),
                            ),
                        ),
                    )
                    .map_err(|e| anyhow!(e))?;
            }
        }
        Ok(())
    }

    pub fn draw_projectiles(
        &'static self,
        canvas: &mut Canvas<Window>,
        state: &'static State,
    ) -> Result<()> {
        for (y, projs) in self.projectiles.iter().enumerate() {
            for proj in projs {
                canvas
                    .copy_f(
                        proj.texture(state.into()),
                        None,
                        scale(
                            self.surface,
                            proj.rect(
                                self.map.pos_to_coord_y(y) + self.map.row_heigth() / 2.
                                    - proj.rect(0.).height() / 2.,
                            ),
                        ),
                    )
                    .map_err(|e| anyhow!(e))?;
            }
        }
        Ok(())
    }
    pub fn draw_suns(&'static self, canvas: &mut Canvas<Window>, state: Ref<State>) -> Result<()> {
        for sun in self.suns.iter() {
            canvas
                .copy_f(Sun::texture(state), None, scale(self.surface, sun.rect()))
                .map_err(|e| anyhow!(e))?;
        }
        Ok(())
    }
}
