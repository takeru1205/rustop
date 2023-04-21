use crate::bar;
use crate::bar::PreID;
use crossterm::{cursor, queue, style::Print, Result};
use std::io::Write;
use sysinfo::{System, SystemExt};

pub fn display_memory_info(sys: &mut System, stdout: &mut impl Write, y: &mut u16) -> Result<u16> {
    sys.refresh_memory(); // Refreshing memory information.
    let memory_usage: Vec<u64> = vec![
        sys.total_memory(),
        sys.used_memory(),
        sys.total_swap(),
        sys.used_swap(),
    ];

    let memory_use_case: Vec<String> = vec![
        "total memory".to_string(),
        "used memory".to_string(),
        "total swap".to_string(),
        "used swap".to_string(),
    ];

    let memory_usage_percentage =
        mem_to_gb(sys.used_memory()) / mem_to_gb(sys.total_memory()) * 100.;
    let swap_usage_percentage = mem_to_gb(sys.used_swap()) / mem_to_gb(sys.total_swap()) * 100.;
    _ = &mut bar::display_usage_bar(
        memory_usage_percentage as f32,
        0,
        stdout,
        y,
        PreID::DispName("RAM".to_string()),
    )
    .unwrap();
    _ = &mut bar::display_usage_bar(
        swap_usage_percentage as f32,
        1,
        stdout,
        y,
        PreID::DispName("Swap".to_string()),
    )
    .unwrap();
    *y += 1;

    for (case, usage) in memory_use_case.iter().zip(memory_usage.iter()) {
        queue!(
            stdout,
            cursor::MoveTo(crate::X, *y),
            Print(format!("{}: {} bytes", case, usage))
        )?;
        *y += 1;
    }

    queue!(
        stdout,
        cursor::MoveTo(crate::X, *y),
        Print(format!("{}", memory_usage_percentage))
    )?;
    *y += 1;
    Ok(*y)
}

fn mem_to_gb(mem: u64) -> f64 {
    (mem * 1024) as f64 / (1024. * 1024. * 1024.)
}
