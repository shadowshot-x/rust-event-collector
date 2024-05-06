use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

pub fn record_log_counter(counter: &i32, file_name: &String, counter_file: &String) {
    // add counter to counter.txt
    let mut counter_file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(counter_file)
    .expect("Failed to open file");
    let counter_string = file_name.to_owned() + " - " + &counter.to_string() + "\n";
    counter_file.write_all(counter_string.as_bytes()).expect("Failed to write to file");
}


pub fn record_log_line(log_line: &String, aggregate_file: &String) {
    // write log line to aggregate file
    let mut log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(aggregate_file)
        .expect("Failed to open file");


    let now_utc = Utc::now();
    let log_enriched = format!("{} - {}\n", now_utc.to_string(), log_line);

    log_file.write(log_enriched.as_bytes()).expect("Failed to write to file");
}


