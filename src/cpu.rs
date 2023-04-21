use crate::bar;
use crate::bar::PreID;
use crossterm::Result;
use std::io::Write;
use sysinfo::{CpuExt, System, SystemExt};

pub fn display_cpu_info(sys: &mut System, stdout: &mut impl Write, y: &mut u16) -> Result<u16> {
    sys.refresh_cpu(); // Refreshing CPU information.

    let cpus = sys.cpus();
    let mut counter: u16 = 0;

    // Display CPU usage bars in 2 columns
    for chunk in cpus.chunks(2) {
        for (index, cpu) in chunk.iter().enumerate() {
            let cpu_usage_percentage = cpu.cpu_usage() as f32;
            _ = &mut bar::display_usage_bar(
                cpu_usage_percentage,
                index as u16,
                stdout,
                y,
                PreID::Num(counter),
            )
            .unwrap();
            counter += 1;
        }
        *y += 1;
    }
    Ok(*y)
}
