use crossterm::{
    cursor, execute, queue,
    style::{self, Print, Stylize},
    terminal, Result,
};
use nvml_wrapper::enum_wrappers::device::Brand;
use nvml_wrapper::Nvml;
use std::fmt;
use std::io::{stdout, Write};
use sysinfo::{CpuExt, System, SystemExt};

const REFRESH: u64 = 500; // Frequency to get information
const X: u16 = 10; // Left end line
const Y_INIT: u16 = 10; // Start line

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

fn main() -> Result<()> {
    let mut sys = System::new();
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();

    // Refresh the default terminal
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    for _ in 0..10 {
        display_cpu_info(&mut sys, &mut stdout)?;
        display_memory_info(&mut sys, &mut stdout)?;
        display_gpu_info(&device, &mut stdout)?;
        draw_frame(&mut stdout)?;

        stdout.flush().unwrap();

        // Sleep between updates
        std::thread::sleep(std::time::Duration::from_millis(REFRESH));
    }
    stdout.flush()?;
    Ok(())
}

fn display_cpu_info(sys: &mut System, stdout: &mut impl Write) -> Result<()> {
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

fn display_memory_info(sys: &mut System, stdout: &mut impl Write) -> Result<()> {
    let mut y = Y_INIT + 1;

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
            cursor::MoveTo(X, y),
            Print(format!("{}: {} bytes", case, usage))
        )?;
        y += 1;
    }
    Ok(())
}

fn display_gpu_info(device: &nvml_wrapper::Device, stdout: &mut impl Write) -> Result<()> {
    let mut y = Y_INIT + 5;
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
            cursor::MoveTo(X, y),
            Print(format!("{}: {}", case, usage))
        )?;
        y += 1;
    }

    Ok(())
}

fn draw_frame(stdout: &mut impl Write) -> Result<()> {
    let width = 150;
    let height = 40;

    for y in 0..height {
        for x in 0..width {
            if (y == 0 || y == height - 1) || (x == 0 || x == width - 1) {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent("█".blue())
                )?;
            }
        }
    }
    Ok(())
}
