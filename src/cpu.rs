use crossterm::{
    cursor, queue,
    style::{self, Print, Stylize},
    terminal, Result,
};
use std::io::Write;
use sysinfo::{CpuExt, System, SystemExt};

pub fn display_cpu_info(sys: &mut System, stdout: &mut impl Write, y: &mut u16) -> Result<u16> {
    sys.refresh_cpu(); // Refreshing CPU information.
    let (width, _) = terminal::size().unwrap();

    let cpus = sys.cpus();
    let half_width = (width - 3) / 2;

    // Adjust the maximum width of the bar based on screen width
    let bar_max_width = (width - crate::EDGE * 2 - 10) / 2;

    // Display CPU usage bars in 2 columns
    for chunk in cpus.chunks(2) {
        for (index, cpu) in chunk.iter().enumerate() {
            let cpu_usage_percentage = cpu.cpu_usage() as f32;
            let cpu_usage_bar_width: u16 =
                (cpu_usage_percentage / 100.0 * (bar_max_width as f32)) as u16;

            let print_style: style::PrintStyledContent<&str>;
            if cpu_usage_percentage < 40. {
                // print_style = style::PrintStyledContent("█".green());
                print_style = style::PrintStyledContent("■".green());
            } else if cpu_usage_percentage < 80. {
                print_style = style::PrintStyledContent("■".yellow());
            } else {
                print_style = style::PrintStyledContent("■".red());
            }

            let x_offset = 3 + index as u16 * half_width;
            for x in x_offset..(x_offset + cpu_usage_bar_width) {
                queue!(stdout, cursor::MoveTo(x, *y), print_style).unwrap();
            }

            for x in (x_offset + cpu_usage_bar_width)..(x_offset + half_width) {
                queue!(stdout, cursor::MoveTo(x, *y), Print(" ")).unwrap();
            }

            queue!(
                stdout,
                cursor::MoveTo(crate::EDGE + index as u16 * half_width, *y),
                Print(format!("{: >02}", cpu_usage_percentage as u16))
            )
            .unwrap();
        }

        *y += 1;
    }

    Ok(*y)
}
