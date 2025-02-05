use std::{collections::HashMap, thread, time::Duration};

use anyhow::{anyhow, Ok, Result};
use red_sdl::{
    event::Event,
    functions::StateEnum,
    missing::ui_string::UIString,
    refs::{MutRef, Ref},
    simple_grid,
    ui_element::{
        grid::{ColType, Grid, Pos, RowType},
        scroll_view::ScrollView,
        ui_rect::UIRect,
    },
    user_control::{BWindow, GameWindow, UserControl},
};
use red_sdl_macro::UserControl;
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::FRect,
    render::Canvas,
    video::{FullscreenType, Window},
};
use serde_json::Value;

use crate::{default_button, level::Level, texts::load_texts, State};

pub struct Win {
    running: bool,
    surface: FRect,
    pub pause: bool,

    level: Option<Level>,

    main_menu: Grid<Win, State, MainMenuElement>,
    options: Grid<Win, State, UIRect<Win, State>>,
    overlay: Grid<Win, State, UIRect<Win, State>>,
}

#[derive(UserControl)]
#[parent(Win)]
#[state(State)]
enum MainMenuElement {
    Sub1(Grid<Win, State, UIRect<Win, State>>),
    Sub2(ScrollView<Win, State, Grid<Win, State, UIRect<Win, State>>>),
}

impl Win {
    #[allow(clippy::too_many_lines)]
    pub fn new(canvas: &mut Canvas<Window>, mut state: MutRef<State>) -> Result<Self> {
        load_texts(state);

        let s = state.as_mut();
        thread::spawn(|| s.update_available = Some(Win::update_available()));
        let mut s = Self {
            running: true,
            surface: FRect::new(0., 0., -1., -1.),
            pause: false,
            level: None,
            main_menu: simple_grid!(
                ColType::Ratio(380.),
                ColType::Ratio(275.),
                ColType::Ratio(10.),
                ColType::Ratio(235.),
                ColType::Ratio(380.);
                RowType::Ratio(175.),
                RowType::Ratio(370.),
                RowType::Ratio(175.);
                Pos { x: 1, y: 1 } => simple_grid!(
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
                    Pos { x: 0, y: 0 } => default_button()
                        .action(Box::new(Self::next_lang))
                        .text(Box::new(|_, _self, state| {
                            Ok((Some(state.as_ref().texts().lang.clone()), Color::WHITE))
                        })),
                    Pos { x: 0, y: 2 } => default_button()
                        .action(Box::new(|_, _self, _, canvas| {
                            Self::change_full_screen(_self, canvas)
                        }))
                        .text(Box::new(|_, _self, state| {
                            Ok((
                                Some(state.as_ref().texts().full_screen.clone()),
                                Color::WHITE,
                            ))
                        })),
                    Pos { x: 0, y: 4 } => default_button()
                        .action(Box::new(Self::quit))
                        .text(Box::new(|_, _self, state| {
                            Ok((Some(state.as_ref().texts().quit.clone()), Color::WHITE))
                        })),
                    Pos { x: 0, y: 6 } => default_button()
                        .text(Box::new(|_, _self, state| {
                            Ok((
                                Some(match state.update_available.as_ref() {
                                    Some(std::result::Result::Ok(true)) => {
                                        state.as_ref().texts().update_available.clone()
                                    }
                                    Some(std::result::Result::Ok(false)) => {
                                        state.as_ref().texts().up_to_date.clone()
                                    }
                                    Some(Err(e)) => {
                                        UIString::new(state.as_ref().textures().font(), e.to_string())?
                                            .ok_or(anyhow!("Error too long"))?
                                    }
                                    None => state.as_ref().texts().loading.clone(),
                                }),
                                Color::WHITE,
                            ))
                        })),
                    ).into(),
                Pos { x: 3, y: 1 } => ScrollView::new(
                        Grid::new(
                            vec![ColType::Ratio(1.)],
                            (0..state.levels_count)
                                .flat_map(|_| [RowType::Ratio(10.), RowType::Ratio(1.)])
                                .take((state.levels_count as usize) * 2 - 1)
                                .collect(),
                            HashMap::from_iter((0..state.levels_count).map(|level| {
                                (
                                    Pos {
                                        x: 0,
                                        y: level as usize * 2,
                                    },
                                    default_button()
                                    .action(Box::new(
                                        move |_, mut _self:MutRef<Win>, state, canvas| {
                                            let mut level = Level::load(
                                                level
                                            )?;
                                            let surface = _self.surface;
                                            Level::event((&mut level).into(),canvas,
                                                Event::ElementMove { x: surface.x(), y: surface.y() },
                                                _self,state)?;
                                            Level::event((&mut level).into(),canvas,
                                                Event::ElementResize { width: surface.width(), height: surface.height() },
                                                _self,state)?;

                                            _self.as_mut().level = Some(level);
                                            Ok(())
                                        },
                                    ))
                                    .text(Box::new(
                                        move |_, _, _state| {
                                            UIString::new(
                                                _state.as_ref().textures().font(),
                                                format!("{:0>3}", level + 1),
                                            )
                                            .map(|s| (s, Color::WHITE))
                                        },
                                    )),
                                )
                            })),
                        ),
                        235.,
                        90. * (state.levels_count as f32),
                        Box::new(|_, _, _| Color::RGBA(200, 200, 200, 200)),
                    ).into(),
            ),
            options: simple_grid!(
                ColType::Ratio(565.),
                ColType::Ratio(150.),
                ColType::Ratio(565.);
                RowType::Ratio(200.),
                RowType::Ratio(40.),
                RowType::Ratio(20.),
                RowType::Ratio(40.),
                RowType::Ratio(20.),
                RowType::Ratio(40.),
                RowType::Ratio(20.),
                RowType::Ratio(40.),
                RowType::Ratio(300.);
                Pos { x: 1, y: 1 } => default_button()
                    .action(Box::new(Self::next_lang))
                    .text(Box::new(|_, _self, state| {
                        Ok((Some(state.as_ref().texts().lang.clone()), Color::WHITE))
                    })),
                Pos { x: 1, y: 3 } => default_button()
                    .action(Box::new(|_, _self, _, canvas| {
                        Self::change_full_screen(_self, canvas)
                    }))
                    .text(Box::new(|_, _self, state| {
                         Ok((Some(state.as_ref().texts().full_screen.clone()), Color::WHITE))
                    })),
                Pos { x: 1, y: 5 } => default_button()
                    .action(Box::new(Self::_return))
                    .text(Box::new(|_, _self, state| {
                        Ok((Some(state.as_ref().texts()._return.clone()), Color::WHITE))
                    })),
                Pos { x: 1, y: 7 } => default_button()
                    .action(Box::new(Self::quit))
                    .text(Box::new(|_, _self, state| {
                        Ok((Some(state.as_ref().texts().quit.clone()), Color::WHITE))
                    })),
            ),
            overlay: simple_grid!(
                ColType::Ratio(1120.),
                ColType::Ratio(150.),
                ColType::Ratio(10.);
                RowType::Ratio(10.),
                RowType::Ratio(100.),
                RowType::Ratio(500.),
                RowType::Ratio(100.),
                RowType::Ratio(10.);
                Pos { x: 1, y: 1 } => default_button()
                    .action(Box::new(Self::menu))
                    .text(Box::new(|_, _self, state| {
                        Ok((Some(state.as_ref().texts().menu.clone()), Color::WHITE))
                    })),
                Pos { x: 1, y: 3 } => UIRect::new(
                        Box::new(|_, _self: Ref<Win>, _| {
                                if let Some(level) = _self.level.as_ref() {
                                    if level.started.is_none() {
                                        StateEnum::Enable
                                    } else {
                                        StateEnum::Hidden
                                    }
                                } else {
                                    StateEnum::Hidden
                                }
                            }),
                        Box::new(|_, _, _| Color::BLACK),
                    )
                    .action(Box::new(Self::play))
                    .text(Box::new(|_, _self, state| {
                        Ok((Some(state.as_ref().texts().start.clone()), Color::WHITE))
                    })),
            ),
        };
        let s2 = MutRef::new(&mut s);
        UserControl::event(
            (&mut s.main_menu).into(),
            canvas,
            Event::ElementMove { x: 0., y: 0. },
            s2,
            state,
        )?;
        UserControl::event(
            (&mut s.options).into(),
            canvas,
            Event::ElementMove { x: 0., y: 0. },
            s2,
            state,
        )?;
        UserControl::event(
            (&mut s.overlay).into(),
            canvas,
            Event::ElementMove { x: 0., y: 0. },
            s2,
            state,
        )?;

        if let Some(level) = s.level.as_mut() {
            UserControl::event(
                level.into(),
                canvas,
                Event::ElementMove { x: 0., y: 0. },
                s2,
                state,
            )?;
        }
        Ok(s)
    }

    fn update_available() -> Result<bool> {
        let req = reqwest::blocking::Client::builder()
            .build()
            .map_err(|e| anyhow!(e))?
            .get("https://api.github.com/repos/RedYou1/PVZ/releases")
            .header("User-Agent", "PVZ")
            .send()
            .map_err(|e| anyhow!(e))?;
        let text = req.text().map_err(|e| anyhow!(e))?;
        let json: Value = serde_json::from_str(text.as_str()).map_err(|e| anyhow!(e))?;
        let releases = json.as_array().ok_or(anyhow!("Error fetching"))?;
        let releases: Vec<&str> = releases
            .iter()
            .filter_map(|e| e["tag_name"].as_str())
            .skip_while(|e| !e.starts_with("pvz"))
            .collect();
        let first = *releases.first().ok_or(anyhow!("Error fetching"))?;
        Ok(first.ne("pvz_v0.2.0"))
    }

    fn next_lang(
        _: MutRef<UIRect<Win, State>>,
        _: MutRef<Self>,
        mut state: MutRef<State>,
        _: &Canvas<Window>,
    ) -> Result<()> {
        state.save.next_lang()
    }

    fn change_full_screen(_: MutRef<Self>, canvas: *const Canvas<Window>) -> Result<()> {
        let window = unsafe {
            (canvas as *mut Canvas<Window>)
                .as_mut()
                .ok_or(anyhow!("get window mut full screen"))
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

    fn _return(
        _: MutRef<UIRect<Win, State>>,
        mut this: MutRef<Self>,
        _: MutRef<State>,
        _: &Canvas<Window>,
    ) -> Result<()> {
        this.level = None;
        this.pause = false;
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

    fn menu(
        _: MutRef<UIRect<Win, State>>,
        mut this: MutRef<Self>,
        _: MutRef<State>,
        _: &Canvas<Window>,
    ) -> Result<()> {
        let t = !this.pause;
        this.pause = t;
        Ok(())
    }

    fn play(
        _: MutRef<UIRect<Win, State>>,
        mut this: MutRef<Self>,
        state: MutRef<State>,
        canvas: &Canvas<Window>,
    ) -> Result<()> {
        if let Some(level) = this.as_mut().level.as_mut() {
            Level::start(level.into(), canvas, state)?;
        }
        Ok(())
    }
}

impl BWindow<State> for Win {
    fn running(this: Ref<Self>, _: Ref<State>) -> bool {
        this.running
    }
}

impl GameWindow<State> for Win {
    fn time_scale(this: Ref<Self>, _: Ref<State>) -> f32 {
        if this.pause {
            0.
        } else {
            1.
        }
    }

    fn fps(_: Ref<Self>, _: Ref<State>) -> f32 {
        60.
    }
}

impl UserControl<(), State> for Win {
    fn surface(this: Ref<Self>, _: Ref<()>, _: Ref<State>) -> FRect {
        this.surface
    }

    fn event(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        event: Event,
        _: MutRef<()>,
        state: MutRef<State>,
    ) -> Result<()> {
        match event {
            Event::ElementResize { width, height } => {
                this.surface.set_width(width);
                this.surface.set_height(height);

                UserControl::event(
                    (&mut this.main_menu).into(),
                    canvas,
                    event.clone(),
                    this,
                    state,
                )?;
                UserControl::event(
                    (&mut this.options).into(),
                    canvas,
                    event.clone(),
                    this,
                    state,
                )?;
                UserControl::event(
                    (&mut this.overlay).into(),
                    canvas,
                    event.clone(),
                    this,
                    state,
                )?;

                if let Some(level) = this.level.as_mut() {
                    UserControl::event(level.into(), canvas, event, this, state)?;
                }
                return Ok(());
            }
            Event::Quit { .. } => this.running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                let t = !this.pause;
                this.pause = t;
            }
            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                Self::change_full_screen(this, canvas)?;
            }
            _ => {}
        }
        if let Some(level) = this.as_mut().level.as_mut() {
            if this.pause {
                UserControl::event(
                    (&mut this.options).into(),
                    canvas,
                    event.clone(),
                    this,
                    state,
                )?;
            }
            UserControl::event(
                (&mut this.overlay).into(),
                canvas,
                event.clone(),
                this,
                state,
            )?;
            UserControl::event(level.into(), canvas, event.clone(), this, state)
        } else {
            UserControl::event((&mut this.main_menu).into(), canvas, event, this, state)
        }
    }

    fn update(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        elapsed: Duration,
        _: MutRef<()>,
        state: MutRef<State>,
    ) -> Result<()> {
        if let Some(level) = this.as_mut().level.as_mut() {
            if this.pause {
                UserControl::update((&mut this.options).into(), canvas, elapsed, this, state)
            } else {
                UserControl::update((&mut this.overlay).into(), canvas, elapsed, this, state)?;
                UserControl::update(level.into(), canvas, elapsed, this, state)
            }
        } else {
            UserControl::update((&mut this.main_menu).into(), canvas, elapsed, this, state)
        }
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        _: Ref<()>,
        state: Ref<State>,
    ) -> Result<()> {
        if let Some(level) = this.level.as_ref() {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            UserControl::draw(level.into(), canvas, this, state)?;
            UserControl::draw((&this.overlay).into(), canvas, this, state)?;
            if this.pause {
                UserControl::draw((&this.options).into(), canvas, this, state)?;
            }
            return Ok(());
        }
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();

        UserControl::draw((&this.main_menu).into(), canvas, this, state)
    }
}
