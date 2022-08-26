use crossterm::style;

use rand::Rng;
use std::collections::{HashMap, HashSet};

pub fn create_game() -> GameState {
    let level_width = 10;
    let level_height = 15;
    let mut board = Vec::new();

    for _ in 0..level_height {
        board.push(vec![0; level_width]);
    }

    let colors = HashMap::from([
        (1, style::Color::Red),
        (2, style::Color::Yellow),
        (3, style::Color::Blue),
        (4, style::Color::Green),
        (5, style::Color::Magenta),
        (6, style::Color::Cyan),
        (7, style::Color::DarkYellow),
    ]);

    let mut game = GameState {
        board,
        colors,
        figure_falling: Vec::new(),
        figure_falling_color: 1,
    };
    create_new_random_figure(&mut game);
    game
}

pub fn drop_one_step(game: &mut GameState) {
    if can_be_moved(game, MoveDirection::Down) {
        move_g(game, MoveDirection::Down);
    } else {
        create_new_random_figure(game);
    }
}
pub fn drop_to_end(game: &mut GameState) {
    while can_be_moved(game, MoveDirection::Down) {
        move_g(game, MoveDirection::Down);
    }
    create_new_random_figure(game);
}

pub fn shift(game: &mut GameState, direction: MoveDirection) {
    if can_be_moved(game, direction.clone()) {
        move_g(game, direction);
    }
}

fn create_new_random_figure(game: &mut GameState) {
    game.figure_falling.clear();
    insert_new_figure(game);
    paint_falling_figure(game);
}

fn can_be_moved(game: &GameState, direction: MoveDirection) -> bool {
    let mut cells_moving: HashSet<(usize, usize)> = HashSet::new();

    for p in &game.figure_falling {
        cells_moving.insert((p.x, p.y));
    }

    if matches!(direction, MoveDirection::Down) {
        for p in &game.figure_falling {
            let r = (p.x, p.y + 1);
            if p.y + 1 > game.board.len() - 1
                || (game.board[p.y + 1][p.x] != 0 && !cells_moving.contains(&r))
            {
                return false;
            }
        }
    } else if matches!(direction, MoveDirection::Left) {
        for p in &game.figure_falling {
            let r = (if p.x > 0 { p.x - 1 } else { 0 }, p.y);
            if p.x <= 0 || (game.board[p.y][p.x - 1] != 0 && !cells_moving.contains(&r)) {
                return false;
            }
        }
    } else {
        for p in &game.figure_falling {
            let r = (p.x + 1, p.y);
            if p.x + 1 > game.board[0].len() - 1
                || (game.board[p.y][p.x + 1] != 0 && !cells_moving.contains(&r))
            {
                return false;
            }
        }
    };

    true
}

fn move_g(game: &mut GameState, direction: MoveDirection) {
    unpaint_falling_figure(game);

    for p in game.figure_falling.iter_mut() {
        if matches!(direction, MoveDirection::Down) {
            p.y += 1;
        } else if matches!(direction, MoveDirection::Left) {
            p.x -= 1;
        } else {
            p.x += 1;
        };
    }
    paint_falling_figure(game)
}

fn unpaint_falling_figure(game: &mut GameState) {
    for p in &game.figure_falling {
        game.board[p.y][p.x] = 0;
    }
}

fn paint_falling_figure(game: &mut GameState) {
    for p in &game.figure_falling {
        game.board[p.y][p.x] = game.figure_falling_color;
    }
}

fn insert_new_figure(game: &mut GameState) {
    let figure_padding = game.board[0].len() / 2 - 1;
    let f = &mut game.figure_falling;

    let mut rng = rand::thread_rng();

    // simulate rolling a die:
    match rng.gen_range(1..=7) {
        1 => {
            f.push(point(0 + figure_padding, 0));
            f.push(point(0 + figure_padding, 1));
            f.push(point(1 + figure_padding, 1));
            f.push(point(1 + figure_padding, 2));
            game.figure_falling_color = 1;
        }
        2 => {
            f.push(point(0 + figure_padding, 0));
            f.push(point(1 + figure_padding, 0));
            f.push(point(2 + figure_padding, 0));
            f.push(point(3 + figure_padding, 0));
            game.figure_falling_color = 2;
        }
        3 => {
            f.push(point(0 + figure_padding, 0));
            f.push(point(1 + figure_padding, 0));
            f.push(point(1 + figure_padding, 1));
            f.push(point(2 + figure_padding, 1));
            game.figure_falling_color = 3;
        }
        4 => {
            f.push(point(0 + figure_padding, 0));
            f.push(point(0 + figure_padding, 1));
            f.push(point(1 + figure_padding, 0));
            f.push(point(1 + figure_padding, 1));
            game.figure_falling_color = 4;
        }
        5 => {
            f.push(point(0 + figure_padding, 0));
            f.push(point(0 + figure_padding, 1));
            f.push(point(0 + figure_padding, 2));
            f.push(point(1 + figure_padding, 2));
            game.figure_falling_color = 5;
        }
        6 => {
            f.push(point(0 + figure_padding, 0));
            f.push(point(0 + figure_padding, 1));
            f.push(point(0 + figure_padding, 2));
            f.push(point(1 + figure_padding, 0));
            game.figure_falling_color = 7;
        }
        7 => {
            f.push(point(0 + figure_padding, 0));
            f.push(point(1 + figure_padding, 0));
            f.push(point(2 + figure_padding, 0));
            f.push(point(1 + figure_padding, 1));
            game.figure_falling_color = 6;
        }
        _ => {
            panic!("Rolled number outside of handled")
        }
    }
}

fn point(x: usize, y: usize) -> Point {
    Point { x, y }
}

pub struct GameState {
    pub board: Vec<Vec<usize>>,
    pub colors: HashMap<usize, style::Color>,
    pub figure_falling: Vec<Point>,
    pub figure_falling_color: usize,
}

pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone)]
pub enum MoveDirection {
    Left,
    Down,
    Right,
}
