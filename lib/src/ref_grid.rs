use std::{collections::HashMap, time::Duration};

use sdl2::{rect::FRect, render::Canvas, video::Window};

use crate::{
    event::Event,
    grid::{ColType, GridChildren, Pos, RowType},
    user_control::UserControl,
};

pub struct RefGridElement<T> {
    surface: FRect,
    element: *mut dyn GridChildren<T>,
}

pub struct RefGrid<T> {
    parent: *mut T,
    elements: HashMap<Pos, RefGridElement<T>>,
    static_x: f32,
    static_y: f32,
    cols: Vec<ColType>,
    rows: Vec<RowType>,
    last_width: f32,
    last_height: f32,
}

impl<T> RefGrid<T> {
    /// # Safety
    /// can't call any function because will fail.
    #[allow(invalid_value)]
    pub unsafe fn empty() -> Self {
        Self {
            parent: std::ptr::null_mut(),
            elements: HashMap::new(),
            static_x: 0.,
            static_y: 0.,
            cols: Vec::new(),
            rows: Vec::new(),
            last_width: 0.,
            last_height: 0.,
        }
    }

    pub fn new(
        parent: *mut T,
        cols: Vec<ColType>,
        rows: Vec<RowType>,
        elements: HashMap<Pos, *mut dyn GridChildren<T>>,
    ) -> Self {
        let mut static_x = 0.;
        let mut dyn_x = 0.;
        for col in &cols {
            match col {
                ColType::Px(x) => static_x += *x,
                ColType::Ratio(x) => dyn_x += *x,
            }
        }
        let mut static_y = 0.;
        let mut dyn_y = 0.;
        for row in &rows {
            match row {
                RowType::Px(y) => static_y += *y,
                RowType::Ratio(y) => dyn_y += *y,
            }
        }

        let elements = elements
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    RefGridElement {
                        surface: FRect::new(0., 0., 0., 0.),
                        element: v,
                    },
                )
            })
            .collect();

        Self {
            parent,
            elements,
            static_x,
            static_y,
            cols: cols.into_iter().map(|c| c.scale_ration(dyn_x)).collect(),
            rows: rows.into_iter().map(|r| r.scale_ration(dyn_y)).collect(),
            last_width: 0.,
            last_height: 0.,
        }
    }
}

impl<T> UserControl for RefGrid<T> {
    fn init(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let parent = unsafe {
            self.parent
                .as_mut()
                .ok_or("unwrap ptr init parent refgrid")?
        };
        for (_, RefGridElement { element, .. }) in self.elements.iter_mut() {
            unsafe { element.as_mut().ok_or("unwrap ptr init refgrid")? }
                .grid_init(canvas, parent)?;
        }
        Ok(())
    }

    fn init_frame(&mut self, canvas: &mut Canvas<Window>, surface: FRect) -> Result<(), String> {
        let parent = unsafe {
            self.parent
                .as_mut()
                .ok_or("unwrap ptr init_frame parent refgrid")?
        };
        if self.last_width != surface.width() || self.last_height != surface.height() {
            let mut p_x = surface.x();
            let mut p_y = surface.y();
            let remain_width = surface.width() - self.static_x;
            let remain_height = surface.height() - self.static_y;
            if remain_width < 0. || remain_height < 0. {
                return Err(format!(
                    "Not enough space for grid: {remain_width}x{remain_height}"
                ));
            }

            for (y, pos_y) in self.rows.iter().enumerate() {
                let height = pos_y.to_px(remain_height);
                for (x, pos_x) in self.cols.iter().enumerate() {
                    let width = pos_x.to_px(remain_width);
                    if let Some(RefGridElement { surface, element }) =
                        self.elements.get_mut(&Pos { x, y })
                    {
                        *surface = FRect::new(p_x, p_y, width, height);
                        unsafe { element.as_mut().ok_or("unwrap ptr init_frame refgrid")? }
                            .grid_init_frame(canvas, *surface, parent)?;
                    }
                    p_x += width;
                }
                p_x = surface.x();
                p_y += height;
            }
            self.last_width = surface.width();
            self.last_height = surface.height();
        } else {
            for RefGridElement { surface, element } in self.elements.values_mut() {
                unsafe { element.as_mut().ok_or("unwrap ptr init_frame2 refgrid")? }
                    .grid_init_frame(canvas, *surface, parent)?;
            }
        }
        Ok(())
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        let parent = unsafe {
            self.parent
                .as_mut()
                .ok_or("unwrap ptr event parent refgrid")?
        };
        for (
            _,
            RefGridElement {
                surface, element, ..
            },
        ) in self.elements.iter_mut()
        {
            if let Some(event) = event.clone().hover(*surface) {
                unsafe { element.as_mut().ok_or("unwrap ptr event refgrid")? }
                    .grid_event(canvas, event, parent)?;
            }
        }
        Ok(())
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        let parent = unsafe {
            self.parent
                .as_mut()
                .ok_or("unwrap ptr update parent refgrid")?
        };
        for (_, RefGridElement { element, .. }) in self.elements.iter_mut() {
            unsafe { element.as_mut().ok_or("unwrap ptr update refgrid")? }
                .grid_update(canvas, elapsed, parent)?;
        }
        Ok(())
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let parent = unsafe {
            self.parent
                .as_ref()
                .ok_or("unwrap ptr draw parent refgrid")?
        };
        for (_, RefGridElement { element, .. }) in self.elements.iter() {
            unsafe { element.as_ref().ok_or("unwrap ptr draw refgrid")? }
                .grid_draw(canvas, parent)?;
        }
        Ok(())
    }
}

impl<K, V> GridChildren<K> for RefGrid<V> {
    fn grid_init(&mut self, canvas: &mut Canvas<Window>, _: &mut K) -> Result<(), String> {
        self.init(canvas)
    }

    fn grid_init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        surface: FRect,
        _: &mut K,
    ) -> Result<(), String> {
        self.init_frame(canvas, surface)
    }

    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        _: &mut K,
    ) -> Result<(), String> {
        self.event(canvas, event)
    }

    fn grid_update(
        &mut self,
        canvas: &mut Canvas<Window>,
        elapsed: Duration,
        _: &mut K,
    ) -> Result<(), String> {
        self.update(canvas, elapsed)
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &K) -> Result<(), String> {
        self.draw(canvas)
    }
}

#[macro_export]
macro_rules! refgrid {
    ($self:ident, $($col:expr),*; $($row:expr),*; $($pos:expr => $child:expr),* $(,)?) => {
        RefGrid::new(
            $self,
            vec![$($col),*],
            vec![$($row),*],
            HashMap::from([$(($pos, Box::new($child) as Box<dyn GridChildren<Win>>)),*])
        )
    };
}
