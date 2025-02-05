use std::{collections::HashMap, fs, time::Duration};

use anyhow::{anyhow, Result};
use pvz::default_button;
use red_sdl::{
    event::Event,
    missing::ui_string::UIString,
    refs::{MutRef, Ref},
    simple_grid,
    ui_element::{
        grid::{ColType, Grid, Pos, RowType},
        scroll_view::ScrollView,
        text_box::TextBox,
        ui_rect::UIRect,
    },
    user_control::{BWindow, EventWindow, UserControl},
    zero,
};
use red_sdl_macro::UserControl;
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::FRect,
    render::Canvas,
    video::{FullscreenType, Window},
};

use crate::{level_config::LevelConfig, load_texts, map_config::MapConfig, State};

#[derive(UserControl)]
#[parent(Win)]
#[state(State)]
pub enum Page {
    Uninit(()),
    MainMenu(Grid<Win, State, PageMainMenu>),
    Map(MapConfig),
    Level(LevelConfig),
}

impl Page {
    #[allow(clippy::too_many_lines)]
    pub fn main_menu(state: Ref<State>) -> Self {
        Page::MainMenu(simple_grid!(
            ColType::Ratio(277.5),
            ColType::Ratio(235.),
            ColType::Ratio(10.),
            ColType::Ratio(235.),
            ColType::Ratio(10.),
            ColType::Ratio(235.),
            ColType::Ratio(277.5);
            RowType::Ratio(175.),
            RowType::Ratio(370.),
            RowType::Ratio(175.);
            Pos{x:1,y:1} => PageMainMenu::Scroll(ScrollView::new(
                Grid::new(
                    vec![ColType::Ratio(1.)],
                    (0..state.as_ref().maps_count).flat_map(|_| [RowType::Ratio(10.),RowType::Ratio(1.)]).take((state.as_ref().maps_count as usize)*2-1).collect(),
                    HashMap::from_iter((0..state.as_ref().maps_count).map(|map| {
                        (
                            Pos { x: 0, y: map as usize * 2 },
                            default_button().action(Box::new(
                                move |_, _self: MutRef<Win>,mut state:MutRef<State>, canvas| {
                                    state.as_mut().set_page(Page::Map(MapConfig::new(map, state.into())?));
                                    let Page::Map(config) = state.as_mut().get_page_mut() else {
                                        return Err(anyhow!("set mapconfig"));
                                    };
                                    MapConfig::add_pins(config.into(), state, canvas)?;

                                    UserControl::event(state.as_mut().get_page_mut().into(),
                                        canvas,
                                        Event::ElementMove {
                                            x: _self.surface.x(),
                                            y: _self.surface.y(),
                                        },
                                        _self,
                                        state,
                                    )?;
                                    UserControl::event(state.as_mut().get_page_mut().into(),
                                        canvas,
                                        Event::ElementResize {
                                            width: _self.surface.width(),
                                            height: _self.surface.height(),
                                        },
                                        _self,
                                        state,
                                    )
                                }))
                                .image(Box::new(move |_, _self,state| Ok(state.as_ref().textures().map(map as usize))))
                                .text(Box::new(
                                    move |_, _self,state| {
                                    UIString::new(state.as_ref().textures().font(), format!("{:0>3}", map+1)).map(|s| (s, Color::WHITE))
                                },
                            )),
                        )
                        }))
                    ),
                    235., 90. * state.as_ref().maps_count as f32, Box::new(|_, _, _|Color::RGBA(200,200,200,200)))),
            Pos{x:3, y:1} => ScrollView::new(
                Grid::new(
                vec![ColType::Ratio(1.)],
                (0..state.as_ref().levels_count).flat_map(|_| [RowType::Ratio(10.),RowType::Ratio(1.)]).take((state.as_ref().levels_count as usize)*2-1).collect(),
                HashMap::from_iter((0..state.as_ref().levels_count).map(|level| {
                    (
                        Pos { x: 0, y: level as usize * 2 },
                        default_button().action(Box::new(
                            move |_, _self: MutRef<Win>,mut state: MutRef<State>, canvas| {
                                state.as_mut().set_page(Page::Level(LevelConfig::new(level, _self.surface, state,canvas)?));
                                Ok(())
                            })).text(Box::new(
                                move |_,_self,state| {
                                UIString::new(state.as_ref().textures().font(), format!("{:0>3}", level+1)).map(|s| (s, Color::WHITE))
                            },
                        )),
                    )
                }))
                ), 235., 90. * state.as_ref().levels_count as f32, Box::new(|_,_, _|Color::RGBA(200,200,200,200))).into(),
            Pos{x:3, y:2} => default_button()
                .action(Box::new(
                    move |_, _self: MutRef<Win>, mut state: MutRef<State>, canvas| {
                        let level = state.as_ref().levels_count;
                        state.as_mut().levels_count += 1;
                        fs::write(format!("levels/{level}.data"), [0;6]).map_err(|e| anyhow!(e))?;
                        state.as_mut().set_page(Page::Level(LevelConfig::new(level, _self.surface, state,canvas)?));
                        Ok(())
                    }))
                    .text(Box::new(move |_, _, state| Ok((Some(UIString::new_const(state.as_ref().textures().font(), "Add Level")), Color::WHITE)))).into(),
            Pos{x:5,y:1} => simple_grid!(
                ColType::Ratio(1.);
                RowType::Ratio(10.),
                RowType::Ratio(1.),
                RowType::Ratio(10.),
                RowType::Ratio(1.),
                RowType::Ratio(10.),
                RowType::Ratio(1.),
                RowType::Ratio(10.),
                RowType::Ratio(1.),
                RowType::Ratio(10.);
                Pos{x:0,y:0} => default_button().action(Box::new(Win::quit)).text(Box::new(|_, _, state: Ref<State>| Ok((Some(state.as_ref().texts().quit.clone()), Color::WHITE)))),
            ).into()
        ))
    }
}

pub struct Win {
    running: bool,
    surface: FRect,
}

#[derive(UserControl)]
#[parent(Win)]
#[state(State)]
pub enum PageMainMenu {
    Label(UIRect<Win, State>),
    Scroll(ScrollView<Win, State, Grid<Win, State, UIRect<Win, State>>>),
    Grid(Grid<Win, State, UIRect<Win, State>>),
}

#[derive(UserControl)]
#[parent(Win)]
#[state(State)]
pub enum PageLevel {
    Label(UIRect<Win, State>),
    Grid(Grid<Win, State, PageMapSub>),
    Ref(LevelConfig),
}

#[derive(UserControl)]
#[parent(Win)]
#[state(State)]
pub enum PageMap {
    Label(UIRect<Win, State>),
    Grid(Grid<Win, State, PageMapSub>),
    Ref(LevelConfig),
}

#[derive(UserControl)]
#[parent(Win)]
#[state(State)]
pub enum PageMapSub {
    Label(UIRect<Win, State>),
    TextBox(TextBox<Win, State>),
}

impl Win {
    pub fn new(_: &mut Canvas<Window>, mut state: MutRef<State>) -> Result<Self> {
        load_texts(state);
        state.as_mut().set_page(Page::main_menu(state.into()));

        Ok(Self {
            running: true,
            surface: zero(),
        })
    }

    fn change_full_screen(&mut self, canvas: &Canvas<Window>) -> Result<()> {
        let window = unsafe {
            ((canvas as *const Canvas<Window>) as *mut Canvas<Window>)
                .as_mut()
                .ok_or(anyhow!("FullScreen"))
        }?
        .window_mut();
        window
            .set_fullscreen(if window.fullscreen_state() == FullscreenType::Off {
                FullscreenType::Desktop
            } else {
                FullscreenType::Off
            })
            .map_err(|e| anyhow!(e))?;
        Ok(())
    }

    fn quit(
        _: MutRef<UIRect<Win, State>>,
        mut this: MutRef<Self>,
        _: MutRef<State>,
        _: &Canvas<Window>,
    ) -> Result<()> {
        this.running = false;
        Ok(())
    }
}

impl BWindow<State> for Win {
    fn running(this: Ref<Self>, _: Ref<State>) -> bool {
        this.running
    }
}

impl EventWindow<State> for Win {}

impl UserControl<(), State> for Win {
    fn surface(this: Ref<Self>, _: Ref<()>, _: Ref<State>) -> FRect {
        this.surface
    }

    fn event(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        event: Event,
        _: MutRef<()>,
        mut state: MutRef<State>,
    ) -> Result<()> {
        match event {
            Event::ElementMove { .. } => {
                return Ok(());
            }
            Event::ElementResize { width, height } => {
                this.surface.set_width(width);
                this.surface.set_height(height);
                return UserControl::event(
                    state.as_mut().get_page_mut().into(),
                    canvas,
                    event,
                    this,
                    state,
                );
            }
            Event::Quit { .. } => this.running = false,
            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                this.change_full_screen(canvas)?;
            }
            _ => {}
        }
        UserControl::event(
            state.as_mut().get_page_mut().into(),
            canvas,
            event,
            this,
            state,
        )?;
        let surface = UserControl::surface(
            state.as_mut().get_page_mut().into(),
            this.into(),
            state.into(),
        );
        if surface.x() != this.surface.x() || surface.y() != this.surface.y() {
            UserControl::event(
                state.as_mut().get_page_mut().into(),
                canvas,
                Event::ElementMove {
                    x: this.surface.x(),
                    y: this.surface.y(),
                },
                this,
                state,
            )?;
        }
        if surface.width() != this.surface.width() || surface.height() != this.surface.height() {
            UserControl::event(
                state.as_mut().get_page_mut().into(),
                canvas,
                Event::ElementResize {
                    width: this.surface.width(),
                    height: this.surface.height(),
                },
                this,
                state,
            )?;
        }
        Ok(())
    }

    fn update(
        this: MutRef<Self>,
        canvas: &Canvas<Window>,
        elapsed: Duration,
        _: MutRef<()>,
        mut state: MutRef<State>,
    ) -> Result<()> {
        UserControl::update(
            state.as_mut().get_page_mut().into(),
            canvas,
            elapsed,
            this,
            state,
        )
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        _: Ref<()>,
        state: Ref<State>,
    ) -> Result<()> {
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();
        UserControl::draw(state.as_ref().get_page().into(), canvas, this, state)
    }
}
