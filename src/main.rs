use std::{thread, time::Duration};

use gameoflife::{GameOfLife, Viewport};

fn main() {
    let mut game = GameOfLife::new();
    game.set(1, 1);
    game.set(2, 1);
    game.set(3, 1);

    let viewport = Viewport::new(0..=5, 0..=5);
    println!("{}", viewport.render(&game));

    loop {
        game.tick();
        println!("{}", viewport.render(&game));
        thread::sleep(Duration::from_millis(500));
    }
}
