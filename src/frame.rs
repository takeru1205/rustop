use crossterm::{
    cursor, queue,
    style::{self, Stylize},
    terminal, Result,
};
use std::io::Write;

pub fn draw_frame(stdout: &mut impl Write) -> Result<()> {
    let (width, height) = terminal::size().unwrap();
    for y in 0..height {
        for x in 0..width {
            if (y == 0 || y == height - 1) || (x == 0 || x == width - 1) {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent("█".blue())
                )?;
            }
        }
    }
    Ok(())
}
