use crossterm::{cursor, execute, queue, style::Print, terminal, Result};
// use crossterm::{execute, terminal, Result};
use nvml_wrapper::Nvml;
use std::io::{stdout, Write};
use sysinfo::{System, SystemExt};
mod bar;
mod cpu;
mod frame;
mod memory;
mod model;
mod nvidia;

pub const EDGE: u16 = 4; // Edge of frame
pub const X: u16 = 10; // Left end line
pub const Y_INIT: u16 = 1; // Start line
const REFRESH: u64 = 500; // Frequency to refresh information

fn main() -> Result<()> {
    let mut sys = System::new();
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();
    let cpu_model = model::get_cpu_model().unwrap();

    // Refresh the default terminal
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    for _ in 0..10 {
        // Display the CPU model
        queue!(
            stdout,
            cursor::MoveTo(crate::X, Y_INIT),
            Print(format!("{}", cpu_model))
        )?;
        let mut y: u16 = Y_INIT + 1;

        y = cpu::display_cpu_info(&mut sys, &mut stdout, &mut y)?;
        y = memory::display_memory_info(&mut sys, &mut stdout, &mut y)?;
        _ = nvidia::display_gpu_info(&device, &mut stdout, &mut y)?;
        frame::draw_frame(&mut stdout)?;

        // Refresh the terminal
        stdout.flush().unwrap();

        // Sleep between updates
        std::thread::sleep(std::time::Duration::from_millis(REFRESH));
    }
    stdout.flush()?;
    println!("");
    Ok(())
}
