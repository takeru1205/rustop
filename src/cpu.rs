use crossterm::{
    cursor, queue,
    style::{self, Print, Stylize},
    Result,
};
use std::io::Write;
use sysinfo::{CpuExt, System, SystemExt};

pub fn display_cpu_info(sys: &mut System, stdout: &mut impl Write, y: &mut u16) -> Result<u16> {
    sys.refresh_cpu(); // Refreshing CPU information.

    for cpu in sys.cpus().iter() {
        // let cpu_usage: u16 = (cpu.cpu_usage() / (crate::WIDTH as f32) * 100.0) as u16;
        let cpu_usage: u16 = cpu.cpu_usage() as u16;
        // Change color bar depends on usage
        let print_style: style::PrintStyledContent<&str>;
        if cpu_usage < 40 {
            print_style = style::PrintStyledContent("█".green());
        } else if cpu_usage < 80 {
            print_style = style::PrintStyledContent("█".yellow());
        } else {
            print_style = style::PrintStyledContent("█".red());
        }

        // Print usage bar
        for x in crate::X..(crate::X + 100) {
            if x <= cpu_usage {
                queue!(stdout, cursor::MoveTo(x, *y), print_style).unwrap();
            } else {
                queue!(stdout, cursor::MoveTo(x, *y), Print(" ")).unwrap();
            }
        }

        // Debug to print cpu usage
        queue!(
            stdout,
            cursor::MoveTo(3, *y),
            Print(format!("{: >02}", cpu_usage))
        )
        .unwrap();

        *y += 1;
    }

    Ok(*y)
}
