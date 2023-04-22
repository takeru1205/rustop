use crossterm::{
    cursor, queue,
    style::{self, Print, Stylize},
    terminal, Result,
};
use std::io::Write;

pub fn display_usage_bar(
    usage: f32,
    index: u16,
    stdout: &mut impl Write,
    y: &mut u16,
    // preid: PreID,
    preid: u16,
) -> Result<()> {
    let (width, _) = terminal::size().unwrap();

    let half_width = (width - crate::EDGE) / 2;

    // Adjust the maximum width of the bar based on screen width
    let bar_max_width = (width - crate::EDGE * 2 - 10) / 2;

    let usage_bar_width: u16 = (usage / 100.0 * (bar_max_width as f32)) as u16;

    let print_style: style::PrintStyledContent<&str>;
    if usage < 40. {
        print_style = style::PrintStyledContent("■".green());
    } else if usage < 80. {
        print_style = style::PrintStyledContent("■".yellow());
    } else {
        print_style = style::PrintStyledContent("■".red());
    }

    let x_offset = crate::EDGE + 3 + index as u16 * half_width;
    for x in x_offset..(x_offset + usage_bar_width) {
        queue!(stdout, cursor::MoveTo(x, *y), print_style).unwrap();
    }

    for x in (x_offset + usage_bar_width)..(x_offset + half_width) {
        queue!(stdout, cursor::MoveTo(x, *y), Print(" ")).unwrap();
    }

    queue!(
        stdout,
        cursor::MoveTo(crate::EDGE + index as u16 * half_width, *y),
        Print(format!("{:>2}", preid))
    )
    .unwrap();

    Ok(())
}
