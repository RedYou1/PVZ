use std::{collections::HashMap, ops::Range, time::Duration};

use anyhow::{anyhow, Error, Result};
use pvz::{
    default_button,
    level::{config::Map, Level},
    zombie::{valide_zombie_id, zombie_from_id},
};
use red_sdl::{
    event::Event,
    functions::StateEnum,
    missing::ui_string::UIString,
    refs::{MutRef, Ref},
    simple_grid,
    ui_element::{
        grid::{ColType, Grid, Pos, RowType},
        scroll_view::ScrollView,
        text_box::TextBox,
        ui_rect::UIRect,
    },
    user_control::UserControl,
};
use red_sdl_macro::UserControl;
use sdl2::{
    pixels::Color,
    rect::FRect,
    render::{Canvas, Texture},
    ttf::Font,
    video::Window,
};

use crate::{win::Win, State};

const SCROLL_ELEMENT_SIZE: f32 = 80.;

pub struct LevelConfig {
    pub level: Level,
    save_ok: bool,
    grid: Grid<LevelConfig, State, LevelConfigElement>,
}

#[derive(UserControl)]
#[parent(LevelConfig)]
#[state(State)]
pub enum LevelConfigElement {
    UIRect(UIRect<LevelConfig, State>),
    Header(Grid<LevelConfig, State, LevelSubElement>),
    Sroll(ScrollView<LevelConfig, State, Grid<LevelConfig, State, LevelSubElement>>),
}

#[derive(UserControl)]
#[parent(LevelConfig)]
#[state(State)]
pub enum LevelSubElement {
    Label(UIRect<LevelConfig, State>),
    TextBox(TextBox<LevelConfig, State>),
}

impl LevelConfig {
    #[allow(clippy::too_many_lines)]
    pub fn new(
        id: u8,
        surface: FRect,
        state: MutRef<State>,
        canvas: &Canvas<Window>,
    ) -> Result<Self> {
        let font = state.as_ref().textures().font();
        let level = Level::load(id)?;
        let map_id = level.map.id;
        let money = level.money;
        let mut elements = HashMap::new();
        let mut index_element = 0;
        for (time, zombies) in level.spawn_waits.iter().enumerate().map(|(i, wait)| {
            let mut zombies: HashMap<u8, u32> = HashMap::new();
            for z in level.spawn_zombies[i].iter().map(|z| z.0) {
                if let Some(v) = zombies.get_mut(&z) {
                    *v += 1;
                } else {
                    zombies.insert(z, 1);
                }
            }
            let mut zombies: Vec<(u8, u32)> = zombies.into_iter().collect();
            zombies.sort_by(|z1, z2| (z1.0 - z2.0).cmp(&z1.0));
            (wait, zombies)
        }) {
            zombie_time(font, &mut elements, index_element, time.as_secs())?;
            for (_type, amount) in zombies.into_iter() {
                zombie_row(font, &mut elements, index_element, _type, amount)?;
                index_element += 1;
            }
            wave_buttons(font, &mut elements, index_element);
            index_element += 2;
        }
        if index_element == 0 {
            elements.insert(
                Pos {
                    x: 0,
                    y: index_element,
                },
                default_button()
                    .text(Box::new(|_, _, _| {
                        Ok((UIString::new(font, "+ Wave".to_owned())?, Color::WHITE))
                    }))
                    .action(Box::new(add_wave_action(0)))
                    .into(),
            );
            index_element = 1;
        }

        let mut s = Self {
            level,
            save_ok: false,
            grid: simple_grid!(
                ColType::Ratio(150.),
                ColType::Ratio(980.),
                ColType::Ratio(150.);
                RowType::Ratio(100.),
                RowType::Ratio(620.),
                RowType::Ratio(100.);
                Pos{x:0,y:0} => UIRect::new(Box::new(|_, _, _| StateEnum::Enable),Box::new(|_, _self: Ref<LevelConfig>, _| if _self.save_ok { Color::BLACK } else {Color::RED})).action(Box::new(|a,mut _self,state,canvas|{
                        _self.as_mut().save_ok = _self.try_save().is_ok();
                        if _self.save_ok {
                            _self.level.save_config()?;
                            State::_return(a,_self,state,canvas)?;
                        }
                        Ok(())
                    })).text(Box::new(|_, _, state| Ok((Some(state.as_ref().texts()._return.clone()), Color::WHITE)))).into(),
                Pos{x:1,y:0} => simple_grid!(
                    ColType::Ratio(1.),
                    ColType::Ratio(1.),
                    ColType::Ratio(1.);
                    RowType::Ratio(1.);
                    Pos{x:0,y:0} => TextBox::new(
                        font,
                        UIString::new(font, money.to_string())?.ok_or(anyhow!("money too large"))?,
                        Box::new(|_, _, _| StateEnum::Enable),
                        Box::new(|_, _, _| Color::RGBA(255, 255, 255, 100)),
                        Box::new(|_, _, _| Color::WHITE),
                        Box::new(|_, _, _| Color::WHITE),
                        Box::new(|t, _, _| {
                            if t.text().as_str().parse::<u32>().is_ok() {
                                Color::BLACK
                            } else {
                                Color::RED
                            }
                        }),
                    ).into(),
                    Pos{x:1,y:0} => TextBox::new(
                        font,
                        UIString::new(font, map_id.to_string())?.ok_or(anyhow!("map id too large"))?,
                        Box::new(|_, _, _| StateEnum::Enable),
                        Box::new(|_, _, _| Color::RGBA(255, 255, 255, 100)),
                        Box::new(|_, _, _| Color::WHITE),
                        Box::new(|_, _, _| Color::WHITE),
                        Box::new(|t, _self: Ref<LevelConfig>, state: Ref<State>| {
                            if let Ok(map) = t.text().as_str().parse::<u8>() {
                                if map < state.as_ref().maps_count{
                                    return Color::BLACK;
                                }
                            }
                            Color::RED
                        }),
                    ).into(),
                    Pos{x:2,y:0} => UIRect::new(
                        Box::new(|_, _self: Ref<LevelConfig>, state: Ref<State>| {
                            if let Ok(map) = _self.get_map_text().parse::<u8>() {
                                if map < state.as_ref().maps_count {
                                    return StateEnum::Enable;
                                }
                            }
                            StateEnum::Hidden
                        }),
                        Box::new(|_, _, _| Color::BLACK),
                    )
                    .image(Box::new(|_, _self: Ref<LevelConfig>, state: Ref<State>| {
                        if let Ok(map) = _self.get_map_text().parse::<u8>() {
                            if map < state.as_ref().maps_count {
                                return Ok(state.as_ref().textures().map(map as usize));
                            }
                        }
                        Err(anyhow!("Not supposed showing"))
                    })).into()
                ).into(),
                Pos{x:1,y:1} => ScrollView::new(
                    Grid::new(
                        vec![
                            ColType::Ratio(100.),
                            ColType::Ratio(10.),
                            ColType::Ratio(100.),
                            ColType::Ratio(10.),
                            ColType::Ratio(100.),
                            ColType::Ratio(10.),
                            ColType::Ratio(100.),
                        ],
                        (0..index_element).map(|_| RowType::Ratio(1.)).collect(),
                        elements,
                    ),
                    1000., SCROLL_ELEMENT_SIZE * index_element as f32,
                    Box::new(|_, _, _| Color::RGBA(255, 255, 255, 100)),
                ).into(),
            ),
        };
        UserControl::event(
            (&mut s.grid).into(),
            canvas,
            Event::ElementMove {
                x: surface.x(),
                y: surface.y(),
            },
            (&mut s).into(),
            state,
        )?;
        UserControl::event(
            (&mut s.grid).into(),
            canvas,
            Event::ElementResize {
                width: surface.width(),
                height: surface.height(),
            },
            (&mut s).into(),
            state,
        )?;
        s.save_ok = s.try_save().is_ok();
        Ok(s)
    }

    fn get_header(&self) -> &Grid<LevelConfig, State, LevelSubElement> {
        match self.grid.get_element(1, 0).expect("level header not there") {
            LevelConfigElement::Header(header) => header,
            _ => panic!("level header not there"),
        }
    }

    fn get_money_text(&self) -> &str {
        match self
            .get_header()
            .get_element(0, 0)
            .expect("level money not there")
        {
            LevelSubElement::TextBox(t) => t.text().as_str(),
            _ => panic!("level money not there"),
        }
    }

    fn get_map_text(&self) -> &str {
        match self
            .get_header()
            .get_element(1, 0)
            .expect("level map not there")
        {
            LevelSubElement::TextBox(t) => t.text().as_str(),
            _ => panic!("level map not there"),
        }
    }

    fn get_level_config(&self) -> &Grid<LevelConfig, State, LevelSubElement> {
        match self.grid.get_element(1, 1).expect("level_config not there") {
            LevelConfigElement::Sroll(scroll_view) => scroll_view.child(),
            _ => panic!("level_config not there"),
        }
    }

    fn get_level_config_scroll_mut(
        &mut self,
    ) -> &mut ScrollView<LevelConfig, State, Grid<LevelConfig, State, LevelSubElement>> {
        match self
            .grid
            .get_element_mut(1, 1)
            .expect("level_config not there")
        {
            LevelConfigElement::Sroll(scroll_view) => scroll_view,
            _ => panic!("level_config not there"),
        }
    }

    fn get_level_config_mut(&mut self) -> &mut Grid<LevelConfig, State, LevelSubElement> {
        match self
            .grid
            .get_element_mut(1, 1)
            .expect("level_config not there")
        {
            LevelConfigElement::Sroll(scroll_view) => scroll_view.child_mut(),
            _ => panic!("level_config not there"),
        }
    }

    fn waves_indexes(&self) -> impl Iterator<Item = usize> + use<'_> {
        let grid = self.get_level_config();
        (0..grid.rows().len())
            .filter(|&i| matches!(grid.get_element(0, i), Some(LevelSubElement::TextBox(_))))
    }

    fn waves(&self) -> impl Iterator<Item = Range<usize>> + use<'_> {
        let grid = self.get_level_config();
        (0..grid.rows().len()).filter_map(|i1| {
            if let Some(LevelSubElement::TextBox(_)) = grid.get_element(0, i1) {
                let mut i2 = i1 + 1;
                while let Some(LevelSubElement::Label(_)) = grid.get_element(6, i2) {
                    i2 += 1;
                }
                Some(i1..i2)
            } else {
                None
            }
        })
    }

    pub fn try_save(&mut self) -> Result<()> {
        self.level.money = self
            .get_money_text()
            .parse::<u32>()
            .map_err(|e| anyhow!(e))?;
        self.level.map = Map::load(self.get_map_text().parse::<u8>().map_err(|e| anyhow!(e))?)
            .map_err(|e| anyhow!(e))?;

        self.level.spawn_waits = self
            .waves_indexes()
            .map(|i| {
                if let Some(LevelSubElement::TextBox(t)) = self.get_level_config().get_element(0, i)
                {
                    Ok(Duration::from_secs(
                        t.text().as_str().parse::<u8>().map_err(|e| anyhow!(e))? as u64,
                    ))
                } else {
                    Err(anyhow!("wrongly placed wave time"))
                }
            })
            .collect::<Result<Vec<Duration>, Error>>()?;
        self.level.spawn_zombies = self
            .waves()
            .map(|i| {
                Ok(i.map(|zi| {
                    let Some(LevelSubElement::TextBox(zombie)) =
                        self.get_level_config().get_element(2, zi)
                    else {
                        return Err(anyhow!("wrongly placed zombie id"));
                    };
                    let Some(LevelSubElement::TextBox(amount)) =
                        self.get_level_config().get_element(4, zi)
                    else {
                        return Err(anyhow!("wrongly placed zombie amount"));
                    };

                    let zombie = zombie
                        .text()
                        .as_str()
                        .parse::<u8>()
                        .map_err(|e| anyhow!(e))?;
                    if !valide_zombie_id(zombie) {
                        return Err(anyhow!("invalide zombie id"));
                    }
                    let amount = amount
                        .text()
                        .as_str()
                        .parse::<u8>()
                        .map_err(|e| anyhow!(e))?;
                    if amount == 0 {
                        return Err(anyhow!("Amount too low"));
                    }
                    Ok::<Vec<(u8, f32, f32)>, Error>(
                        (0..amount).map(move |_| (zombie, 0., 0.)).collect(),
                    )
                })
                .collect::<Result<Vec<Vec<(u8, f32, f32)>>, Error>>()?
                .into_iter()
                .flatten()
                .collect())
            })
            .collect::<Result<Vec<Vec<(u8, f32, f32)>>, Error>>()?;
        Ok(())
    }

    fn insert_row(
        &self,
        from: usize,
        to: usize,
        grid: &mut HashMap<Pos, LevelSubElement>,
        state: Ref<State>,
    ) {
        for y in (from..to).rev() {
            for x in (0..=6).step_by(2) {
                if let Some(mut v) = grid.remove(&Pos { x, y }) {
                    self.update_index((&mut v).into(), x, y + 1, state);
                    grid.insert(Pos { x, y: y + 1 }, v);
                }
            }
        }
    }

    fn remove_row(
        &self,
        row: usize,
        to: usize,
        grid: &mut HashMap<Pos, LevelSubElement>,
        state: Ref<State>,
    ) {
        for x in (0..=6).step_by(2) {
            grid.remove(&Pos { x, y: row });
        }
        for y in (row + 1)..to {
            for x in (0..=6).step_by(2) {
                if let Some(mut v) = grid.remove(&Pos { x, y }) {
                    self.update_index((&mut v).into(), x, y - 1, state);
                    grid.insert(Pos { x, y: y - 1 }, v);
                }
            }
        }
    }

    fn update_index(
        &self,
        mut element: MutRef<LevelSubElement>,
        col: usize,
        row: usize,
        state: Ref<State>,
    ) {
        match element.as_mut() {
            LevelSubElement::TextBox(_) => match col {
                0 => {}
                2 => {}
                4 => {}
                _ => panic!("level_config::update_index not supported"),
            },
            LevelSubElement::Label(uirect) => match (
                col,
                UIRect::get_text(uirect.into(), self.into(), state)
                    .as_ref()
                    .map(|t| t.as_ref().map(|t| t.as_str())),
            ) {
                (6, _) => {
                    *uirect.state_mut() = Box::new(zombie_image_state(row));
                    let image = zombie_image_image(row);
                    *uirect.back_draw_mut() = Some(Box::new(move |this, canvas, parent, state| {
                        canvas
                            .copy_f(image(this, parent, state)?, None, this.surface())
                            .map_err(|e| anyhow!(e))
                    }));
                }
                (0, Ok(Some("+ Zombie"))) => {
                    *uirect.action_mut() = Some(Box::new(add_zombie_action(row)));
                }
                (0, Ok(Some("+ Wave"))) => {
                    *uirect.action_mut() = Some(Box::new(add_wave_action(row)));
                }
                (2, Ok(Some("- Zombie"))) => {
                    *uirect.action_mut() = Some(Box::new(remove_zombie_action(row)));
                }
                (2, Ok(Some("- Wave"))) => {
                    *uirect.action_mut() = Some(Box::new(remove_wave_action(row)));
                }
                _ => panic!("level_config::update_index not supported"),
            },
        }
    }
}

impl UserControl<Win, State> for LevelConfig {
    fn surface(this: Ref<Self>, _: Ref<Win>, state: Ref<State>) -> FRect {
        UserControl::surface((&this.grid).into(), this, state)
    }

    fn event(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        event: Event,
        _: MutRef<Win>,
        state: MutRef<State>,
    ) -> Result<()> {
        UserControl::event((&mut this.as_mut().grid).into(), canvas, event, this, state)
    }

    fn update(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        elapsed: Duration,
        _: MutRef<Win>,
        state: MutRef<State>,
    ) -> Result<()> {
        this.save_ok = this.try_save().is_ok();
        UserControl::update(
            (&mut this.as_mut().grid).into(),
            canvas,
            elapsed,
            this,
            state,
        )
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        _: Ref<Win>,
        state: Ref<State>,
    ) -> Result<()> {
        UserControl::draw((&this.grid).into(), canvas, this, state)
    }
}

fn wave_buttons(
    font: &'static Font<'_, '_>,
    elements: &mut HashMap<Pos, LevelSubElement>,
    index_element: usize,
) {
    elements.insert(
        Pos {
            x: 0,
            y: index_element,
        },
        default_button()
            .text(Box::new(|_, _, _| {
                Ok((UIString::new(font, "+ Zombie".to_owned())?, Color::WHITE))
            }))
            .action(Box::new(add_zombie_action(index_element)))
            .into(),
    );
    elements.insert(
        Pos {
            x: 2,
            y: index_element,
        },
        default_button()
            .text(Box::new(|_, _, _| {
                Ok((UIString::new(font, "- Zombie".to_owned())?, Color::WHITE))
            }))
            .action(Box::new(remove_zombie_action(index_element)))
            .into(),
    );
    elements.insert(
        Pos {
            x: 0,
            y: index_element + 1,
        },
        default_button()
            .text(Box::new(|_, _, _| {
                Ok((UIString::new(font, "+ Wave".to_owned())?, Color::WHITE))
            }))
            .action(Box::new(add_wave_action(index_element + 1)))
            .into(),
    );
    elements.insert(
        Pos {
            x: 2,
            y: index_element + 1,
        },
        default_button()
            .text(Box::new(|_, _, _| {
                Ok((UIString::new(font, "- Wave".to_owned())?, Color::WHITE))
            }))
            .action(Box::new(remove_wave_action(index_element + 1)))
            .into(),
    );
}

fn zombie_time(
    font: &'static Font<'_, '_>,
    elements: &mut HashMap<Pos, LevelSubElement>,
    index_element: usize,
    time_secs: u64,
) -> Result<(), Error> {
    elements.insert(
        Pos {
            x: 0,
            y: index_element,
        },
        TextBox::new(
            font,
            UIString::new(font, time_secs.to_string())?.ok_or(anyhow!("sized"))?,
            Box::new(|_, _, _| StateEnum::Enable),
            Box::new(|_, _, _| Color::RGBA(255, 255, 255, 100)),
            Box::new(|_, _, _| Color::WHITE),
            Box::new(|_, _, _| Color::WHITE),
            Box::new(|t, _, _| {
                if t.text().as_str().parse::<u8>().is_ok() {
                    Color::BLACK
                } else {
                    Color::RED
                }
            }),
        )
        .into(),
    );
    Ok(())
}

fn zombie_row(
    font: &'static Font<'_, '_>,
    elements: &mut HashMap<Pos, LevelSubElement>,
    index_element: usize,
    _type: u8,
    amount: u32,
) -> Result<(), Error> {
    elements.insert(
        Pos {
            x: 2,
            y: index_element,
        },
        edit_zombie_id(font, _type)?,
    );
    elements.insert(
        Pos {
            x: 4,
            y: index_element,
        },
        edit_zombie_amount(font, amount)?,
    );
    elements.insert(
        Pos {
            x: 6,
            y: index_element,
        },
        UIRect::new(
            Box::new(zombie_image_state(index_element)),
            Box::new(|_, _, _| Color::BLACK),
        )
        .image(Box::new(zombie_image_image(index_element)))
        .into(),
    );
    Ok(())
}

fn edit_zombie_id(
    font: &'static Font<'static, 'static>,
    _type: u8,
) -> Result<LevelSubElement, Error> {
    Ok(TextBox::new(
        font,
        UIString::new(font, _type.to_string())?.ok_or(anyhow!("sized"))?,
        Box::new(|_, _, _| StateEnum::Enable),
        Box::new(|_, _, _| Color::RGBA(255, 255, 255, 100)),
        Box::new(|_, _, _| Color::WHITE),
        Box::new(|_, _, _| Color::WHITE),
        Box::new(|t, _, _| {
            if let Ok(id) = t.text().as_str().parse::<u8>() {
                if valide_zombie_id(id) {
                    return Color::BLACK;
                }
            }
            Color::RED
        }),
    )
    .into())
}

fn edit_zombie_amount(
    font: &'static Font<'static, 'static>,
    amount: u32,
) -> Result<LevelSubElement, Error> {
    Ok(TextBox::new(
        font,
        UIString::new(font, amount.to_string())?.ok_or(anyhow!("sized"))?,
        Box::new(|_, _, _| StateEnum::Enable),
        Box::new(|_, _, _| Color::RGBA(255, 255, 255, 100)),
        Box::new(|_, _, _| Color::WHITE),
        Box::new(|_, _, _| Color::WHITE),
        Box::new(|t, _, _| {
            if let Ok(t) = t.text().as_str().parse::<u8>() {
                if t != 0 {
                    Color::BLACK
                } else {
                    Color::RED
                }
            } else {
                Color::RED
            }
        }),
    )
    .into())
}

#[allow(clippy::type_complexity)]
fn add_wave_action(
    mut index_element: usize,
) -> impl FnMut(
    MutRef<UIRect<LevelConfig, State>>,
    MutRef<LevelConfig>,
    MutRef<State>,
    &Canvas<Window>,
) -> std::result::Result<(), Error> {
    move |_, mut _self: MutRef<LevelConfig>, state, _| {
        _self
            .as_mut()
            .get_level_config_mut()
            .state_manager
            .add(Box::new(move |(mut _self, _, mut rows, mut elements)| {
                let child_size = &mut _self
                    .as_mut()
                    .get_level_config_scroll_mut()
                    .child_size_mut()
                    .1;
                if index_element == 0 {
                    elements.clear();
                    rows.clear();
                    *child_size = 0.;
                } else {
                    if index_element + 1 < rows.len() {
                        _self.insert_row(
                            index_element + 1,
                            rows.len(),
                            elements.as_mut(),
                            state.into(),
                        );
                        _self.insert_row(
                            index_element + 2,
                            rows.len() + 1,
                            elements.as_mut(),
                            state.into(),
                        );
                        _self.insert_row(
                            index_element + 3,
                            rows.len() + 2,
                            elements.as_mut(),
                            state.into(),
                        );
                    }
                    index_element += 1;
                };
                let f = rows.first().map_or(1., |a| a.to_px(1.));
                rows.push(RowType::Ratio(f));
                rows.push(RowType::Ratio(f));
                rows.push(RowType::Ratio(f));
                *child_size += SCROLL_ELEMENT_SIZE * 3.;
                let font = state.as_ref().textures().font();
                zombie_time(font, &mut elements, index_element, 0)?;
                zombie_row(font, &mut elements, index_element, 255, 1)?;
                wave_buttons(font, &mut elements, index_element + 1);
                Ok(())
            }));
        Ok(())
    }
}

#[allow(clippy::type_complexity)]
fn remove_wave_action(
    index_element: usize,
) -> impl FnMut(
    MutRef<UIRect<LevelConfig, State>>,
    MutRef<LevelConfig>,
    MutRef<State>,
    &Canvas<Window>,
) -> std::result::Result<(), Error> {
    move |_, mut _self: MutRef<LevelConfig>, state, _| {
        _self
            .as_mut()
            .get_level_config_mut()
            .state_manager
            .add(Box::new(move |(mut _self, _, mut rows, mut elements)| {
                for i in (0..=index_element).rev() {
                    let last = matches!(
                        elements.get(&Pos { x: 0, y: i }),
                        Some(LevelSubElement::TextBox(_))
                    );
                    _self.remove_row(i, rows.len(), elements.as_mut(), state.into());
                    rows.pop();
                    _self.get_level_config_scroll_mut().child_size_mut().1 -= SCROLL_ELEMENT_SIZE;
                    if last {
                        break;
                    }
                }

                if rows.is_empty() {
                    elements.insert(
                        Pos { x: 0, y: 0 },
                        default_button()
                            .text(Box::new(|_, _, state: Ref<State>| {
                                Ok((
                                    UIString::new(
                                        state.as_ref().textures().font(),
                                        "+ Wave".to_owned(),
                                    )?,
                                    Color::WHITE,
                                ))
                            }))
                            .action(Box::new(add_wave_action(0)))
                            .into(),
                    );
                    let f = rows.first().map_or(1., |a| a.to_px(1.));
                    rows.push(RowType::Ratio(f));
                    _self.get_level_config_scroll_mut().child_size_mut().1 += SCROLL_ELEMENT_SIZE;
                }
                Ok(())
            }));
        Ok(())
    }
}

#[allow(clippy::type_complexity)]
fn add_zombie_action(
    index_element: usize,
) -> impl FnMut(
    MutRef<UIRect<LevelConfig, State>>,
    MutRef<LevelConfig>,
    MutRef<State>,
    &Canvas<Window>,
) -> Result<()> {
    move |_, mut _self: MutRef<LevelConfig>, state, _| {
        _self
            .as_mut()
            .get_level_config_mut()
            .state_manager
            .add(Box::new(move |(mut _self, _, mut rows, mut elements)| {
                _self.insert_row(index_element, rows.len(), elements.as_mut(), state.into());
                zombie_row(
                    state.as_ref().textures().font(),
                    &mut elements,
                    index_element,
                    255,
                    1,
                )?;
                let f = rows.first().map_or(1., |a| a.to_px(1.));
                rows.push(RowType::Ratio(f));

                _self
                    .as_mut()
                    .get_level_config_scroll_mut()
                    .child_size_mut()
                    .1 += SCROLL_ELEMENT_SIZE;
                Ok(())
            }));
        Ok(())
    }
}

#[allow(clippy::type_complexity)]
fn remove_zombie_action(
    index_element: usize,
) -> impl FnMut(
    MutRef<UIRect<LevelConfig, State>>,
    MutRef<LevelConfig>,
    MutRef<State>,
    &Canvas<Window>,
) -> std::result::Result<(), Error> {
    move |_, mut _self: MutRef<LevelConfig>, state, _| {
        _self
            .as_mut()
            .get_level_config_mut()
            .state_manager
            .add(Box::new(move |(mut _self, _, mut rows, mut elements)| {
                _self.remove_row(
                    index_element - 1,
                    rows.len(),
                    elements.as_mut(),
                    state.into(),
                );
                rows.pop();
                _self.get_level_config_scroll_mut().child_size_mut().1 -= SCROLL_ELEMENT_SIZE;
                if index_element == 1
                    || matches!(
                        elements.get(&Pos {
                            x: 0,
                            y: index_element - 2,
                        }),
                        Some(LevelSubElement::Label(_))
                    )
                {
                    _self.remove_row(
                        index_element - 1,
                        rows.len(),
                        elements.as_mut(),
                        state.into(),
                    );
                    rows.pop();
                    _self.remove_row(
                        index_element - 1,
                        rows.len(),
                        elements.as_mut(),
                        state.into(),
                    );
                    rows.pop();
                    _self.get_level_config_scroll_mut().child_size_mut().1 -=
                        SCROLL_ELEMENT_SIZE * 2.;
                    if rows.is_empty() {
                        elements.insert(
                            Pos { x: 0, y: 0 },
                            default_button()
                                .text(Box::new(|_, _, state: Ref<State>| {
                                    Ok((
                                        UIString::new(
                                            state.as_ref().textures().font(),
                                            "+ Wave".to_owned(),
                                        )?,
                                        Color::WHITE,
                                    ))
                                }))
                                .action(Box::new(add_wave_action(0)))
                                .into(),
                        );
                        let f = rows.first().map_or(1., |a| a.to_px(1.));
                        rows.push(RowType::Ratio(f));
                        _self.get_level_config_scroll_mut().child_size_mut().1 +=
                            SCROLL_ELEMENT_SIZE;
                    }
                }
                Ok(())
            }));
        Ok(())
    }
}

#[allow(clippy::type_complexity)]
fn zombie_image_image(
    index_element: usize,
) -> impl Fn(
    Ref<UIRect<LevelConfig, State>>,
    Ref<LevelConfig>,
    Ref<State>,
) -> Result<&'static Texture<'static>> {
    move |_, _self, state| {
        if let Some(LevelSubElement::TextBox(t)) =
            _self.get_level_config().get_element(2, index_element)
        {
            match t.text().as_str().parse::<u8>() {
                Ok(id) if valide_zombie_id(id) => {
                    Ok(zombie_from_id(id).texture(state.as_ref().textures()))
                }
                _ => Err(anyhow!("error zombie id image should be hidden")),
            }
        } else {
            Err(anyhow!("error zombie id image should be hidden"))
        }
    }
}

fn zombie_image_state(
    index_element: usize,
) -> impl Fn(Ref<UIRect<LevelConfig, State>>, Ref<LevelConfig>, Ref<State>) -> StateEnum {
    move |_, _self, _| {
        if let Some(LevelSubElement::TextBox(t)) =
            _self.get_level_config().get_element(2, index_element)
        {
            match t.text().as_str().parse::<u8>() {
                Ok(id) if valide_zombie_id(id) => StateEnum::Enable,
                _ => StateEnum::Hidden,
            }
        } else {
            StateEnum::Hidden
        }
    }
}
