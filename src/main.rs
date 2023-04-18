use crossterm::{execute, terminal, Result};
use nvml_wrapper::Nvml;
use std::io::{stdout, Write};
use sysinfo::{System, SystemExt};
mod cpu;
mod frame;
mod memory;
mod nvidia;

const REFRESH: u64 = 500; // Frequency to get information

fn main() -> Result<()> {
    let mut sys = System::new();
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();

    // Refresh the default terminal
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    for _ in 0..10 {
        cpu::display_cpu_info(&mut sys, &mut stdout)?;
        memory::display_memory_info(&mut sys, &mut stdout)?;
        nvidia::display_gpu_info(&device, &mut stdout)?;
        frame::draw_frame(&mut stdout)?;

        stdout.flush().unwrap();

        // Sleep between updates
        std::thread::sleep(std::time::Duration::from_millis(REFRESH));
    }
    stdout.flush()?;
    Ok(())
}
