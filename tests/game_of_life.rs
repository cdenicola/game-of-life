use gameoflife::GameOfLife;

/*
 * HELPER UTILITIES
 */
fn pattern_from_ascii(rows: &[&str]) -> GameOfLife {
    let mut game = GameOfLife::new();
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if matches!(ch, '#' | 'O' | 'o' | 'X') {
                game.set(x as i32, y as i32);
            }
        }
    }
    game
}

fn assert_period(mut game: GameOfLife, period: usize, name: &str) {
    let baseline = game.clone();
    for step in 1..period {
        game.tick();
        assert_ne!(
            baseline, game,
            "{name} unexpectedly returned to its baseline after {step} tick(s)"
        );
    }

    game.tick();
    assert_eq!(
        baseline, game,
        "{name} did not return to its baseline after {period} tick(s)"
    );
}

/*
 * SHAPES (sourced from https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Examples_of_patterns)
 */
fn blinker() -> GameOfLife {
    pattern_from_ascii(&[".#.", ".#.", ".#."])
}

fn square() -> GameOfLife {
    pattern_from_ascii(&["##", "##"])
}

fn tub() -> GameOfLife {
    pattern_from_ascii(&[".#.", "#.#", ".#."])
}

fn toad() -> GameOfLife {
    pattern_from_ascii(&[".###", "###."])
}

fn beacon() -> GameOfLife {
    pattern_from_ascii(&["##..", "##..", "..##", "..##"])
}

fn pulsar() -> GameOfLife {
    pattern_from_ascii(&[
        "..###...###..",
        ".............",
        "#....#.#....#",
        "#....#.#....#",
        "#....#.#....#",
        "..###...###..",
        ".............",
        "..###...###..",
        "#....#.#....#",
        "#....#.#....#",
        "#....#.#....#",
        ".............",
        "..###...###..",
    ])
}

fn pentadecathlon() -> GameOfLife {
    pattern_from_ascii(&[
        "..#..",
        ".#.#.",
        "#...#",
        "#...#",
        "#...#",
        "#...#",
        "#...#",
        "#...#",
        ".#.#.",
        "..#..",
    ])
}

/*
 * TESTS
 */
#[test]
fn set_marks_cells_alive() {
    let mut game = GameOfLife::new();
    game.set(0, 0);
    game.set(1, 0);
    game.set(2, 0);

    assert!(game.get(0, 0));
    assert!(game.get(1, 0));
    assert!(game.get(2, 0));
    assert!(!game.get(0, 1));
}

#[test]
fn unset_clears_cells() {
    let mut game = GameOfLife::new();
    game.set(0, 0);
    game.set(0, 1);
    game.unset(0, 1);

    assert!(game.get(0, 0));
    assert!(!game.get(0, 1));
}

#[test]
fn clear_resets_board() {
    let mut game = GameOfLife::new();
    game.set(0, 0);
    game.set(1, 1);
    game.clear();

    for x in -1..=1 {
        for y in -1..=1 {
            assert!(!game.get(x, y));
        }
    }
}

#[test]
fn equality_matches_structural_state() {
    let a = blinker();
    let mut b = blinker();

    assert_eq!(a, b);

    b.tick();
    assert_ne!(a, b);
}

#[test]
fn still_lifes() {
    for state in [square(), tub()] {
        let a = state.clone();
        let mut b = state.clone();
        assert_eq!(a, b);
        for _ in 0..=4 {
            b.tick();
            assert_eq!(a, b);
        }
    }
}

#[test]
fn oscillators_have_known_periods() {
    let oscillators = [
        ("blinker", blinker(), 2usize),
        ("toad", toad(), 2),
        ("beacon", beacon(), 2),
        ("pulsar", pulsar(), 3),
        ("pentadecathlon", pentadecathlon(), 15),
    ];

    for (name, oscillator, period) in oscillators {
        assert_period(oscillator, period, name);
    }
}
