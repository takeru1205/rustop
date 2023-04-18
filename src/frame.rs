use crossterm::{
    cursor, queue,
    style::{self, Stylize},
    Result,
};
use std::io::Write;

pub fn draw_frame(stdout: &mut impl Write) -> Result<()> {
    for y in 0..crate::HEIGHT {
        for x in 0..crate::WIDTH {
            if (y == 0 || y == crate::HEIGHT - 1) || (x == 0 || x == crate::WIDTH - 1) {
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
