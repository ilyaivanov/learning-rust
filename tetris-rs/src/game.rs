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
    board.push(vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 1]);

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
        falling_figure: FallingFigure {
            frame_x: 0,
            frame_y: 0,
            points: Vec::new(),
            color: 1,
        },
    };
    create_new_random_figure(&mut game);
    game
}

pub fn drop_one_step(game: &mut GameState) {
    if can_be_moved(game, MoveDirection::Down) {
        move_g(game, MoveDirection::Down);
    } else {
        on_piece_drop(game);
    }
}
pub fn rotate(game: &mut GameState) {
    unpaint_falling_figure(game);
    //TODO: need to check if rotation can be done
    for point in game.falling_figure.points.iter_mut() {
        match point {
            Point { x: 0, y: 0 } => point.x = 2,
            Point { x: 1, y: 0 } => {
                point.x = 2;
                point.y = 1;
            }
            Point { x: 2, y: 0 } => point.y = 2,

            Point { x: 0, y: 1 } => {
                point.x = 1;
                point.y = 0;
            }
            Point { x: 2, y: 1 } => {
                point.x = 1;
                point.y = 2;
            }

            Point { x: 0, y: 2 } => point.y = 0,
            Point { x: 1, y: 2 } => {
                point.x = 0;
                point.y = 1;
            }
            Point { x: 2, y: 2 } => point.x = 0,

            _ => {}
        }
    }
    paint_falling_figure(game)
}

pub fn drop_to_end(game: &mut GameState) {
    while can_be_moved(game, MoveDirection::Down) {
        move_g(game, MoveDirection::Down);
    }
    on_piece_drop(game);
}

pub fn shift(game: &mut GameState, direction: MoveDirection) {
    if can_be_moved(game, direction.clone()) {
        move_g(game, direction);
    }
}

fn on_piece_drop(game: &mut GameState) {
    let width = game.board[0].len();
    let starting_len = game.board.len();
    game.board.retain(|line| !is_line_full(line));

    let lines_removed = starting_len - game.board.len();
    if lines_removed > 0 {
        for _ in 0..lines_removed {
            game.board.insert(1, vec![0; width])
        }
    }

    create_new_random_figure(game);
}

fn is_line_full(line: &Vec<usize>) -> bool {
    line.iter().all(|a| *a != 0)
}

fn create_new_random_figure(game: &mut GameState) {
    game.falling_figure.points.clear();
    insert_new_figure(game);
    paint_falling_figure(game);
}

fn can_be_moved(game: &GameState, direction: MoveDirection) -> bool {
    let mut cells_moving: HashSet<(usize, usize)> = HashSet::new();

    let global_points = get_global_points(&game.falling_figure);
    for p in &global_points {
        cells_moving.insert((p.x, p.y));
    }

    let is_cell_occupied = |x: usize, y: usize| {
        let r = (x, y);
        game.board[y][x] != 0 && !cells_moving.contains(&r)
    };

    if matches!(direction, MoveDirection::Down) {
        for p in &global_points {
            if p.y + 1 > game.board.len() - 1 || is_cell_occupied(p.x, p.y + 1) {
                return false;
            }
        }
    } else if matches!(direction, MoveDirection::Left) {
        for p in &global_points {
            if p.x <= 0 || is_cell_occupied(p.x - 1, p.y) {
                return false;
            }
        }
    } else {
        for p in &global_points {
            if p.x + 1 > game.board[0].len() - 1 || is_cell_occupied(p.x + 1, p.y) {
                return false;
            }
        }
    };

    true
}

fn move_g(game: &mut GameState, direction: MoveDirection) {
    unpaint_falling_figure(game);

    if matches!(direction, MoveDirection::Down) {
        game.falling_figure.frame_y += 1;
    } else if matches!(direction, MoveDirection::Left) {
        game.falling_figure.frame_x -= 1;
    } else {
        game.falling_figure.frame_x += 1;
    };
    paint_falling_figure(game)
}

fn unpaint_falling_figure(game: &mut GameState) {
    for p in &get_global_points(&game.falling_figure) {
        game.board[p.y][p.x] = 0;
    }
}

fn paint_falling_figure(game: &mut GameState) {
    for p in &get_global_points(&game.falling_figure) {
        game.board[p.y][p.x] = game.falling_figure.color;
    }
}

fn insert_new_figure(game: &mut GameState) {
    game.falling_figure.frame_y = 0;
    game.falling_figure.frame_x = (game.board[0].len() / 2 - 1) as isize;
    let f = &mut game.falling_figure.points;

    let mut rng = rand::thread_rng();

    // simulate rolling a die:
    match rng.gen_range(1..=7) {
        1 => {
            f.push(point(0, 0));
            f.push(point(0, 1));
            f.push(point(1, 1));
            f.push(point(1, 2));
            game.falling_figure.color = 1;
        }
        2 => {
            f.push(point(0, 0));
            f.push(point(0, 1));
            f.push(point(1, 1));
            f.push(point(1, 2));
            game.falling_figure.color = 1;

            // LINE
            // TODO: need to figure out how to rotate this
            // f.push(point(0, 0));
            // f.push(point(1, 0));
            // f.push(point(2, 0));
            // f.push(point(3, 0));
            // game.falling_figure.color = 2;
        }
        3 => {
            f.push(point(0, 0));
            f.push(point(1, 0));
            f.push(point(1, 1));
            f.push(point(2, 1));
            game.falling_figure.color = 3;
        }
        4 => {
            f.push(point(0, 0));
            f.push(point(0, 1));
            f.push(point(1, 0));
            f.push(point(1, 1));
            game.falling_figure.color = 4;
        }
        5 => {
            f.push(point(0, 0));
            f.push(point(0, 1));
            f.push(point(0, 2));
            f.push(point(1, 2));
            game.falling_figure.color = 5;
        }
        6 => {
            f.push(point(0, 0));
            f.push(point(0, 1));
            f.push(point(0, 2));
            f.push(point(1, 0));
            game.falling_figure.color = 7;
        }
        7 => {
            f.push(point(0, 0));
            f.push(point(1, 0));
            f.push(point(2, 0));
            f.push(point(1, 1));
            game.falling_figure.color = 6;
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
    pub falling_figure: FallingFigure,
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub enum MoveDirection {
    Left,
    Down,
    Right,
}

pub struct FallingFigure {
    frame_x: isize,
    frame_y: usize,
    points: Vec<Point>,
    color: usize,
}

fn get_global_points(figure: &FallingFigure) -> Vec<Point> {
    figure
        .points
        .iter()
        .map(|p| Point {
            x: (p.x as isize + figure.frame_x) as usize,
            y: p.y + figure.frame_y,
        })
        .collect()
}
