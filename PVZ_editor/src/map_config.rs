use std::{collections::HashMap, time::Duration};

use anyhow::{anyhow, Result};
use pvz::{default_button, level::config::Map};
use red_sdl::{
    event::Event,
    functions::StateEnum,
    missing::ui_string::UIString,
    refs::{MutRef, Ref},
    simple_grid,
    ui_element::{
        grid::{ColType, Grid, Pos, RowType},
        panel::Panel,
        text_box::TextBox,
        ui_rect::UIRect,
    },
    user_control::UserControl,
};
use red_sdl_macro::UserControl;
use sdl2::{
    pixels::Color,
    rect::{FRect, Rect},
    render::Canvas,
    video::Window,
};

use crate::{pin::Pin, rows_editor::RowsEditor, win::Win, State};

#[derive(UserControl)]
#[parent(Win)]
#[state(State)]
pub struct MapConfig {
    pub map: Map,
    #[childSelf]
    grid: Grid<MapConfig, State, MapElement>,
}

#[derive(UserControl)]
#[parent(MapConfig)]
#[state(State)]
pub enum MapElement {
    Label(UIRect<MapConfig, State>),
    GridLabel(Grid<MapConfig, State, UIRect<MapConfig, State>>),
    Rows(RowsEditor),
    Grid(Grid<MapConfig, State, MapSubElement>),
    Pan(Panel<MapConfig, State, MapPanElement>),
}

#[derive(UserControl)]
#[parent(MapConfig)]
#[state(State)]
pub enum MapSubElement {
    Label(UIRect<MapConfig, State>),
    TextBox(TextBox<MapConfig, State>),
}

#[derive(UserControl)]
#[parent(MapConfig)]
#[state(State)]
pub enum MapPanElement {
    Img(UIRect<MapConfig, State>),
    Pin(Pin),
}

impl MapConfig {
    pub fn new(id: u8, state: Ref<State>) -> Result<Self> {
        let map = Map::load(id)?;
        let rows = map.rows.len() as u8;
        let cols = map.cols.to_string();
        Ok(Self {
            map,
            grid: simple_grid!(
                ColType::Ratio(150.),
                ColType::Ratio(980.),
                ColType::Ratio(150.);
                RowType::Ratio(100.),
                RowType::Ratio(620.),
                RowType::Ratio(100.);
                Pos{x:0,y:0} => default_button().action(Box::new(State::_return)).text(Box::new(|_, _, state: Ref<State>| Ok((Some(state.as_ref().texts()._return.clone()), Color::WHITE)))).into(),
                Pos{x:0,y:1} => RowsEditor::new(rows, state)?.into(),
                Pos{x:0,y:2} => simple_grid!(
                    ColType::Ratio(1.);
                    RowType::Ratio(1.),
                    RowType::Ratio(1.);
                    Pos{x:0,y:0} => default_button().action(Box::new(Self::add_row)).text(Box::new(|_, _self, state| Ok((UIString::new(state.as_ref().textures().font(), "Add row".to_owned())?, Color::WHITE)))),
                    Pos{x:0,y:1} => default_button().action(Box::new(Self::sub_row)).text(Box::new(|_, _self, state| Ok((UIString::new(state.as_ref().textures().font(), "Remove row".to_owned())?, Color::WHITE)))),
                ).into(),
                Pos{x:1,y:1} => Panel::new(vec![
                    UIRect::new( Box::new(|_, _, _| StateEnum::Enable), Box::new(|_, _, _| Color::RGBA(0, 0, 0, 0)))
                            .back_draw(Box::new(|ui, canvas, _self: Ref<MapConfig>, state: Ref<State>| {
                                canvas
                                    .copy_f(state.as_ref().textures().map(_self.map.id as usize), Some(Rect::new(0, 0, 762, 429)), ui.surface())
                                    .map_err(|e| anyhow!(e))
                            })).into(),
                    ]).into(),
                Pos{x:1,y:0} => simple_grid!(
                    ColType::Ratio(1.),
                    ColType::Ratio(10.),
                    ColType::Ratio(1.),
                    ColType::Ratio(10.),
                    ColType::Ratio(1.),
                    ColType::Ratio(10.),
                    ColType::Ratio(1.),
                    ColType::Ratio(10.);
                    RowType::Ratio(1.);
                    Pos{x:1,y:0} => default_button().text(Box::new(|_, _self: Ref<MapConfig>, state: Ref<State>| Ok((UIString::new(state.as_ref().textures().font(), format!("top:{}", _self.map.top))?, Color::WHITE)))),
                    Pos{x:3,y:0} => default_button().text(Box::new(|_, _self: Ref<MapConfig>, state: Ref<State>| Ok((UIString::new(state.as_ref().textures().font(), format!("left:{}", _self.map.left))?, Color::WHITE)))),
                    Pos{x:5,y:0} => default_button().text(Box::new(|_, _self: Ref<MapConfig>, state: Ref<State>| Ok((UIString::new(state.as_ref().textures().font(), format!("width:{}", _self.map.width))?, Color::WHITE)))),
                    Pos{x:7,y:0} => default_button().text(Box::new(|_, _self: Ref<MapConfig>, state: Ref<State>| Ok((UIString::new(state.as_ref().textures().font(), format!("height:{}", _self.map.height))?, Color::WHITE)))),
                ).into(),
                Pos{x:1,y:2} => simple_grid!(
                    ColType::Ratio(10.),
                    ColType::Ratio(10.),
                    ColType::Ratio(1.),
                    ColType::Ratio(10.),
                    ColType::Ratio(10.);
                    RowType::Ratio(1.);
                    Pos{x:0,y:0} => default_button().text(Box::new(|_, _self: Ref<MapConfig>, state: Ref<State>| Ok((UIString::new(state.as_ref().textures().font(), "Cols:".to_owned())?, Color::WHITE)))).into(),
                    Pos{x:1,y:0} => Into::<MapSubElement>::into(TextBox::new(
                        state.as_ref().textures().font(),
                        UIString::new(state.as_ref().textures().font(), cols)?.ok_or(anyhow!("cols too big"))?,
                        Box::new(|_, _, _| StateEnum::Enable),
                        Box::new(|_, _, _| Color::RGBA(255, 255, 255, 100)),
                        Box::new(|_, _, _| Color::WHITE),
                        Box::new(|_, _, _| Color::WHITE),
                        Box::new(|t, _self: Ref<MapConfig>, _| if t.text().as_str().eq("0") || t.text().as_str().parse::<u8>().is_err() {Color::RED} else {Color::BLACK}),
                    )),
                ).into(),
                Pos{x:2,y:2} => default_button().action(Box::new(|_, mut _self: MutRef<MapConfig>, _,_| _self.save())).text(Box::new(|_, _, state: Ref<State>| Ok((UIString::new(state.as_ref().textures().font(), "Save".to_owned())?, Color::WHITE)))).into(),
            ),
        })
    }

    pub fn add_pins(
        mut this: MutRef<Self>,
        state: MutRef<State>,
        canvas: &Canvas<Window>,
    ) -> Result<()> {
        let Some(MapElement::Pan(pins)) = this.grid.get_element_mut(1, 1) else {
            return Err(anyhow!("edit col not found"));
        };
        pins.state_manager.add(Box::new(|(mut this, mut elements)| {
            elements.push(Pin::new((&mut this.map).into(), true).into());
            elements.push(Pin::new((&mut this.map).into(), false).into());
            Ok(())
        }));
        UserControl::update(pins.into(), canvas, Duration::ZERO, this, state)
    }

    fn save(&mut self) -> Result<()> {
        let Some(MapElement::Grid(col)) = self.grid.get_element(1, 2) else {
            return Err(anyhow!("edit col not found"));
        };
        let Some(MapSubElement::TextBox(col)) = col.get_element(1, 0) else {
            return Err(anyhow!("edit col not found"));
        };

        let Ok(col) = col.text().as_str().parse() else {
            return Err(anyhow!("edit col faild"));
        };
        self.map.cols = col;
        self.map.save()
    }

    fn add_row(
        _: MutRef<UIRect<MapConfig, State>>,
        mut this: MutRef<Self>,
        _: MutRef<State>,
        _: &Canvas<Window>,
    ) -> Result<()> {
        this.map.rows.push(pvz::level::config::RowType::Grass);
        Ok(())
    }

    fn sub_row(
        _: MutRef<UIRect<MapConfig, State>>,
        mut this: MutRef<Self>,
        _: MutRef<State>,
        _: &Canvas<Window>,
    ) -> Result<()> {
        this.map.rows.pop();
        Ok(())
    }
}
