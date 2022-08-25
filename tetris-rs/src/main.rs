use crossterm::{
    event::{read, Event, KeyCode},
    execute, terminal, Result,
};

use std::io::stdout;
mod game;
mod render;

fn main() -> Result<()> {
    let mut out = stdout();
    execute!(&out, terminal::EnterAlternateScreen)?;

    let game = game::create_game();
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
