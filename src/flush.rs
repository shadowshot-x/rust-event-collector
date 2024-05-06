use std::fs::{self, read_to_string, OpenOptions};
use std::io::Write;
use uuid::Uuid;

// Picked up from env variables
const CLUSTER_NAME: &str = "cluster0";
const NAMESPACE_NAME: &str = "namespace0";
const POD_NAME: &str = "pod0";
const CONTAINER_NAME: &str = "container0";

pub fn flush_log_file(f_name: &String, aggregate_file: &String, tail_folder: &String, audit_file: &String, output_folder: &String) {
    let data = read_to_string(aggregate_file);
    let data_string = match data {
        Ok(data) => data,
        Err(_) => {
            println!("File is already parsed");
            return
        },
    };

    // Create a file with uuid in filter_folder.
    let orig_file_name = f_name.replace(tail_folder, "");
    let orig_file_name = orig_file_name.replace(".", "%2E");
    let file_path = &format!("{}{}%2F{}%2F{}%2F{}-{}-{}", output_folder, CLUSTER_NAME,  NAMESPACE_NAME,  POD_NAME, CONTAINER_NAME, orig_file_name ,Uuid::new_v4());

    let mut log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open file");


    log_file.write_all(data_string.as_bytes()).expect("Failed to write to file");
    fs::remove_file(aggregate_file).expect("Unable to delete file");

    // record flushed log file names to delete them from source
    let mut flushed_log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(audit_file)
        .expect("Failed to open file");

    flushed_log_file.write_all(( f_name.to_string() + " - " + &file_path.to_owned() + "\n").as_bytes()).expect("Failed to write to file");

    println!("Log File Flushed - {}", file_path);

}