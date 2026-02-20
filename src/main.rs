mod game;
mod terminal;
mod ui;

use crossterm::{
    event::{Event, KeyCode, poll, read},
    terminal::size,
};
use game::Game;
use std::{io, process, thread, time::Duration};
use terminal::Terminal;

fn main() {
    if let Err(e) = run() {
        eprint!("{e}");
        process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let mut game = Game::new(size()?);
    let _terminal = Terminal::start()?;
    loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => game.snake.change_direction(game::Direction::Up),
                    KeyCode::Down => game.snake.change_direction(game::Direction::Down),
                    KeyCode::Right => game.snake.change_direction(game::Direction::Right),
                    KeyCode::Left => game.snake.change_direction(game::Direction::Left),
                    _ => {}
                },
                Event::Resize(c, r) => game.change_sreen_size((c, r)),
                _ => {}
            }
        }
        if !game.update() {
            println!("Game over!");
            thread::sleep(Duration::from_secs(3));
            break;
        }
        ui::draw(&game)?;
    }
    Ok(())
}
