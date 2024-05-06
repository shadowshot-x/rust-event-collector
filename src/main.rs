use std::fs::{self, read_to_string, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::string;
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use serde::Deserialize;


// Picked up from env variables
const CLUSTER_NAME: &str = "cluster0";
const NAMESPACE_NAME: &str = "namespace0";
const POD_NAME: &str = "pod0";
const CONTAINER_NAME: &str = "container0";

#[derive(Debug, Deserialize)]
struct Config{
    counter_file : String,
    aggregate_file : String,
    tail_folder : String,
    audit_file: String,
    flush: FlushFolder
}

#[derive(Debug, Deserialize)]
struct FlushFolder {
    mode : String,
    location: String,
    rotation_threshold: i32
}

fn record_log_counter(counter: &i32, file_name: &String, counter_file: &String) {
    let mut counter_file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(counter_file)
    .expect("Failed to open file");
    let counter_string = file_name.to_owned() + " - " + &counter.to_string() + "\n";
    counter_file.write_all(counter_string.as_bytes()).expect("Failed to write to file");

    println!("Recorded Counter: {}", counter);
}

fn record_log_line(log_line: &string::String, aggregate_file: &String) {

    let mut log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(aggregate_file)
        .expect("Failed to open file");


    let now_utc = Utc::now();
    let log_enriched = format!("{} - {}\n", now_utc.to_string(), log_line);

    log_file.write(log_enriched.as_bytes()).expect("Failed to write to file");

    println!("Recorded Log Line: {}", log_line);
}

fn flush_log_file(f_name: &String, aggregate_file: &String, tail_folder: &String, audit_file: &String, output_folder: &String) {

    // Create a file with uuid in filter_folder.
    let orig_file_name = f_name.replace(tail_folder, "");
    let orig_file_name = orig_file_name.replace(".", "%2E");
    let file_path = &format!("{}{}%2F{}%2F{}%2F{}-{}-{}", output_folder,CLUSTER_NAME, NAMESPACE_NAME, POD_NAME, CONTAINER_NAME, orig_file_name ,Uuid::new_v4());
    println!("{}",file_path);
    let mut log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open file");


    let data = read_to_string(aggregate_file).expect("Unable to read file");

    log_file.write_all(data.as_bytes()).expect("Failed to write to file");
    fs::remove_file(aggregate_file).expect("Unable to delete file");

    // record flushed log file names to delete them from source
    let mut flushed_log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(audit_file)
        .expect("Failed to open file");

    flushed_log_file.write_all(( f_name.to_string() + " - " + &file_path.to_owned() + "\n").as_bytes()).expect("Failed to write to file");

    println!("Log File Flushed");

}

fn read_large_file(f_name: &String, f_counter: i32, aggregate_file: &String, tail_folder: &String, audit_file: &String, counter_file: &String, output_folder: &String, rotation_threshold: &i32) {
    let file = File::open(f_name).expect("Failed to Read file");

    let file_reader = BufReader::new(file);

    let mut counter = 0;
    for file_line in file_reader.lines() {
        // if counter > len(file_reader.lines()) {
        //     return
        // }
        if counter <= f_counter {
            counter +=1;
            continue;
        }



        let log_line = file_line.unwrap();

        counter +=1;
        record_log_line(&log_line, aggregate_file);
        record_log_counter(&counter, f_name, counter_file);

        if counter % rotation_threshold == 0 {
            flush_log_file(f_name, aggregate_file, tail_folder, audit_file, output_folder);
        }
    }

    if counter % rotation_threshold != 0 {
        flush_log_file(f_name, aggregate_file, tail_folder, audit_file, output_folder);
    }
}

fn thread_process(counter_map: &HashMap<String, i32>, counter_file: &String, aggregate_file: &String, tail_folder: &String, audit_file: &String, output_folder: &String, rotation_threshold: &i32) {
    let f_name = "./log_files/abc.txt".to_string();

    let f_counter = boot_thread(counter_map, &f_name);
    println!("{}", f_counter);
    read_large_file(&f_name, f_counter, aggregate_file, tail_folder, audit_file, counter_file, output_folder, rotation_threshold);

}

fn boot_thread(counter_map: &HashMap<String, i32>, f_name: &String) -> i32 {
    if counter_map.contains_key(f_name) {
        return *counter_map.get(f_name).unwrap();
    } else {
        return 0;
    }
}

fn record_counters(counter_file_name: &String) -> HashMap<String, i32> {
    let mut counter_map:HashMap<String, i32>  = HashMap::new();

    let file = File::open(counter_file_name).expect("Failed to Read file");

    let file_reader = BufReader::new(file);
    
    for file_line in file_reader.lines() {
        let line = file_line.unwrap();
            let parts: Vec<&str> = line.split(" - ").collect();
            let f_counter = parts[1].to_string().parse::<i32>().unwrap();
            if counter_map.contains_key(&parts[0].to_string()) {
                if counter_map.get(&parts[0].to_string()).unwrap() < &f_counter {
                    counter_map.insert(parts[0].to_string(), f_counter);
                }
            } else {
                counter_map.insert(parts[0].to_string(), f_counter);
            }
    }

    counter_map
}

fn config_unmarshall(config_file_location: &String) -> Config {
    let data = fs::read_to_string(config_file_location).expect("Unable to read config file");

    let conf_var: Config = serde_json::from_str(&data).unwrap();

    conf_var
}

fn main() {
    let config = config_unmarshall(&"./config/config.json".to_string());

    let counter_file = config.counter_file;
    let aggregate_file = config.aggregate_file;
    let tail_folder = config.tail_folder;
    let audit_file = config.audit_file;
    let output_folder = config.flush.location;
    let rotation_threshold = config.flush.rotation_threshold;

    let counter_map =record_counters(&counter_file);

    // let thread_process_future =  move |counter_map: HashMap<String, i32>, counter_file: &String, aggregate_file: &String, tail_folder: &String, audit_file: &String, output_folder: &String, rotation_threshold: &i32|{
    //     thread_process(counter_map, &counter_file, &aggregate_file, &tail_folder, &audit_file, &output_folder, &rotation_threshold);
    // };

    // let paths = fs::read_dir(&tail_folder).unwrap();
    // for path in paths {
    // }

    thread_process(&counter_map, &counter_file, &aggregate_file, &tail_folder, &audit_file, &output_folder, &rotation_threshold);
}
