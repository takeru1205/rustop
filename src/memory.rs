use crate::bar;
use crate::bar::PreID;
use crossterm::{cursor, queue, style::Print, terminal, Result};
use std::io::Write;
use sysinfo::{System, SystemExt};

pub fn display_memory_info(sys: &mut System, stdout: &mut impl Write, y: &mut u16) -> Result<u16> {
    sys.refresh_memory(); // Refreshing memory information.
    let (width, _) = terminal::size().unwrap();
    let half_width = (width - crate::EDGE) / 2;
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

    // Display memory usage info
    let used_mem_mb = mem_to_mb(sys.used_memory());
    let total_mem_mb = mem_to_mb(sys.total_memory());
    let memory_usage_percentage = used_mem_mb / total_mem_mb * 100.;
    _ = &mut bar::display_usage_bar(
        memory_usage_percentage as f32,
        0,
        stdout,
        y,
        PreID::DispName("RAM".to_string()),
    )
    .unwrap();
    queue!(
        stdout,
        cursor::MoveTo(crate::EDGE + half_width, *y),
        Print(format!(
            "{0: >10} MB / {1: >10} MB",
            used_mem_mb, total_mem_mb
        ))
    )
    .unwrap();
    *y += 1;

    // Display Swap usage info
    let used_swap_mb = mem_to_mb(sys.used_swap());
    let total_swap_mb = mem_to_mb(sys.total_swap());
    let swap_usage_percentage = used_swap_mb / total_swap_mb * 100.;
    _ = &mut bar::display_usage_bar(
        swap_usage_percentage as f32,
        0,
        stdout,
        y,
        PreID::DispName("Swap".to_string()),
    )
    .unwrap();
    queue!(
        stdout,
        cursor::MoveTo(crate::EDGE + half_width, *y),
        Print(format!(
            "{0: >10} MB / {1: >10} MB",
            used_swap_mb, total_swap_mb
        ))
    )
    .unwrap();
    *y += 1;

    // Display infos
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

fn mem_to_mb(mem: u64) -> f64 {
    (mem * 1024) as f64 / (1024. * 1024.)
}
