use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

mod config;
mod counter;
mod util;
mod flush;


fn read_large_file(f_name: &String, f_counter: i32, aggregate_file: &String, tail_folder: &String, audit_file: &String, counter_file: &String, output_folder: &String, rotation_threshold: &i32) {
    // Open the file to tail
    let file = File::open(f_name).expect("Failed to Read file");

    // Reaf the records line by line and increment the counter
    let file_reader = BufReader::new(file);

    let mut counter = 0;
    for file_line in file_reader.lines() {
        if counter < f_counter {
            counter +=1;
            continue;
        }

        let log_line = file_line.unwrap();

        counter +=1;

        // Add log line to aggregate file
        util::record_log_line(&log_line, aggregate_file);

        // Record file counter in the counter.txt
        util::record_log_counter(&counter, f_name, counter_file);

        // If we exceed the rotation threshold flush log file
        if counter % rotation_threshold == 0 {
            flush::flush_log_file(f_name, aggregate_file, tail_folder, audit_file, output_folder);
        }
    }

    // handle left log lines at end of file
    if counter % rotation_threshold != 0 {
        flush::flush_log_file(f_name, aggregate_file, tail_folder, audit_file, output_folder);
    }
}

fn thread_process(counter_map: &HashMap<String, i32>, counter_file: &String, aggregate_file: &String, tail_file_name: &String, tail_folder: &String, audit_file: &String, output_folder: &String, rotation_threshold: &i32) {
    // Calculate the path of the file to be tailed.
    let f_name = tail_folder.to_owned() + tail_file_name;

    // Counter to start from the correct place.
    let f_counter = counter::fetch_file_counter(counter_map, &f_name);
    println!("COUNTER LOCATION FOR FILE - {} is {}", f_name, f_counter);

    // Start reading the log file
    read_large_file(&f_name, f_counter, aggregate_file, tail_folder, audit_file, counter_file, output_folder, rotation_threshold);
}

// Start the processing
fn main() {
    // use config module to read the config file and get the metadata.
    let config = config::config_unmarshall(&"./config/config.json".to_string());

    // we have different files to record the counters to restart from the correct place.
    let counter_file = config.counter_file;
    let aggregate_file = config.aggregate_file;
    let tail_folder = config.tail_folder;
    let audit_file = config.audit_file;
    let output_folder = config.flush.location;
    let rotation_threshold = config.flush.rotation_threshold;

    // to restart multiple counters from the correct place, we maintain a hashmap of file name and counter.
    let counter_map =counter::record_counters(&counter_file);

    // read all the files in the tail folder and start processing them.
    let paths = fs::read_dir(&tail_folder).unwrap();
    for path in paths {
        let tail_file_name = path.unwrap().path().file_name().unwrap().to_string_lossy().into_owned();

        // start the thread to process the file.
        thread_process(&counter_map, &counter_file, &aggregate_file, &tail_file_name, &tail_folder, &audit_file, &output_folder, &rotation_threshold);
    }
}
