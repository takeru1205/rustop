use crossterm::{
    cursor, queue,
    style::{self, Print, Stylize},
    Result,
};
use std::io::Write;
use sysinfo::{CpuExt, System, SystemExt};

pub fn display_cpu_info(
    sys: &mut System,
    stdout: &mut impl Write,
    y: &mut u16,
    width: u16,
) -> Result<u16> {
    sys.refresh_cpu(); // Refreshing CPU information.

    let cpus = sys.cpus();
    let half_width = (width - 3) / 2;

    // Display CPU usage bars in 2 columns
    for chunk in cpus.chunks(2) {
        for (index, cpu) in chunk.iter().enumerate() {
            let cpu_usage: u16 = ((cpu.cpu_usage() as f32) / 100.0 * (half_width as f32)) as u16;

            let print_style: style::PrintStyledContent<&str>;
            if cpu_usage < 40 {
                print_style = style::PrintStyledContent("█".green());
            } else if cpu_usage < 80 {
                print_style = style::PrintStyledContent("█".yellow());
            } else {
                print_style = style::PrintStyledContent("█".red());
            }

            let x_offset = 3 + index as u16 * half_width;
            for x in x_offset..(x_offset + cpu_usage) {
                queue!(stdout, cursor::MoveTo(x, *y), print_style).unwrap();
            }

            for x in (x_offset + cpu_usage)..(x_offset + half_width) {
                queue!(stdout, cursor::MoveTo(x, *y), Print(" ")).unwrap();
            }

            queue!(
                stdout,
                cursor::MoveTo(3 + index as u16 * half_width, *y),
                Print(format!("{: >02}", cpu_usage))
            )
            .unwrap();
        }

        *y += 1;
    }

    Ok(*y)
}
