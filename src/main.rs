use std::collections::HashSet;
use std::fmt;
use std::{thread, time};

#[derive(Debug)]
struct GameOfLife {
    state: HashSet<(i32, i32)>,
}

impl GameOfLife {
    pub fn new() -> Self {
        GameOfLife {
            state: HashSet::new(),
        }
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        self.state.contains(&(x, y))
    }

    pub fn set(&mut self, x: i32, y: i32, value: bool) {
        match value {
            true => self.state.insert((x, y)),
            false => self.state.remove(&(x, y)),
        };
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

    pub fn tick(&mut self) {
        // consider neighbors (handles live cells, b/c live cells only survive if has neighbors)
        let neighbors: HashSet<(i32, i32)> = self
            .state
            .iter()
            .copied()
            .flat_map(|(x, y)| Self::get_neighbors(x, y))
            .collect();
        // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // 2. Any live cell with two or three live neighbours lives on to the next generation.
        // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
        // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        self.state = neighbors
            .iter()
            .copied()
            .filter(|(x, y)| {
                let c: usize = Self::get_neighbors(*x, *y)
                    .iter()
                    .filter(|(x, y)| self.get(*x, *y))
                    .count();
                return match (c, self.get(*x,*y)) {
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
        for y in (0..=5).rev() {
        for x in 0..=5 {
                let _ = match self.get(x,y) {
                    true  => write!(f, "◼"),
                    false => write!(f, "◻"),
                };
            }
            write!(f,"\n");
        }
        Ok(())
    }
}

fn main() {
    let mut game = GameOfLife::new();
    game.set(1, 1, true);
    game.set(2, 1, true);
    game.set(3, 1, true);
    println!("{:?}", game);
    loop {
        game.tick();
        //println!("{:?}", game);
        println!("{}", game);
        thread::sleep(time::Duration::from_millis(500));
    }
}
