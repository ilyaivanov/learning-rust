use console::Term;
use num_format::{Locale, ToFormattedString};
use std::cmp;
use std::io;

enum GameState {
    Running,
    Guessed,
}

fn main() {
    let show_info = false;

    let max_value: u64 = 1_000_000_000_000;
    println!(
        "Guess some number between 0 and {}. I shoud take around {} guesses. Press ENTER...",
        format(max_value),
        45
    );

    let _ = read_line();

    let mut pivot = max_value / 2;
    let mut step = max_value / 2;
    let mut game_state = GameState::Running;

    let mut guests = 1;

    while matches!(game_state, GameState::Running) {
        println!(
            "{}. Bigger than {}? (y - yes, n - no, c - correct)",
            guests,
            format(pivot)
        );
        step = cmp::max(step / 2, 1);
        if show_info {
            println!("Step: {}.", step);
        }
        guests += 1;

        match read_key() {
            'n' => {
                pivot -= step;
            }
            'c' => {
                game_state = GameState::Guessed;
                print!("Yay!!! You guessed {}", format(pivot));
            }
            'y' => {
                pivot += step;
            }
            _ => print!("unknown input"),
        }
    }
}

fn format(val: u64) -> String {
    return val.to_formatted_string(&Locale::uk);
}

fn read_line() -> String {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read from console");

    return str;
}

fn read_key() -> char {
    let term = Term::stdout();
    return Term::read_char(&term).expect("Failed to read from console");
}
