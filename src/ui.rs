use crate::game::Game;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{self, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub fn draw(game: &Game) -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::All))?;
    let (fx, fy) = game.food_position;
    execute!(
        io::stdout(),
        MoveTo(fx, fy),
        style::PrintStyledContent("██".red())
    )?;
    for segment in &game.snake.body {
        execute!(
            io::stdout(),
            MoveTo(segment.0, segment.1),
            style::PrintStyledContent("██".green())
        )?;
    }
    io::stdout().flush()?;
    Ok(())
}
