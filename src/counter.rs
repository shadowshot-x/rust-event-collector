use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;

// Record the counters to restart from the correct place.
pub fn record_counters(counter_file_name: &String) -> HashMap<String, i32> {
    let mut counter_map:HashMap<String, i32>  = HashMap::new();

    // for each counter file, read each line and update counter map based on file name
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

// Fetch location from config hashmap
pub fn fetch_file_counter(counter_map: &HashMap<String, i32>, f_name: &String) -> i32 {
    if counter_map.contains_key(f_name) {
        return *counter_map.get(f_name).unwrap();
    } else {
        return 0;
    }
}