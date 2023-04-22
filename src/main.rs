use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal, Result,
};
use nvml_wrapper::Nvml;
use std::io::{stdout, Write};
use sysinfo::{System, SystemExt};
mod bar;
mod cpu;
mod frame;
mod memory;
mod model;
mod nvidia;
mod pie_chart;

pub const EDGE: u16 = 4;
pub const RADIUS: u16 = 10;
pub const X: u16 = 10;
pub const Y_INIT: u16 = 1;
const REFRESH: u64 = 500;

fn main() -> Result<()> {
    let mut sys = System::new();
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();
    let cpu_model = model::get_cpu_model().unwrap();

    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    terminal::enable_raw_mode()?;
    loop {
        queue!(
            stdout,
            cursor::MoveTo(crate::X, Y_INIT),
            Print(format!("{}", cpu_model))
        )?;
        let mut y: u16 = Y_INIT + 1;

        y = cpu::display_cpu_info(&mut sys, &mut stdout, &mut y)?;
        y += 1;
        let mut mem_y = y.clone();
        _ = memory::display_memory_info(&mut sys, &mut stdout, &mut y)?;
        _ = nvidia::display_gpu_info(&device, &mut stdout, &mut mem_y)?;
        frame::draw_frame(&mut stdout)?;

        stdout.flush().unwrap();

        if event::poll(std::time::Duration::from_millis(REFRESH))? {
            let evt = event::read()?;

            if let Event::Key(key) = evt {
                if key.code == KeyCode::Char('q') {
                    queue!(
                        stdout,
                        terminal::Clear(terminal::ClearType::All),
                        cursor::MoveTo(0, 0)
                    )?;
                    break;
                }
            }
        }
    }

    stdout.flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}
