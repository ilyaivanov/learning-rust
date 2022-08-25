use crossterm::style;

use std::collections::HashMap;

pub fn create_game() -> GameState {
    let board = vec![
        vec![0, 6, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 6, 6, 0, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 6, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 7, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 7, 7, 0, 0],
        vec![2, 2, 2, 3, 3, 1, 7, 0, 6, 0],
        vec![2, 5, 5, 3, 0, 1, 0, 0, 6, 6],
        vec![0, 5, 5, 3, 0, 1, 0, 4, 0, 6],
        vec![0, 0, 0, 0, 0, 1, 4, 4, 4, 0],
    ];

    let colors = HashMap::from([
        (1, style::Color::Red),
        (2, style::Color::Yellow),
        (3, style::Color::Blue),
        (4, style::Color::Green),
        (5, style::Color::Magenta),
        (6, style::Color::Cyan),
        (7, style::Color::DarkYellow),
    ]);

    GameState {
        board,
        colors,
        figure_falling: Vec::new(),
    }
}

pub struct GameState {
    pub board: Vec<Vec<usize>>,
    pub colors: HashMap<usize, style::Color>,
    pub figure_falling: Vec<Point>,
}

pub struct Point {
    x: usize,
    y: usize,
}
