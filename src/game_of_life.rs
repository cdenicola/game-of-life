use std::collections::HashSet;
use std::fmt;
use std::ops::RangeInclusive;

/// Core Game of Life state machine backed by a sparse hash set.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GameOfLife {
    state: HashSet<(i32, i32)>,
}

impl GameOfLife {
    /// Creates a new, empty game board.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether the cell at `(x, y)` is alive.
    pub fn get(&self, x: i32, y: i32) -> bool {
        self.state.contains(&(x, y))
    }

    /// Marks the cell at `(x, y)` as alive and returns `self` for chaining.
    pub fn set(&mut self, x: i32, y: i32) -> &mut Self {
        self.state.insert((x, y));
        self
    }

    /// Marks the cell at `(x, y)` as dead and returns `self` for chaining.
    pub fn unset(&mut self, x: i32, y: i32) -> &mut Self {
        self.state.remove(&(x, y));
        self
    }

    /// Removes all live cells from the board and returns `self` for chaining.
    pub fn clear(&mut self) -> &mut Self {
        self.state.clear();
        self
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

    /// Returns `true` when `other` has exactly the same live cells as `self`.
    pub fn same_state(&self, other: &Self) -> bool {
        self == other
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
