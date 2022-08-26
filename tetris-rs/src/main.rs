use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute, style, terminal, Result,
};

use std::{
    io::stdout,
    time::{Duration, Instant},
};

pub mod game;
mod render;

fn main() -> Result<()> {
    let mut out = stdout();
    execute!(&out, terminal::EnterAlternateScreen)?;

    let mut game_state = game::create_game();
    let now = Instant::now();

    // let mut messages = Vec::new();

    let mut need_to_draw = true;
    let mut last_drop_at = 0;
    let drop_each = 1000;
    loop {
        if need_to_draw {
            clear(&mut out);
            render::draw(&mut out, &game_state)?;

            // for m in &messages {
            //     println!("{}", m);
            // }
            need_to_draw = false;
        }

        // Wait up to 1s for another event
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let key = read()?;
            // messages.push(format!("{:?}", key));
            match key {
                Event::Key(key_e) => match key_e.code {
                    KeyCode::Left => {
                        game::shift(&mut game_state, game::MoveDirection::Left);
                        need_to_draw = true;
                    }
                    KeyCode::Right => {
                        game::shift(&mut game_state, game::MoveDirection::Right);
                        need_to_draw = true;
                    }
                    KeyCode::Down => {
                        game::drop_one_step(&mut game_state);
                        last_drop_at = now.elapsed().as_millis();
                        need_to_draw = true;
                    }
                    KeyCode::Up => {
                        game::rotate(&mut game_state);
                        need_to_draw = true;
                    }
                    KeyCode::Char(' ') => {
                        game::drop_to_end(&mut game_state);
                        last_drop_at = now.elapsed().as_millis();
                        need_to_draw = true;
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                Event::Resize(_, _) => need_to_draw = true,
                _ => {}
            }
        }

        if now.elapsed().as_millis() - last_drop_at > drop_each {
            game::drop_one_step(&mut game_state);
            last_drop_at = now.elapsed().as_millis();
            need_to_draw = true;
        }
    }

    execute!(&out, terminal::LeaveAlternateScreen)
}

fn clear(out: &mut std::io::Stdout) {
    let _ = execute!(
        out,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
    );
}
