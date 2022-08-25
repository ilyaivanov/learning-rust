use crossterm::{cursor, execute, style, terminal, Result};

use std::io;

use crate::GameState;

pub fn draw(out: &mut io::Stdout, game: &GameState) -> Result<()> {
    execute!(
        out,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
    )?;

    let board = &game.board;
    let colors = &game.colors;

    let size = terminal::size().expect("Couldn't get the size of the terminal");

    let width = size.0 as usize;
    let padding = (width - (board[0].len() * 2 + 2)) / 2;
    let padding_str = " ".repeat(padding);

    let top_side = "▄".repeat(board[0].len() * 2 + 2);
    let bottom_side = "▀".repeat(board[0].len() * 2 + 2);

    let border_color = style::Color::DarkGrey;

    crossterm::execute!(out, style::SetForegroundColor(border_color))?;

    for _ in 0..2 {
        println!();
    }

    println!("{}{}", &padding_str, top_side);

    for line in board {
        print!("{}█", padding_str);
        for c in line {
            match c {
                0 => {
                    print!("  ");
                }
                v => {
                    let color = colors
                        .get(&v)
                        .expect(&format!("Couldn't find color for code: {}", v));

                    crossterm::execute!(out, style::SetForegroundColor(*color))?;
                    print!("██");
                }
            }
        }
        crossterm::execute!(out, style::SetForegroundColor(border_color))?;
        println!("█");
    }
    println!("{}{}", &padding_str, bottom_side);

    Ok(())
}

// ║╚╝╔╗╣╠╩═╬╦
// ▀  ▄  █
