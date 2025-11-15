use std::collections::HashSet;
use std::fmt;
use std::ops::RangeInclusive;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Core Game of Life state machine backed by a sparse hash set.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GameOfLife {
    state: HashSet<(i32, i32)>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl GameOfLife {
    /// Creates a new, empty game board.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether the cell at `(x, y)` is alive.
    pub fn get(&self, x: i32, y: i32) -> bool {
        self.state.contains(&(x, y))
    }

    /// Marks the cell at `(x, y)` as alive.
    pub fn set(&mut self, x: i32, y: i32) {
        self.state.insert((x, y));
    }

    /// Marks the cell at `(x, y)` as dead.
    pub fn unset(&mut self, x: i32, y: i32) {
        self.state.remove(&(x, y));
    }

    /// Removes all live cells from the board.
    pub fn clear(&mut self) {
        self.state.clear();
    }

    /// Toggles the cell at `(x, y)` and returns the new state.
    pub fn toggle(&mut self, x: i32, y: i32) -> bool {
        if self.get(x, y) {
            self.unset(x, y);
            false
        } else {
            self.set(x, y);
            true
        }
    }

    /// Serializes a `width` by `height` viewport starting at the origin into a flat buffer of 0s and 1s.
    pub fn cells(&self, width: i32, height: i32) -> Vec<u8> {
        self.cells_at(width, height, 0, 0)
    }

    /// Serializes a `width` by `height` viewport starting at `(origin_x, origin_y)`.
    pub fn cells_at(&self, width: i32, height: i32, origin_x: i32, origin_y: i32) -> Vec<u8> {
        assert!(width >= 0 && height >= 0, "width and height must be non-negative");
        let mut cells = vec![0u8; (width * height) as usize];
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                cells[idx] = self.get(origin_x + x, origin_y + y) as u8;
            }
        }
        cells
    }

    fn get_neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::with_capacity(8);
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                neighbors.push((x + dx, y + dy));
            }
        }
        neighbors
    }

    /// Advances the simulation one generation in place.
    pub fn tick(&mut self) {
        // Start with the union of all neighbors of live cells to avoid scanning an infinite grid.
        let neighbors: HashSet<(i32, i32)> = self
            .state
            .iter()
            .copied()
            .flat_map(|(x, y)| Self::get_neighbors(x, y))
            .collect();

        self.state = neighbors
            .iter()
            .copied()
            .filter(|(x, y)| {
                let live_neighbor_count = Self::get_neighbors(*x, *y)
                    .iter()
                    .filter(|(nx, ny)| self.get(*nx, *ny))
                    .count();

                match (live_neighbor_count, self.get(*x, *y)) {
                    (0..=1, true) => false,
                    (2..=3, true) => true,
                    (4.., true) => false,
                    (3, false) => true,
                    (_, false) => false,
                }
            })
            .collect();
    }

}

impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Viewport::new(0..=5, 0..=5).render(self).fmt(f)
    }
}

/// Stores reusable viewing bounds for rendering `GameOfLife` states.
#[derive(Clone, Debug)]
pub struct Viewport {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl Viewport {
    pub fn new(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Self {
        Self { x_range, y_range }
    }

    pub fn render<'a>(&'a self, game: &'a GameOfLife) -> ViewportRender<'a> {
        ViewportRender {
            viewport: self,
            game,
        }
    }
}

pub struct ViewportRender<'a> {
    viewport: &'a Viewport,
    game: &'a GameOfLife,
}

impl<'a> ViewportRender<'a> {
    fn ordered_bounds(range: &RangeInclusive<i32>) -> (i32, i32) {
        let start = *range.start();
        let end = *range.end();
        if start <= end {
            (start, end)
        } else {
            (end, start)
        }
    }
}

impl<'a> fmt::Display for ViewportRender<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x_min, x_max) = Self::ordered_bounds(&self.viewport.x_range);
        let (y_min, y_max) = Self::ordered_bounds(&self.viewport.y_range);

        for y in (y_min..=y_max).rev() {
            for x in x_min..=x_max {
                if self.game.get(x, y) {
                    write!(f, "◼")?;
                } else {
                    write!(f, "◻")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
