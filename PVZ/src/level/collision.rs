use std::time::Duration;

use sdl2::rect::FRect;

use crate::{
    plants::{nenuphar::Nenuphar, Plant},
    projectile::{DamageType, Projectile},
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
                        self.config.pos_to_coord_x(x) + plant.rect(0., 0.).width() / 2.,
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
    prev_x: f32,
    elapsed: Duration,
) {
    if let Some(x) = config.coord_to_pos_x(prev_x) {
        if let Some(plant) = plants[x].as_mut() {
            zombie.set_x(prev_x);
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
    } else if let Some(x) = config.coord_to_pos_x(zombie.rect(0.).left()) {
        if let Some(plant) = plants[x].as_ref() {
            let rect = plant.rect(config.pos_to_coord_x(x), 0.);
            if zombie.rect(0.).has_intersection(rect) {
                zombie.set_x(rect.x() + rect.width());
            }
        }
    }
}

pub(super) fn do_damage_to_zombies(
    row: &mut [Box<dyn Zombie>],
    proj: &dyn Projectile,
) -> (bool, Vec<usize>) {
    let mut zombies = row
        .iter_mut()
        .enumerate()
        .filter_map(|(i, zombie)| {
            let hit_box = zombie.hit_box(0.);
            if hit_box.has_intersection(proj.rect(0.)) {
                Some((i, hit_box))
            } else {
                None
            }
        })
        .collect::<Vec<(usize, FRect)>>();
    zombies.sort_by(|(_, pos1), (_, pos2)| pos1.left().total_cmp(&pos2.left()));
    if let Some(&(zombie_index, _)) = zombies.first() {
        (
            true,
            hit_zombie(
                row,
                zombie_index,
                proj.damage_amount(),
                proj.damage_type(),
                false,
            ),
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
        oz.hit_box(0.)
    };
    let mut to_remove = Vec::new();
    for zombie_index2 in row
        .iter_mut()
        .enumerate()
        .filter_map(|(i, zombie)| {
            if zombie.hit_box(0.).has_intersection(size) {
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
