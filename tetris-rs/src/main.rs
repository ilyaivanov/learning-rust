use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    style::ResetColor,
    terminal, Result,
};
use std::io::stdout;

fn main() -> Result<()> {
    let out = stdout();
    execute!(&out, terminal::EnterAlternateScreen)?;

    loop {
        draw();
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                println!("{:?}", event);
                match event.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
            Event::Mouse(event) => {}
            Event::Resize(width, height) => {}
            _ => {}
        }
    }

    execute!(&out, terminal::LeaveAlternateScreen)
}

fn draw() {
    let _ = execute!(
        stdout(),
        ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
    );
    let size = terminal::size().expect("Couldn't get the size of the terminal");

    let width = size.0 as usize;
    let height = size.1 as usize;
    let area_size = "║                            ║".len();
    let padding = (width - area_size) / 2;
    let padding_str = " ".repeat(padding);
    draw_str_at_center(
        &format!("Width: {}, Height: {}", width, height),
        &padding_str,
    );

    draw_str_at_center("╔════════════════════════════╗", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║            ██              ║", &padding_str);
    draw_str_at_center("║        ██████              ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("║                            ║", &padding_str);
    draw_str_at_center("╚════════════════════════════╝", &padding_str);
}

fn draw_str_at_center(str: &str, padd: &str) {
    println!("{}{}", padd, str);
}

// ║╚╝╔╗╣╠╩═╬╦ █
