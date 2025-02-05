use anyhow::Result;
use rand::Rng;
use std::time::Duration;

use crate::{sun::Sun, zombie::zombie_from_id};

use super::{
    collision::{do_damage_to_plant, do_damage_to_zombies},
    Level,
};

impl Level {
    pub(super) fn update_zombies(&mut self, elapsed: Duration) -> Result<()> {
        for (y, zombies) in self.zombies.iter_mut().enumerate() {
            for zombie in zombies.iter_mut() {
                let prev_x = zombie.rect(0.).x();
                zombie.update(elapsed)?;

                if zombie.rect(0.).x() + zombie.rect(0.).width() < self.map.left {
                    self.end = Some(false);
                    return Ok(());
                } else {
                    do_damage_to_plant(
                        zombie.as_mut(),
                        y,
                        &mut self.map_plants,
                        &self.map,
                        self.map.rows[y],
                        prev_x,
                        elapsed,
                    );
                }
            }
        }
        Ok(())
    }

    pub(super) fn update_projectiles(&mut self, elapsed: Duration) -> Result<()> {
        for (y, projs) in self.projectiles.iter_mut().enumerate() {
            let mut indx = Vec::new();
            for (i, proj) in projs.iter_mut().enumerate() {
                proj.update(elapsed)?;

                let proj = proj.as_ref();

                if proj.to_remove() {
                    indx.insert(0, i);
                    continue;
                }

                let mut zombie_to_remove = do_damage_to_zombies(self.zombies[y].as_mut(), proj);
                if zombie_to_remove.0 {
                    indx.insert(0, i);
                }
                zombie_to_remove.1.sort();
                zombie_to_remove.1.reverse();
                zombie_to_remove.1.dedup();
                for zombie_index in zombie_to_remove.1 {
                    self.zombies[y].remove(zombie_index);
                }
            }
            for i in indx {
                projs.remove(i);
            }
        }
        Ok(())
    }

    pub(super) fn update_suns(&mut self, elapsed: Duration) -> Result<()> {
        for sun in self.suns.iter_mut() {
            sun.update(elapsed)?;
        }
        if self.next_sun > elapsed {
            self.next_sun -= elapsed
        } else {
            self.next_sun = Duration::new(5, 0) - elapsed + self.next_sun;
            let mut rng = rand::rng();
            self.suns.push(Sun::new(
                rng.random_range(0.0..1.0),
                0.,
                rng.random_range(200.0..420.) / 720.,
            ));
        }
        Ok(())
    }

    pub(super) fn update_zombie_wave(&mut self, mut elapsed: Duration) {
        if !self.spawn_waits.is_empty() {
            if let Some(&f) = self.spawn_waits.first() {
                if elapsed >= f {
                    elapsed -= f;
                    self.spawn_waits.remove(0);
                    let mut z = self.spawn_zombies.remove(0);
                    let mut rng = rand::rng();
                    let mut offsets: Vec<f32> = (0..self.map.rows.len()).map(|_| 1.).collect();
                    while !z.is_empty() {
                        let i = rng.random_range(0..z.len());
                        let mut z = zombie_from_id(z.remove(i).0);
                        let i = rng.random_range(0..self.map.rows.len()) as usize;
                        z.set_x(offsets[i]);
                        offsets[i] += 7.68 / 1280.;
                        self.zombies[i].push(z);
                    }
                }
            }
            if let Some(f) = self.spawn_waits.first_mut() {
                *f -= elapsed;
            }
        }
    }
}
