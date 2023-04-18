use crossterm::{cursor, queue, style::Print, Result};
use std::io::Write;
use sysinfo::{CpuExt, System, SystemExt};

const X: u16 = 10; // Left end line
const Y_INIT: u16 = 10; // Start line

pub fn display_cpu_info(sys: &mut System, stdout: &mut impl Write) -> Result<()> {
    let y = Y_INIT;

    sys.refresh_cpu(); // Refreshing CPU information.
    let cpu_usage_oneline = sys
        .cpus()
        .iter()
        .map(|cpu| format!("{:.2} ", cpu.cpu_usage()))
        .collect::<String>();

    queue!(
        stdout,
        cursor::MoveTo(X, y),
        Print(format!("CPU Usage: {}", cpu_usage_oneline))
    )
}
