use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const PROCESS_NAME: &str = "vb-recorder";
const PROCESS_PATH: &str = "./main";
const REPLACE_WITH_STABLE_VERSION_FREQ: u32 = 3;
const REPLACE_WITH_STABLE_VERSION_DURATION: Duration = Duration::from_secs(3600 * 24);

fn kill_process(id: u32) {
    if cfg!(target_os = "windows") {
        let _ = Command::new(format!("taskkill"))
            .args(["/F", "/PID", &id.to_string()])
            .output()
            .expect("failed to execute process");
    } else {
        // Linux or MacOS
        let _ = Command::new(format!("kill"))
            .args(["-9", &id.to_string()])
            .output()
            .expect("failed to execute process");
    }
}

fn replace_with_stable_version() {
    // TODO: Implement replacing functionality
    println!("replacing...");
}

fn main() {
    let mut crash_count = 0;
    let mut start_time = Instant::now();
    let mut has_crashed = true; // This variable becomes false when RECORDER_RESTART_REQUESTED or RECORDER_STOP_REQUESTED - so it's only true when process has crashed
    let mut should_run = true;

    while should_run {
        println!("VB-Updater: Starting {PROCESS_NAME}\n");

        // Starting process
        let mut output = Command::new(PROCESS_PATH)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        let id = output.id();

        // Reading output from process
        let stdout = output.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            let line = line.unwrap();
            println!("{}", line);

            if line == "RECORDER_RESTART_REQUESTED" {
                println!("\nVB-Updater: RECORDER_RESTART_REQUESTED - Restarting {PROCESS_NAME}");

                // Resetting values
                crash_count = 0;
                start_time = Instant::now();

                has_crashed = false;

                // Killing process
                kill_process(id);
            } else if line == "RECORDER_STOP_REQUESTED" {
                println!("\nVB-Updater: RECORDER_STOP_REQUESTED - Stopping {PROCESS_NAME}");

                should_run = false;
                has_crashed = false;

                // Killing process
                kill_process(id);
            }

            // If duration since start has passed 24 hours, then reset crash_count & start_time
            let duration = start_time.elapsed();
            if duration >= REPLACE_WITH_STABLE_VERSION_DURATION {
                println!("\nVB-Updater: 24 hours passed since start... Resetting crash_count\n");

                crash_count = 0;
                start_time = Instant::now();
            }
        }

        println!("\nVB-Updater: {PROCESS_NAME} has stopped running...");

        if has_crashed {
            crash_count += 1;
            println!("VB-Updater: Crash Count in last 24 hours is {crash_count}");
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
