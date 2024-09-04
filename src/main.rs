use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

mod config;
mod counter;
mod util;
mod flush;

fn read_large_file(f_name: &String, f_counter: i32, aggregate_file: &String, tail_folder: &String, audit_file: &String, counter_file: &String, output_folder: &String, rotation_threshold: &i32, aggregate_counter_mutex: &Arc<Mutex<i32>>) {
    // Open the file to tail
    let file = File::open(f_name).expect("Failed to Read file");

    // Read the records line by line and increment the counter
    let file_reader = BufReader::new(file);

    let mut counter = 0;
    for file_line in file_reader.lines() {
        if counter < f_counter {
            counter +=1;
            continue;
        }

        let log_line = file_line.unwrap();

        counter +=1;
        let mut lock = aggregate_counter_mutex.lock().unwrap();

        *lock= *lock + 1;

        // Add log line to aggregate file
        util::record_log_line(&log_line, aggregate_file);

        // Record file counter in the counter.txt
        util::record_log_counter(&counter, f_name, counter_file);

        // If we exceed the rotation threshold flush log file
        if *lock % rotation_threshold == 0 {
            flush::flush_log_file(f_name, aggregate_file, tail_folder, audit_file, output_folder);
        }

        drop(lock)
    }

    if counter >= f_counter{
        return
    }

    // handle left log lines at end of file
    if counter % *rotation_threshold != 0 && counter > *rotation_threshold {
        flush::flush_log_file(f_name, aggregate_file, tail_folder, audit_file, output_folder);
    }
}

async fn thread_process(counter_map: &HashMap<String, i32>, counter_file: &String, aggregate_file: &String, tail_file_name: &String, tail_folder: &String, audit_file: &String, output_folder: &String, rotation_threshold: &i32, counter: &i32, aggregate_counter_mutex: &Arc<Mutex<i32>>) -> i32 {
    // Calculate the path of the file to be tailed.
    let f_name = tail_folder.to_owned() + tail_file_name;

    // Counter to start from the correct place.
    let f_counter = counter::fetch_file_counter(counter_map, &f_name);
    println!("COUNTER LOCATION FOR FILE - {} is {}", f_name, f_counter);

    // Start reading the log file
    read_large_file(&f_name, f_counter, aggregate_file, tail_folder, audit_file, counter_file, output_folder, rotation_threshold, aggregate_counter_mutex);

    return counter.clone();
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let config = config::config_unmarshall(&"./config/config.json".to_string());

    // we have different files to record the counters to restart from the correct place.
    let rotation_threshold = config.flush.rotation_threshold;

    // to restart multiple counters from the correct place, we maintain a hashmap of file name and counter.
    let counter_map =counter::record_counters(&config.counter_file);

    // read all the files in the tail folder and start processing them.
    let paths = fs::read_dir(&config.tail_folder).unwrap();

    let mut join_set = tokio::task::JoinSet::new();

    let mut counter = 0;

    let aggregate_file_counter = Arc::new(Mutex::new(0));

    for path in paths {
        let tail_file_name = path.unwrap().path().file_name().unwrap().to_string_lossy().into_owned();
        println!("Rust Map File {}", tail_file_name);
        let native_counter_map = counter_map.clone();

        let counter_file =  config.counter_file.clone();
        let aggregate_file =  config.aggregate_file.clone();
        let tail_folder =  config.tail_folder.clone();
        let audit_file =  config.audit_file.clone();
        let output_folder =  config.flush.location.clone();
        let aggregate_file_counter = aggregate_file_counter.clone();
        
        // start the thread to process the file.
        join_set.spawn(async move {
            thread_process(&native_counter_map, &counter_file, &aggregate_file, &tail_file_name, &tail_folder, &audit_file, &output_folder, &rotation_threshold, &counter, &aggregate_file_counter).await;
        });

        counter = counter + 1;
    }

    // let mut seen = [false; 10];
    while let Some(_) = join_set.join_next().await {
        println!("Process Complete in Join Handle");
    }

}