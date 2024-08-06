use std::time::Duration;

use crate::{
    plants::{nenuphar::Nenuphar, Plant},
    projectile::DamageType,
    zombie::Zombie,
};

use super::{
    config::{LevelConfig, RowType},
    Level,
};

impl Level {
    pub(super) fn spawn_projectiles(&mut self) {
        for (y, plants) in self.plants.iter_mut().enumerate() {
            for (x, plant) in plants.iter_mut().enumerate() {
                if let Some(plant) = plant {
                    let mut spawns = plant.should_spawn(
                        self.config.pos_to_coord_x(x) + plant.width() as i32 / 2,
                        self.config.pos_to_coord_y(y),
                        y,
                        self.config.rows.len() - 1,
                        &self.zombies,
                    );
                    self.suns.append(&mut spawns.0);
                    for (y, proj) in spawns.1 {
                        self.projectiles[y].push(proj);
                    }
                }
            }
        }
    }
}

pub(super) fn do_damage_to_plant(
    zombie: &mut dyn Zombie,
    plants: &mut [Option<Box<dyn Plant>>],
    config: &LevelConfig,
    row_type: RowType,
    prev_pos: f32,
    elapsed: Duration,
) {
    if let Some(x) = config.coord_to_pos_x((1280. - prev_pos * 1280.) as i32) {
        if let Some(plant) = plants[x].as_mut() {
            zombie.set_pos(prev_pos);
            let diff = elapsed.as_secs_f32() * if zombie.freezed() { 0.5 } else { 1. };
            if plant.health().as_secs_f32() < diff {
                plants[x] = if row_type == RowType::Water && !plant.is_nenuphar() {
                    Some(Box::new(Nenuphar::new()))
                } else {
                    None
                }
            } else {
                *plant.health() -= Duration::from_secs_f32(diff);
            }
        }
    } else if let Some(x) = config.coord_to_pos_x((1280. - zombie.pos() * 1280.) as i32) {
        if let Some(plant) = plants[x].as_ref() {
            let pos = (config.pos_to_coord_x(x) as f32 + plant.width() as f32 - 1280.) / -1280.;
            if zombie.pos() > pos {
                zombie.set_pos(pos);
            }
        }
    }
}

pub(super) fn do_damage_to_zombies(
    row: &mut [Box<dyn Zombie>],
    proj_x: i32,
    proj_width: i32,
    damage_amount: usize,
    damage_type: DamageType,
) -> (bool, Vec<usize>) {
    let mut zombies = row
        .iter_mut()
        .enumerate()
        .filter_map(|(i, zombie)| {
            let zx = 1280 - (zombie.pos() * 1280.).floor() as i32 + zombie.hit_box().0 as i32;
            if zx + zombie.hit_box().1 as i32 >= proj_x && zx <= proj_x + proj_width {
                Some((i, zombie.pos()))
            } else {
                None
            }
        })
        .collect::<Vec<(usize, f32)>>();
    zombies.sort_by(|(_, pos1), (_, pos2)| pos2.total_cmp(pos1));
    if let Some(&(zombie_index, _)) = zombies.first() {
        (
            true,
            hit_zombie(row, zombie_index, damage_amount, damage_type, false),
        )
    } else {
        (false, Vec::new())
    }
}

pub(super) fn hit_zombie(
    row: &mut [Box<dyn Zombie>],
    zombie_index: usize,
    damage_amount: usize,
    damage_type: DamageType,
    propagated: bool,
) -> Vec<usize> {
    let hit = row[zombie_index].hit(damage_amount, damage_type, propagated);
    let mut to_remove = Vec::new();
    if hit.0 {
        to_remove.push(zombie_index)
    }
    if hit.1 && !propagated {
        to_remove.extend(propagate(row, zombie_index, damage_amount, damage_type));
    }
    to_remove
}

pub(super) fn propagate(
    row: &mut [Box<dyn Zombie>],
    zombie_index: usize,
    damage_amount: usize,
    damage_type: DamageType,
) -> Vec<usize> {
    let size = {
        let oz = row[zombie_index].as_ref();
        let zx = 1280 - (oz.pos() * 1280.).floor() as i32 + oz.hit_box().0 as i32;
        (zx, zx + oz.hit_box().1 as i32)
    };
    let mut to_remove = Vec::new();
    for zombie_index2 in row
        .iter_mut()
        .enumerate()
        .filter_map(|(i, zombie)| {
            let zx = 1280 - (zombie.pos() * 1280.).floor() as i32 + zombie.hit_box().0 as i32;
            if zx + zombie.hit_box().1 as i32 >= size.0 && zx <= size.1 {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>()
    {
        if zombie_index != zombie_index2 {
            to_remove.extend(hit_zombie(
                row,
                zombie_index2,
                damage_amount,
                damage_type,
                true,
            ));
        }
    }
    to_remove
}
