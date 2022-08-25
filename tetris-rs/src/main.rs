use crossterm::{
    event::{read, Event, KeyCode},
    execute, style, terminal, Result,
};

use std::{collections::HashMap, io::stdout};

mod render;

fn main() -> Result<()> {
    let mut out = stdout();
    execute!(&out, terminal::EnterAlternateScreen)?;

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

    let game = GameState { board, colors };

    loop {
        render::draw(&mut out, &game)?;

        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                println!("{:?}", event);
                match event.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
            // Event::Mouse(event) => {}
            // Event::Resize(width, height) => {}
            _ => {}
        }
    }

    execute!(&out, terminal::LeaveAlternateScreen)
}

pub struct GameState {
    board: Vec<Vec<usize>>,
    colors: HashMap<usize, style::Color>,
}
