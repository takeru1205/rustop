use crossterm::{cursor, queue, style::Print, Result};
use std::io::Write;
use sysinfo::{CpuExt, System, SystemExt};

pub fn display_cpu_info(sys: &mut System, stdout: &mut impl Write, y: &mut u16) -> Result<u16> {
    sys.refresh_cpu(); // Refreshing CPU information.
    let cpu_usage_oneline = sys
        .cpus()
        .iter()
        .map(|cpu| format!("{:.2} ", cpu.cpu_usage()))
        .collect::<String>();

    queue!(
        stdout,
        cursor::MoveTo(crate::X, *y),
        Print(format!("CPU Usage: {}", cpu_usage_oneline))
    )
    .unwrap();

    *y += 1;
    Ok(*y)
}
