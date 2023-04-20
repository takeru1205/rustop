use std::process::Command;
use std::str;

/// Retrieves the CPU model name by executing the command 'lscpu | grep "Model name"'.
/// Returns the CPU model name as a `String`.
pub fn get_cpu_model() -> Result<String, ()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lscpu | grep 'Model name'")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                match str::from_utf8(&output.stdout) {
                    Ok(result) => Ok(result
                        .to_string()
                        .split_whitespace()
                        .skip(2)
                        .collect::<Vec<_>>()
                        .join(" ")),
                    Err(_) => {
                        eprintln!("Error: Failed to convert output to UTF-8");
                        Err(())
                    }
                }
            } else {
                eprintln!("Error: lscpu command failed");
                Err(())
            }
        }
        Err(_) => {
            eprintln!("Error: Failed to run lscpu command");
            Err(())
        }
    }
}
