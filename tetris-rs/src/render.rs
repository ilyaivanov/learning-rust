use crossterm::{cursor, execute, style, terminal, Result};

use std::io::{self, Stdout};

use crate::game::{self, GameState};

pub fn draw(out: &mut io::Stdout, game: &game::GameState) -> Result<()> {
    execute!(
        out,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
    )?;

    let board = &game.board;

    let size = terminal::size().expect("Couldn't get the size of the terminal");
    let terminal_width = size.0 as usize;

    let board_width_in_chars = board[0].len() * 2 + 2;

    let padding = (terminal_width - board_width_in_chars) / 2;
    let padding_str = " ".repeat(padding);

    let border_color = style::Color::DarkGrey;

    set_color(out, border_color);

    for _ in 0..2 {
        println!();
    }

    println!("{}{}", &padding_str, "▄".repeat(board_width_in_chars));

    for line in board {
        print!("{}█", padding_str);
        for c in line {
            match c {
                0 => print!("  "),

                v => {
                    set_color(out, get_color(&game, *v));
                    print!("██");
                }
            }
        }
        set_color(out, border_color);
        println!("█");
    }
    println!("{}{}", &padding_str, "▀".repeat(board_width_in_chars));

    Ok(())
}

fn get_color(game: &GameState, color_code: usize) -> style::Color {
    let c = game
        .colors
        .get(&color_code)
        .expect(&format!("Couldn't find color for code: {}", color_code));

    return *c;
}

fn set_color(out: &mut Stdout, color: style::Color) {
    let _ = crossterm::execute!(out, style::SetForegroundColor(color));
}

// ║╚╝╔╗╣╠╩═╬╦
// ▀  ▄  █
