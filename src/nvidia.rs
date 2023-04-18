use crossterm::{cursor, queue, style::Print, Result};
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

    let gpu_usage: Vec<String> = vec![
        BrandDisplayWrapper(device.brand().unwrap()).to_string(),
        device.fan_speed(0).unwrap().to_string(),
        memory_info.total.to_string(),
        memory_info.used.to_string(),
    ];

    let gpu_use_case: Vec<String> = vec![
        "Brand: ".to_string(),
        "Fan Speed: ".to_string(),
        "Total GPU Memory: ".to_string(),
        "Used GPU Memory: ".to_string(),
    ];

    for (case, usage) in gpu_use_case.iter().zip(gpu_usage.iter()) {
        queue!(
            stdout,
            cursor::MoveTo(crate::X, *y),
            Print(format!("{}: {}", case, usage))
        )?;
        *y += 1;
    }

    Ok(*y)
}
