use std::time::Duration;

use anyhow::Result;
use red_sdl::{
    event::Event,
    functions::StateEnum,
    missing::ui_string::UIString,
    refs::{MutRef, Ref},
    ui_element::{
        grid::{ColType, Grid, Pos, RowType},
        ui_rect::UIRect,
    },
    user_control::UserControl,
};
use red_sdl_macro::UserControl;
use sdl2::{pixels::Color, rect::FRect, render::Canvas, video::Window};

use crate::{map_config::MapConfig, State};

#[derive(UserControl)]
#[parent(MapConfig)]
#[state(State)]
pub struct RowsEditor {
    #[child]
    grid: Grid<MapConfig, State, UIRect<MapConfig, State>>,
}

impl RowsEditor {
    pub fn new(rows: u8, state: Ref<State>) -> Result<Self> {
        let font = state.as_ref().textures().font();
        Ok(Self {
            grid: Grid::new(
                vec![ColType::Ratio(1.), ColType::Ratio(1.)],
                (0..=rows).map(|_| RowType::Ratio(1.)).collect(),
                (0..rows)
                    .flat_map(|i| {
                        [
                            (
                                Pos {
                                    x: 0,
                                    y: i as usize,
                                },
                                UIRect::new(
                                    Box::new(|_, _, _| StateEnum::Enable),
                                    Box::new(move |_, _self: Ref<MapConfig>, _| {
                                        if let Some(pvz::level::config::RowType::Grass) =
                                            _self.map.rows.get(i as usize)
                                        {
                                            Color::GREEN
                                        } else {
                                            Color::RED
                                        }
                                    }),
                                )
                                .action(Box::new(move |_, mut _self: MutRef<MapConfig>, _, _| {
                                    _self.map.rows[i as usize] = pvz::level::config::RowType::Grass;
                                    Ok(())
                                }))
                                .text(Box::new(|_, _, _| {
                                    Ok((UIString::new(font, "Grass".to_owned())?, Color::WHITE))
                                })),
                            ),
                            (
                                Pos {
                                    x: 1,
                                    y: i as usize,
                                },
                                UIRect::new(
                                    Box::new(|_, _, _| StateEnum::Enable),
                                    Box::new(move |_, _self: Ref<MapConfig>, _| {
                                        if let Some(pvz::level::config::RowType::Water) =
                                            _self.map.rows.get(i as usize)
                                        {
                                            Color::GREEN
                                        } else {
                                            Color::RED
                                        }
                                    }),
                                )
                                .action(Box::new(move |_, mut _self: MutRef<MapConfig>, _, _| {
                                    _self.map.rows[i as usize] = pvz::level::config::RowType::Water;
                                    Ok(())
                                }))
                                .text(Box::new(|_, _, _| {
                                    Ok((UIString::new(font, "Water".to_owned())?, Color::WHITE))
                                })),
                            ),
                        ]
                    })
                    .collect(),
            ),
        })
    }
}
