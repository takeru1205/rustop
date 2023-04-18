use crossterm::{cursor, queue, style::Print, Result};
use std::io::Write;
use sysinfo::{System, SystemExt};

pub fn display_memory_info(sys: &mut System, stdout: &mut impl Write, y: &mut u16) -> Result<u16> {
    // let mut y = Y_INIT + 1;

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

    for (case, usage) in memory_use_case.iter().zip(memory_usage.iter()) {
        queue!(
            stdout,
            cursor::MoveTo(crate::X, *y),
            Print(format!("{}: {} bytes", case, usage))
        )?;
        *y += 1;
    }
    Ok(*y)
}
