use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::{Instant, Duration};

const PROCESS_NAME: &str = "vb-rec-v2";
const REPLACE_WITH_STABLE_VERSION_FREQ: u32 = 3;
const REPLACE_WITH_STABLE_VERSION_DURATION: Duration = Duration::from_secs(3600 * 24);

fn replace_with_stable_version() {
    println!("replacing...");
}

fn main() {
    let mut crash_count = 0;
    let mut start_time = Instant::now();

    while true {
        // let mut is_running = false;

        // Checking if vb-recorder-v2 process is running
        // let output = Command::new("ps")
        //     .arg("-a")
        //     .output()
        //     .expect("failed to execute process");

        // let output = String::from_utf8(output.stdout).expect("invalid utf8");
        // let output: Vec<&str> = output.split(" ").collect();

        // for key in output {
        //     let key: Vec<&str> = key.split("\n").collect();
        //     if key[0] == PROCESS_NAME {
        //         is_running = true
        //     }
        // }

        // Starting vb-recorder-v2 if it isn't running
        // if !is_running {
        //     println!("Starting {PROCESS_NAME}");

        //     let output = Command::new(format!("/home/raahim/Dev/vb-rec-v2/src/{PROCESS_NAME}"))
        //         .output()
        //         .expect("failed to execute process");
        // }

        println!("VB-Updater: Starting {PROCESS_NAME}\n");

        let mut output = Command::new(format!("/home/raahim/Dev/vb-rec-v2/src/{PROCESS_NAME}"))
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
      
        let stdout = output.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            let line = line.unwrap();

            println!("{}", line);

            if line == "RECORDER_RESTART_REQUESTED" {
                println!("\nVB-Updater: RECORDER_RESTART_REQUESTED - Restarting VB-Recorder\n");
            } else if line == "RECORDER_STOP_REQUESTED" {
                println!("\nVB-Updater: RECORDER_STOP_REQUESTED - Stopping VB-Recorder\n");
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
        crash_count += 1;
        println!("VB-Updater: Crash Count in last 24 hours: {crash_count}");

        if crash_count == REPLACE_WITH_STABLE_VERSION_FREQ {
            println!("VB-Updater: Replacing {PROCESS_NAME} with stable version...");

            replace_with_stable_version();

            crash_count = 0;
            start_time = Instant::now();
        }
    }
}
