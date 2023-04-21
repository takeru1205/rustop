use crate::bar;
use crate::bar::PreID;
use crate::memory::mem_to_mb;
use crossterm::{cursor, queue, style::Print, terminal, Result};
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
    let half_width = (width - crate::EDGE) / 2;

    let gpu_usage: Vec<String> = vec![
        device.fan_speed(0).unwrap().to_string(),
        memory_info.total.to_string(),
        memory_info.used.to_string(),
    ];

    // Display GPU Brand
    queue!(
        stdout,
        cursor::MoveTo(crate::X, *y),
        Print(format!(
            "NVIDIA {}",
            BrandDisplayWrapper(device.brand().unwrap()).to_string()
        ))
    )?;
    *y += 1;

    // Display GPU usage bar
    let used_gpu_mb = mem_to_mb(memory_info.used);
    let total_gpu_mb = mem_to_mb(memory_info.total);
    let gpu_usage_percentage = used_gpu_mb / total_gpu_mb * 100.;
    _ = &mut bar::display_usage_bar(
        gpu_usage_percentage as f32,
        0,
        stdout,
        y,
        PreID::DispName("VRAM".to_string()),
    )
    .unwrap();
    queue!(
        stdout,
        cursor::MoveTo(crate::EDGE + half_width, *y),
        Print(format!(
            "{0: >10} MB / {1: >10} MB",
            used_gpu_mb, total_gpu_mb
        ))
    )
    .unwrap();
    *y += 1;

    // TODO: Display Fan speed bar

    let gpu_use_case: Vec<String> = vec![
        "Fan Speed: ".to_string(),
        "Total GPU Memory: ".to_string(),
        "Used GPU Memory: ".to_string(),
    ];

    for (case, usage) in gpu_use_case.iter().zip(gpu_usage.iter()) {
        queue!(
            stdout,
            cursor::MoveTo(crate::X, *y),
            Print(format!("{}: {}", case, usage))
        )
        .unwrap();
        *y += 1;
    }

    Ok(*y)
}
