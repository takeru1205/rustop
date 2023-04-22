use crate::pie_chart;
use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor},
    terminal, Result,
};
use std::io::Write;
use sysinfo::{System, SystemExt};

pub fn display_memory_info(sys: &mut System, stdout: &mut impl Write, y: &mut u16) -> Result<u16> {
    sys.refresh_memory(); // Refreshing memory information.
    let (width, _) = terminal::size().unwrap();
    let third_width = (width - crate::EDGE) / 3;

    // Display memory usage info
    let used_mem_mb = mem_to_mb(sys.used_memory());
    let total_mem_mb = mem_to_mb(sys.total_memory());
    // Display Swap usage info
    let used_swap_mb = mem_to_mb(sys.used_swap());
    let total_swap_mb = mem_to_mb(sys.total_swap());

    *y += 1;
    // Display Pie Chart
    // Display Memory usage
    *y = pie_chart::display_pie_chart(
        stdout,
        y,
        &mut vec![used_mem_mb as usize, (total_mem_mb - used_mem_mb) as usize],
        0,
    )
    .unwrap();

    // Display swap usage
    *y = pie_chart::display_pie_chart(
        stdout,
        y,
        &mut vec![
            used_swap_mb as usize,
            (total_swap_mb - used_swap_mb) as usize,
        ],
        1,
    )
    .unwrap();
    *y += 1 + crate::RADIUS;

    queue!(
        stdout,
        cursor::MoveTo(crate::EDGE + 3, *y),
        SetForegroundColor(Color::White),
        Print(format!(
            "{0: >10} MB / {1: >10} MB",
            used_mem_mb, total_mem_mb
        ))
    )
    .unwrap();

    queue!(
        stdout,
        cursor::MoveTo(crate::EDGE + third_width + 3, *y),
        SetForegroundColor(Color::White),
        Print(format!(
            "{0: >10} MB / {1: >10} MB",
            used_swap_mb, total_swap_mb
        ))
    )
    .unwrap();

    *y += 1;

    Ok(*y)
}

// FIXME
pub fn mem_to_mb(mem: u64) -> f64 {
    (mem * 1024) as f64 / (1024. * 1024.)
}
