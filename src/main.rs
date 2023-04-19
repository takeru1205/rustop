use crossterm::{cursor, execute, queue, style::Print, terminal, Result};
use nvml_wrapper::Nvml;
use std::io::{stdout, Write};
use sysinfo::{System, SystemExt};
mod cpu;
mod frame;
mod memory;
mod nvidia;

pub const WIDTH: u16 = 150; // Width of window
pub const HEIGHT: u16 = 40; // Height of window
pub const EDGE: u16 = 10; // Edge of frame
pub const X: u16 = 10; // Left end line
pub const Y_INIT: u16 = 1; // Start line
const REFRESH: u64 = 500; // Frequency to refresh information

fn main() -> Result<()> {
    let mut sys = System::new();
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();

    // Refresh the default terminal
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    let (width, _) = terminal::size().unwrap(); // get Terminal size

    for _ in 0..10 {
        let mut y: u16 = Y_INIT;
        queue!(
            stdout,
            cursor::MoveTo(crate::X, y),
            Print(format!("{:?}", terminal::size()))
        )
        .unwrap();
        y += 1;
        y = cpu::display_cpu_info(&mut sys, &mut stdout, &mut y, width)?;
        y = memory::display_memory_info(&mut sys, &mut stdout, &mut y)?;
        _ = nvidia::display_gpu_info(&device, &mut stdout, &mut y)?;
        frame::draw_frame(&mut stdout)?;

        stdout.flush().unwrap();

        // Sleep between updates
        std::thread::sleep(std::time::Duration::from_millis(REFRESH));
    }
    stdout.flush()?;
    Ok(())
}
