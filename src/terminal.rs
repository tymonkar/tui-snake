use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io;

pub struct Terminal;

impl Terminal {
    pub fn start() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(Self)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        execute!(io::stdout(), LeaveAlternateScreen, Show).ok();
        disable_raw_mode().ok();
    }
}
