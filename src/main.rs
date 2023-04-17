use crossterm::{
    cursor, execute, queue,
    style::{self, Print, Stylize},
    terminal, Result,
};
use std::io::{stdout, Write};
use sysinfo::{CpuExt, System, SystemExt};

const REFRESH: u64 = 500; // Frequency to get information
const X: u16 = 10; // Left end line

fn main() -> Result<()> {
    let mut sys = System::new();
    let mut stdout = stdout();
    // Refresh the default terminal
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    for _ in 0..10 {
        let mut y: u16 = 10;
        // Cursor move to position
        queue!(stdout, cursor::MoveTo(X, y)).unwrap();
        y += 1;

        // Refreshing all informations
        sys.refresh_all();
        // Get CPU Information
        let mut cpu_usage_oneline = String::new();
        for cpu in sys.cpus() {
            cpu_usage_oneline.push_str(&format!("{:.2} ", cpu.cpu_usage()));
        }
        // Print the CPU usage and move to the next line
        queue!(
            stdout,
            cursor::MoveTo(X, y),
            Print(format!("{}", cpu_usage_oneline))
        )
        .unwrap();
        y += 1;

        // print memory usage
        queue!(
            stdout,
            cursor::MoveTo(X, y),
            Print(format!("total memory: {} bytes", sys.total_memory()))
        )
        .unwrap();
        y += 1;
        queue!(
            stdout,
            cursor::MoveTo(X, y),
            Print(format!("used memory : {} bytes", sys.used_memory()))
        )
        .unwrap();
        y += 1;
        queue!(
            stdout,
            cursor::MoveTo(X, y),
            Print(format!("total swap  : {} bytes", sys.total_swap()))
        )
        .unwrap();
        y += 1;
        queue!(
            stdout,
            cursor::MoveTo(X, y),
            Print(format!("used swap   : {} bytes", sys.used_swap()))
        )
        .unwrap();

        // Print blue frame
        for y in 0..40 {
            for x in 0..150 {
                if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                    queue!(
                        stdout,
                        cursor::MoveTo(x, y),
                        style::PrintStyledContent("█".blue())
                    )?;
                }
            }
        }

        stdout.flush().unwrap();

        // Sleep between updates
        std::thread::sleep(std::time::Duration::from_millis(REFRESH));
    }
    stdout.flush()?;
    Ok(())
}
