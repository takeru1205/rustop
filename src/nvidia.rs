use crate::memory::mem_to_mb;
use crate::pie_chart;
use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor},
    terminal, Result,
};
use nvml_wrapper::enum_wrappers::device::Brand;
use std::fmt;
use std::io::Write;

// Wrapper for Brand enum
pub struct BrandDisplayWrapper(pub Brand);

// Display trait for Brand
impl fmt::Display for BrandDisplayWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Brand::Quadro => write!(f, "Quadro"),
            Brand::Tesla => write!(f, "Tesla"),
            Brand::Titan => write!(f, "Titan"),
            Brand::TitanRTX => write!(f, "TitanRTX"),
            Brand::GeForce => write!(f, "GeForce"),
            Brand::GeForceRTX => write!(f, "GeForceRTX"),
            Brand::Nvidia => write!(f, "Nvidia"),
            Brand::NvidiaRTX => write!(f, "NvidiaRTX"),
            _ => write!(f, "Unknown"),
        }
    }
}

pub fn display_gpu_info(
    device: &nvml_wrapper::Device,
    stdout: &mut impl Write,
    y: &mut u16,
) -> Result<u16> {
    let memory_info = device.memory_info().unwrap();
    let (width, _) = terminal::size().unwrap();
    let third_width = (width - crate::EDGE) / 3;
    // Display GPU Brand
    queue!(
        stdout,
        cursor::MoveTo(crate::EDGE + third_width * 2 + 3, *y),
        Print(format!(
            "NVIDIA {}",
            BrandDisplayWrapper(device.brand().unwrap()).to_string()
        ))
    )?;

    // Draw pie chart
    let used_gpu_mb = mem_to_mb(memory_info.used);
    let total_gpu_mb = mem_to_mb(memory_info.total);
    *y += 1;

    *y = pie_chart::display_pie_chart(
        stdout,
        y,
        &mut vec![used_gpu_mb as usize, (total_gpu_mb - used_gpu_mb) as usize],
        2,
    )
    .unwrap();

    *y += 1 + crate::RADIUS;

    queue!(
        stdout,
        cursor::MoveTo(crate::EDGE + third_width * 2 + 3, *y),
        SetForegroundColor(Color::White),
        Print(format!(
            "{0: >10} MB / {1: >10} MB",
            used_gpu_mb, total_gpu_mb
        ))
    )
    .unwrap();

    // TODO: Display Fan speed bar

    Ok(*y)
}
