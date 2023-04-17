use std::io::{self, Write};
use sysinfo::{CpuExt, System, SystemExt};

const REFRESH: u64 = 500; // Frequency to get information
fn main() {
    let mut sys = System::new();

    for _ in 0..10 {
        let mut cpu_usage_oneline = String::new();
        // Refreshing CPU information
        sys.refresh_cpu();
        // Concatenate CPU Usage to oneline string
        for cpu in sys.cpus() {
            cpu_usage_oneline.push_str(&format!("{:.2} ", cpu.cpu_usage()));
        }
        print!("\rCPU Usage: {}", cpu_usage_oneline);
        io::stdout().flush().unwrap();
        // Get Informations per REFRESH
        std::thread::sleep(std::time::Duration::from_millis(REFRESH));
    }

    println!();
}
