use std::time::Duration;

use pvz::textures::textures;
use sdl::{
    event::Event,
    functions::StateEnum,
    grid::{ColType, Grid, GridChildren, Pos, RowType},
    missing::ui_string::UIString,
    ui_rect::UIRect,
    user_control::UserControl,
};
use sdl2::{pixels::Color, rect::FRect, render::Canvas, video::Window};

use crate::win::Win;

pub struct RowsEditor {
    rows_types: *mut Vec<pvz::level::config::RowType>,
    lens: u8,
    grid: Grid<RowsEditor>,
}

impl RowsEditor {
    pub fn new(rows_types: *mut Vec<pvz::level::config::RowType>) -> Self {
        Self {
            rows_types,
            lens: 0,
            grid: unsafe { Grid::empty() },
        }
    }

    fn rows(&self) -> &[pvz::level::config::RowType] {
        unsafe { self.rows_types.as_ref().expect("unwrap ptr") }.as_ref()
    }

    fn rows_mut(&mut self) -> &mut Vec<pvz::level::config::RowType> {
        unsafe { self.rows_types.as_mut().expect("unwrap ptr") }
    }

    pub fn reset_grid(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if self.lens == self.rows().len() as u8 {
            return Ok(());
        }
        let font = &textures()?.font;
        self.lens = self.rows().len() as u8;
        let rows = (0..=self.lens).map(|_| RowType::Ratio(1.));
        let rows_element = (0..self.lens).flat_map(|i| {
            [
                (
                    Pos {
                        x: 0,
                        y: i as usize,
                    },
                    Box::new(
                        UIRect::new(
                            Box::new(|_, _| StateEnum::Enable),
                            Box::new(move |_self: &RowsEditor, _| {
                                if let Some(pvz::level::config::RowType::Grass) =
                                    _self.rows().get(i as usize)
                                {
                                    Color::GREEN
                                } else {
                                    Color::RED
                                }
                            }),
                        )
                        .action(Box::new(move |_self: &mut RowsEditor, _, _, _, _| {
                            _self.rows_mut()[i as usize] = pvz::level::config::RowType::Grass;
                            Ok(())
                        }))
                        .text(Box::new(|_, _| {
                            Ok((UIString::new(font, "Grass".to_owned())?, Color::WHITE))
                        })),
                    ) as Box<dyn GridChildren<RowsEditor>>,
                ),
                (
                    Pos {
                        x: 1,
                        y: i as usize,
                    },
                    Box::new(
                        UIRect::new(
                            Box::new(|_, _| StateEnum::Enable),
                            Box::new(move |_self: &RowsEditor, _| {
                                if let Some(pvz::level::config::RowType::Water) =
                                    _self.rows().get(i as usize)
                                {
                                    Color::GREEN
                                } else {
                                    Color::RED
                                }
                            }),
                        )
                        .action(Box::new(move |_self: &mut RowsEditor, _, _, _, _| {
                            _self.rows_mut()[i as usize] = pvz::level::config::RowType::Water;
                            Ok(())
                        }))
                        .text(Box::new(|_, _| {
                            Ok((UIString::new(font, "Water".to_owned())?, Color::WHITE))
                        })),
                    ) as Box<dyn GridChildren<RowsEditor>>,
                ),
            ]
        });
        self.grid = Grid::new(
            self,
            vec![ColType::Ratio(1.), ColType::Ratio(1.)],
            rows.collect(),
            rows_element.collect(),
        );
        self.grid.init(canvas)
    }
}

impl GridChildren<Win> for RowsEditor {
    fn grid_init(&mut self, _: &mut Canvas<Window>, _: &mut Win) -> Result<(), String> {
        Ok(())
    }

    fn grid_init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        surface: FRect,
        _: &mut Win,
    ) -> Result<(), String> {
        self.reset_grid(canvas)?;
        self.grid.init_frame(canvas, surface)
    }

    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        _: &mut Win,
    ) -> Result<(), String> {
        self.grid.event(canvas, event)
    }

    fn grid_update(
        &mut self,
        canvas: &mut Canvas<Window>,
        elapsed: Duration,
        _: &mut Win,
    ) -> Result<(), String> {
        self.grid.update(canvas, elapsed)
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &Win) -> Result<(), String> {
        self.grid.draw(canvas)
    }
}
