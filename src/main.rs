use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::{Instant, Duration};

const PROCESS_NAME: &str = "main.exe";
const REPLACE_WITH_STABLE_VERSION_FREQ: u32 = 3;
const REPLACE_WITH_STABLE_VERSION_DURATION: Duration = Duration::from_secs(3600 * 24);

fn kill_process(id: u32) {
    if cfg!(target_os = "unix") {
        let _ = Command::new(format!("kill"))
                    .args(["-9", &id.to_string()])
                    .output()
                    .expect("failed to execute process");
    } else {
         let _ = Command::new(format!("taskkill"))
                    .args(["/F", "/PID", &id.to_string()])
                    .output()
                    .expect("failed to execute process");
    }
}

fn replace_with_stable_version() {
    println!("replacing...");
}

fn main() {
    let mut crash_count = 0;
    let mut start_time = Instant::now();
    let mut has_crashed = true;
    let mut should_run = true;

    while should_run {
        println!("VB-Updater: Starting {PROCESS_NAME}\n");

        let mut output = Command::new(format!("C:\\Users\\Administrator\\Desktop\\Dev\\vb-rec-v2-personal\\src\\main.exe"))
                                    .stdout(Stdio::piped())
                                    .spawn()
                                    .expect("failed to execute process");

        let id = output.id();

        println!("{id}");
            
        let stdout = output.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            let line = line.unwrap();

            println!("{}", line);

            if line == "RECORDER_RESTART_REQUESTED" {
                println!("\nVB-Updater: RECORDER_RESTART_REQUESTED - Restarting VB-Recorder");
                
                // Resetting values
                crash_count = 0;
                start_time = Instant::now();
                
                has_crashed = false;

                // Killing process
                kill_process(id);
            } else if line == "RECORDER_STOP_REQUESTED" {
                println!("\nVB-Updater: RECORDER_STOP_REQUESTED - Stopping VB-Recorder");

                should_run = false;
                has_crashed = false;
                
                // Killing process
                kill_process(id);
            }
            
            // If duration since start has passed 24 hours, reset crash_count & start_time
            let duration = start_time.elapsed();
            if duration >= REPLACE_WITH_STABLE_VERSION_DURATION  {
                println!("\nVB-Updater: 24 hours passed since start... Resetting crash_count\n");

                crash_count = 0;
                start_time = Instant::now();
            }
        }

        println!("\nVB-Updater: {PROCESS_NAME} has stopped running...");
        
        if has_crashed {
            crash_count += 1;
            println!("VB-Updater: Crash Count in last 24 hours: {crash_count}");
        } else {
            has_crashed = true;
        }

        if crash_count == REPLACE_WITH_STABLE_VERSION_FREQ {
            println!("VB-Updater: Replacing {PROCESS_NAME} with stable version...");

            replace_with_stable_version();

            crash_count = 0;
            start_time = Instant::now();
        }
    }
}
