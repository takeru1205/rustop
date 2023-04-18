use crossterm::{
    cursor, queue,
    style::{self, Stylize},
    Result,
};
use std::io::Write;

pub fn draw_frame(stdout: &mut impl Write) -> Result<()> {
    let width = 150;
    let height = 40;

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
