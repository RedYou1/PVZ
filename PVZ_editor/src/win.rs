use std::{collections::HashMap, fs, time::Duration};

use pvz::{
    save::SaveFile,
    texts::{load_texts, Texts},
    textures::{load_textures, textures},
};
use sdl::{
    event::Event,
    functions::StateEnum,
    game_window::GameWindow,
    grid::{ColType, Grid, GridChildren, Pos, RowType},
    missing::ui_string::UIString,
    ref_element::RefElement,
    scroll_view::ScrollView,
    simple_grid,
    text_box::TextBox,
    ui_rect::UIRect,
    user_control::UserControl,
};
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::FRect,
    render::Canvas,
    video::{FullscreenType, Window},
};

use crate::{level_config::LevelConfig, map_config::MapConfig, rows_editor::RowsEditor};

pub enum Page {
    MainMenu,
    Map,
    Level,

    UnInitMainMenu,
    UnInitMap,
    UnInitLevel,
}

pub struct Win {
    running: bool,
    save: SaveFile,

    maps_count: u8,
    levels_count: u8,

    page: Page,
    save_ok: bool,
    pub map_config: MapConfig,
    col_text: UIString,
    pub level_config: LevelConfig,

    selected: Option<(String, usize, Option<usize>)>,
    main_menu_page: Grid<Win>,
    map_page: Grid<Win>,
    level_page: Grid<Win>,
}

impl Win {
    pub fn new(canvas: &mut Canvas<Window>) -> Result<Self, String> {
        load_textures(canvas, Box::leak(Box::new(canvas.texture_creator())))?;
        load_texts(&textures()?.font)?;

        let maps_count = fs::read_dir("assets/maps")
            .map_err(|e| e.to_string())?
            .filter(|f| {
                if let Ok(d) = f {
                    d.file_name()
                        .to_str()
                        .map_or(false, |s| s.to_lowercase().ends_with("data"))
                } else {
                    false
                }
            })
            .count();
        if maps_count == 0
            || fs::read_dir("assets/maps")
                .map_err(|e| e.to_string())?
                .count()
                > 99
        {
            return Err("Too much or no levels".to_owned());
        }
        let levels_count = fs::read_dir("levels").map_err(|e| e.to_string())?.count();
        if levels_count == 0 || fs::read_dir("levels").map_err(|e| e.to_string())?.count() > 99 {
            return Err("Too much or no levels".to_owned());
        }

        let font = &textures()?.font;

        Ok(Self {
            running: true,
            save_ok: false,
            save: SaveFile::load()?,
            maps_count: maps_count as u8,
            levels_count: levels_count as u8,
            page: Page::UnInitMainMenu,
            map_config: MapConfig::empty(),
            col_text: UIString::empty(font),
            level_config: LevelConfig::empty(font),
            selected: None,
            main_menu_page: unsafe { Grid::empty() },
            map_page: unsafe { Grid::empty() },
            level_page: unsafe { Grid::empty() },
        })
    }

    pub fn texts(&self) -> Result<&'static Texts, String> {
        self.save.texts()
    }

    fn change_full_screen(
        &mut self,
        _: f32,
        _: f32,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        let window = canvas.window_mut();
        window.set_fullscreen(if window.fullscreen_state() == FullscreenType::Off {
            FullscreenType::Desktop
        } else {
            FullscreenType::Off
        })?;
        Ok(())
    }

    fn quit(
        &mut self,
        _: &UIRect<Win>,
        _: f32,
        _: f32,
        _: &mut Canvas<Window>,
    ) -> Result<(), String> {
        self.running = false;
        Ok(())
    }

    fn _return(
        &mut self,
        _: &UIRect<Win>,
        _: f32,
        _: f32,
        _: &mut Canvas<Window>,
    ) -> Result<(), String> {
        self.page = Page::UnInitMainMenu;
        self.selected = None;
        Ok(())
    }

    fn add_row(
        &mut self,
        _: &UIRect<Win>,
        _: f32,
        _: f32,
        _: &mut Canvas<Window>,
    ) -> Result<(), String> {
        self.map_config
            .map
            .rows
            .push(pvz::level::config::RowType::Grass);
        Ok(())
    }

    fn sub_row(
        &mut self,
        _: &UIRect<Win>,
        _: f32,
        _: f32,
        _: &mut Canvas<Window>,
    ) -> Result<(), String> {
        self.map_config.map.rows.pop();
        Ok(())
    }

    fn set_map(&mut self, canvas: &mut Canvas<Window>, map: u8) -> Result<(), String> {
        self.save_ok = true;
        let _self = self as *mut Win;

        let map_config = MapConfig::new(map)?;
        self.map_config.grid_init(canvas, unsafe {
            _self.as_mut().ok_or("unwrap ptr".to_owned())?
        })?;
        let cols = map_config.map.cols.to_string();

        (self.page, self.selected, self.map_config, self.col_text) = (
            Page::UnInitMap,
            None,
            map_config,
            UIString::new(&textures()?.font, cols)?.ok_or("cant create col".to_owned())?,
        );

        Ok(())
    }

    fn set_level(&mut self, canvas: &mut Canvas<Window>, level: u8) -> Result<(), String> {
        let _self = self as *mut Win;

        let level_config = LevelConfig::new(level)?;
        self.level_config.grid_init(canvas, unsafe {
            _self.as_mut().ok_or("unwrap ptr".to_owned())?
        })?;
        (self.page, self.level_config) = (Page::UnInitLevel, level_config);
        Ok(())
    }

    fn save_map(&mut self) -> Result<(), String> {
        if let Ok(col) = self.col_text.as_str().parse() {
            self.map_config.map.cols = col;
        }
        self.map_config.map.save().map_err(|e| e.to_string())?;
        Ok(())
    }

    fn save_level(&mut self) -> Result<(), String> {
        self.save_ok = false;
        if self.level_config.try_save().is_ok() {
            self.level_config
                .level
                .save_config()
                .map_err(|e| e.to_string())?;
            self.save_ok = true;
        }
        Ok(())
    }

    fn reset_main_menu(&mut self) -> Result<(), String> {
        let font = &textures()?.font;
        self.main_menu_page = simple_grid!(
            self,
            Win,
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
            Pos{x:1,y:1} => ScrollView::new(Grid::new(
                self,
                vec![ColType::Ratio(1.)],
                (0..self.maps_count).flat_map(|_| [RowType::Ratio(10.),RowType::Ratio(1.)]).take((self.maps_count as usize)*2-1).collect(),
                HashMap::from_iter((0..self.maps_count).map(|map| {
                    (
                        Pos { x: 0, y: map as usize * 2 },
                        Box::new(UIRect::new(
                            Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(
                            move |_self: &mut Win, _, _, _, canvas| {
                                _self.set_map(canvas, map)
                            }))
                            .image(Box::new(move |_self, _| Ok(&textures()?.maps[map as usize])))
                            .text(Box::new(
                                move |_self,_| {
                                UIString::new(font, format!("{:0>3}", map+1)).map(|s| (s, Color::WHITE))
                            },
                        ))) as Box<dyn GridChildren<Win>>,
                    )
                }))
                ), 235., 90. * self.maps_count as f32, Box::new(|_, _|Color::RGBA(200,200,200,200))),
            Pos{x:3, y:1} => ScrollView::new(Grid::new(
                self,
                vec![ColType::Ratio(1.)],
                (0..self.levels_count).flat_map(|_| [RowType::Ratio(10.),RowType::Ratio(1.)]).take((self.levels_count as usize)*2-1).collect(),
                HashMap::from_iter((0..self.levels_count).map(|level| {
                    (
                        Pos { x: 0, y: level as usize * 2 },
                        Box::new(UIRect::new(
                            Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(
                            move |_self: &mut Win, _, _, _, canvas| {
                                _self.set_level(canvas, level)
                            })).text(Box::new(
                                move |_self,_| {
                                UIString::new(font, format!("{:0>3}", level+1)).map(|s| (s, Color::WHITE))
                            },
                        ))) as Box<dyn GridChildren<Win>>,
                    )
                }))
                ), 235., 90. * self.levels_count as f32, Box::new(|_, _|Color::RGBA(200,200,200,200))),
            Pos{x:3, y:2} => UIRect::new(Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK))
                .action(Box::new(
                    move |_self: &mut Win, _, _, _, canvas| {
                        let level = _self.levels_count;
                        _self.levels_count += 1;
                        fs::write(format!("levels/{level}.data"), [0;6]).map_err(|e|e.to_string())?;
                        _self.set_level(canvas, level)?;
                        _self.reset_main_menu()
                    }))
                    .text(Box::new(move |_,_| Ok((Some(UIString::new_const(font, "Add Level")), Color::WHITE)))),
            Pos{x:5,y:1} => simple_grid!(
                self,
                Win,
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
                Pos{x:0,y:0} => UIRect::new(Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).action(Box::new(Self::quit)).text(Box::new(|_self, _| Ok((Some(_self.texts()?.quit.clone()), Color::WHITE)))),
            )
        );
        Ok(())
    }
}
impl GameWindow for Win {
    fn running(&mut self) -> bool {
        self.running
    }

    #[allow(clippy::too_many_lines)]
    fn init(&mut self, _: &mut Canvas<Window>) -> Result<(), String> {
        let font = &textures()?.font;
        let map_config = &mut self.map_config as *mut MapConfig;
        let level_config = &mut self.level_config as *mut LevelConfig;
        self.reset_main_menu()?;
        self.map_page = simple_grid!(
            self,
            Win,
            ColType::Ratio(150.),
            ColType::Ratio(980.),
            ColType::Ratio(150.);
            RowType::Ratio(100.),
            RowType::Ratio(620.),
            RowType::Ratio(100.);
            Pos{x:0,y:0} => UIRect::new(Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).action(Box::new(Self::_return)).text(Box::new(|_self, _| Ok((Some(_self.texts()?._return.clone()), Color::WHITE)))),
            Pos{x:0,y:1} => RowsEditor::new(&mut self.map_config.map.rows),
            Pos{x:0,y:2} => simple_grid!(
                self,
                Win,
                ColType::Ratio(1.);
                RowType::Ratio(1.),
                RowType::Ratio(1.);
                Pos{x:0,y:0} => UIRect::new(Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).action(Box::new(Self::add_row)).text(Box::new(|_self, _| Ok((UIString::new(font, "Add row".to_owned())?, Color::WHITE)))),
                Pos{x:0,y:1} => UIRect::new(Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).action(Box::new(Self::sub_row)).text(Box::new(|_self, _| Ok((UIString::new(font, "Remove row".to_owned())?, Color::WHITE)))),
            ),
            Pos{x:1,y:1} => RefElement::new(unsafe{map_config.as_mut().ok_or("unwrap ptr")?}),
            Pos{x:1,y:0} => simple_grid!(
                self,
                Win,
                ColType::Ratio(1.),
                ColType::Ratio(10.),
                ColType::Ratio(1.),
                ColType::Ratio(10.),
                ColType::Ratio(1.),
                ColType::Ratio(10.),
                ColType::Ratio(1.),
                ColType::Ratio(10.);
                RowType::Ratio(1.);
                Pos{x:1,y:0} => UIRect::new( Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).text(Box::new(|_self: &Win, _| Ok((UIString::new(font, format!("top:{}", _self.map_config.map.top))?, Color::WHITE)))),
                Pos{x:3,y:0} => UIRect::new( Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).text(Box::new(|_self: &Win, _| Ok((UIString::new(font, format!("left:{}", _self.map_config.map.left))?, Color::WHITE)))),
                Pos{x:5,y:0} => UIRect::new( Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).text(Box::new(|_self: &Win, _| Ok((UIString::new(font, format!("width:{}", _self.map_config.map.width))?, Color::WHITE)))),
                Pos{x:7,y:0} => UIRect::new( Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).text(Box::new(|_self: &Win, _| Ok((UIString::new(font, format!("height:{}", _self.map_config.map.height))?, Color::WHITE)))),
            ),
            Pos{x:1,y:2} => simple_grid!(
                self,
                Win,
                ColType::Ratio(10.),
                ColType::Ratio(10.),
                ColType::Ratio(1.),
                ColType::Ratio(10.),
                ColType::Ratio(10.);
                RowType::Ratio(1.);
                Pos{x:0,y:0} => UIRect::new(Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).text(Box::new(|_self: &Win, _| Ok((UIString::new(font, "Cols:".to_owned())?, Color::WHITE)))),
                Pos{x:1,y:0} => TextBox::new(
                    "col".to_owned(),
                    &mut self.selected,
                    &textures()?.font,
                    &mut self.col_text,
                    Box::new(|_, _| StateEnum::Enable),
                    Box::new(|_, _| Color::RGBA(255, 255, 255, 100)),
                    Box::new(|_, _| Color::WHITE),
                    Box::new(|_, _| Color::WHITE),
                    Box::new(|_self: &Win, _| if _self.col_text.as_str().eq("0") || _self.col_text.as_str().parse::<u8>().is_err() {Color::RED} else {Color::BLACK}),
                ),
            ),
            Pos{x:2,y:2} => UIRect::new( Box::new(|_, _| StateEnum::Enable),Box::new(|_,_| Color::BLACK)).action(Box::new(|_self: &mut Win,_,_,_,_| _self.save_map())).text(Box::new(|_, _| Ok((UIString::new(font, "Save".to_owned())?, Color::WHITE)))),
        );
        self.level_page = simple_grid!(
            self,
            Win,
            ColType::Ratio(150.),
            ColType::Ratio(980.),
            ColType::Ratio(150.);
            RowType::Ratio(100.),
            RowType::Ratio(620.),
            RowType::Ratio(100.);
            Pos{x:0,y:0} => UIRect::new(Box::new(|_, _| StateEnum::Enable),Box::new(|_self: &Win,_| if _self.save_ok { Color::BLACK } else {Color::RED})).action(Box::new(Self::_return)).text(Box::new(|_self, _| Ok((Some(_self.texts()?._return.clone()), Color::WHITE)))),
            Pos{x:1,y:0} => simple_grid!(
                self,
                Win,
                ColType::Ratio(1.),
                ColType::Ratio(1.),
                ColType::Ratio(1.);
                RowType::Ratio(1.);
                Pos{x:0,y:0} => TextBox::new(
                    "money level".to_owned(),
                    &mut self.selected,
                    font,
                    &mut self.level_config.money,
                    Box::new(|_, _| StateEnum::Enable),
                    Box::new(|_, _| Color::RGBA(255, 255, 255, 100)),
                    Box::new(|_, _| Color::WHITE),
                    Box::new(|_, _| Color::WHITE),
                    Box::new(|_, t| {
                        if t.text().as_str().parse::<u32>().is_ok() {
                            Color::BLACK
                        } else {
                            Color::RED
                        }
                    }),
                ),
                Pos{x:1,y:0} => TextBox::new(
                    "map level".to_owned(),
                    &mut self.selected,
                    font,
                    &mut self.level_config.map,
                    Box::new(|_, _| StateEnum::Enable),
                    Box::new(|_, _| Color::RGBA(255, 255, 255, 100)),
                    Box::new(|_, _| Color::WHITE),
                    Box::new(|_, _| Color::WHITE),
                    Box::new(|_self: &Win, t| {
                        if let Ok(map) = t.text().as_str().parse::<u8>() {
                            if map < _self.maps_count{
                                return Color::BLACK;
                            }
                        }
                        Color::RED
                    }),
                ),
                Pos{x:2,y:0} => UIRect::new(
                    Box::new(|_self: &Win, _| {
                        if let Ok(map) = _self.level_config.map.as_str().parse::<u8>() {
                            if map < _self.maps_count {
                                return StateEnum::Enable;
                            }
                        }
                        StateEnum::Hidden
                    }),
                    Box::new(|_, _| Color::BLACK),
                )
                .image(Box::new(|_self: &Win, _| {
                    if let Ok(map) = _self.level_config.map.as_str().parse::<u8>() {
                        if map < _self.maps_count {
                            return Ok(&textures()?.maps[map as usize]);
                        }
                    }
                    Err("Not supposed showing".to_owned())
                }))
            ),
            Pos{x:1,y:1} => RefElement::new(unsafe{level_config.as_mut().ok_or("unwrap ptr")?}),
        );
        Ok(())
    }

    fn init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        width: f32,
        height: f32,
    ) -> Result<(), String> {
        self.page = match self.page {
            Page::UnInitMainMenu => {
                self.main_menu_page.init(canvas)?;
                Page::MainMenu
            }
            Page::UnInitMap => {
                self.map_page.init(canvas)?;
                Page::Map
            }
            Page::UnInitLevel => {
                self.level_page.init(canvas)?;
                Page::Level
            }
            Page::MainMenu => Page::MainMenu,
            Page::Map => Page::Map,
            Page::Level => Page::Level,
        };
        match self.page {
            Page::MainMenu => self
                .main_menu_page
                .init_frame(canvas, FRect::new(0., 0., width, height)),
            Page::Map => self
                .map_page
                .init_frame(canvas, FRect::new(0., 0., width, height)),
            Page::Level => self
                .level_page
                .init_frame(canvas, FRect::new(0., 0., width, height)),
            _ => Ok(()),
        }
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        match event {
            Event::Quit { .. } => self.running = false,
            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                self.change_full_screen(0., 0., canvas)?;
            }
            _ => {}
        }
        match self.page {
            Page::MainMenu => self.main_menu_page.event(canvas, event.clone()),
            Page::Map => self.map_page.event(canvas, event.clone()),
            Page::Level => self.level_page.event(canvas, event),
            _ => Ok(()),
        }
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        match self.page {
            Page::MainMenu => self.main_menu_page.update(canvas, elapsed),
            Page::Map => self.map_page.update(canvas, elapsed),
            Page::Level => {
                self.level_page.update(canvas, elapsed)?;
                self.save_level()
            }
            _ => Ok(()),
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();
        match self.page {
            Page::MainMenu => self.main_menu_page.draw(canvas),
            Page::Map => self.map_page.draw(canvas),
            Page::Level => self.level_page.draw(canvas),
            _ => Ok(()),
        }
    }
}
