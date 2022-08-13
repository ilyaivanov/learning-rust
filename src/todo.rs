use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, Result,
};
use std::io::stdout;

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    let mut index = 0;
    loop {
        draw(index);

        if let Ok(c) = read_char() {
            match c {
                'q' => break,
                'j' => index = index + 1,
                'k' => index = if index == 0 { 0 } else { index - 1 },
                _ => {}
            }
        }
    }

    execute!(stdout(), LeaveAlternateScreen)
}

fn draw(selected_index: usize) {
    let lines = [
        "● Monday",
        "  ○ Item 1",
        "  ○ Item 2",
        "  ○ Item 3",
        "○ Tuesday",
    ];

    let _x = execute!(
        stdout(),
        ResetColor,
        Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 1),
    );

    for (c, line) in lines.iter().enumerate() {
        if c == selected_index {
            reverse_on();
        }
        print_line(line);

        if c == selected_index {
            reverse_off();
        }
    }
}

fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn reverse_on() {
    let _ = stdout().execute(SetAttribute(Attribute::Reverse));
}
fn reverse_off() {
    let _ = stdout().execute(SetAttribute(Attribute::NoReverse));
}
fn print_line(line: &str) {
    let _ = execute!(stdout(), Print(line), cursor::MoveToNextLine(1));
}
