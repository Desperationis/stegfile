use crate::steglib::capacity::one_file_capacity;
use crate::steglib::split::Split;
use crate::steglib::util::write_data_to_file;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tempfile::TempDir;

const NUM_WORKERS: usize = 10;

fn steghide_embed(photo_path: &str, embedded_path: &str, passphrase: &str) {
    let _output = std::process::Command::new("steghide")
        .arg("embed")
        .arg("-cf")
        .arg(photo_path)
        .arg("-ef")
        .arg(embedded_path)
        .arg("-p")
        .arg(passphrase)
        .arg("-Z")
        .arg("-N")
        .arg("-K")
        .arg("-e")
        .arg("none")
        .output()
        .expect("Command failed to start");

    let stdout = String::from_utf8_lossy(&_output.stdout);
    let stderr = String::from_utf8_lossy(&_output.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    println!("Embedded {} into {}", embedded_path, photo_path);
}

/**
 * Embed data from a buffer into multiple files using the chosen split method.
*/
pub fn mul_embed<T: Split>(input_buffer: Vec<u8>, image_paths: &Vec<String>, passphrase: &str) {
    println!("Getting capacities of all images...");
    // Get max byte capacity of each image
    let mut capacities: Vec<u64> = Vec::new();
    for image in image_paths {
        capacities.push(one_file_capacity(image));
    }

    // Split content
    println!("Splitting file to different bins....");
    let split_content = T::split_to_bins(&input_buffer, &capacities);

    println!("Writing split files to memory...");
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let mut index: usize = 0;
    for mut bucket in split_content {
        // Prepend the bucket with its piece number in first 8 bytes
        let bytes = index.to_be_bytes();
        bucket.splice(0..0, bytes.iter().copied());

        println!("Writing to {:?}/file_part_{}", temp_dir, index);
        let temp_file = temp_path.join(format!("file_part_{}", index));
        let temp_file_path: &str = temp_file.to_str().unwrap();
        let _ = write_data_to_file(temp_file_path, bucket);
        index += 1;
    }

    // Embed each file piece with its associated image
    println!("Embedding each temp file to its file....");

    /////////////////////////////////////////
    let mut tmp: usize = 0;
    let mut paths: Vec<String> = Vec::new();
    for _ in 0..image_paths.len() {
        let file_path = temp_path.join(format!("file_part_{}", tmp));
        let file_path_str = file_path.to_str().unwrap().to_string();
        println!("{}", file_path_str);
        paths.push(file_path_str);
        tmp += 1;
    }

    // Create a channel for sending work items
    let (tx, rx) = mpsc::channel::<(String, String)>();
    let rx = Arc::new(Mutex::new(rx));
    let passphrase_mux = Arc::new(String::from(passphrase));

    // Create a vector to hold the worker threads
    let mut workers = Vec::with_capacity(NUM_WORKERS);

    // Create a thread pool
    for id in 0..NUM_WORKERS {
        let rx = Arc::clone(&rx);
        let shared_string_clone = Arc::clone(&passphrase_mux);

        let worker = thread::spawn(move || {
            loop {
                // Receive a string from the channel
                let work = rx.lock().unwrap().recv();

                match work {
                    Ok(string) => {
                        // Process the string (here we just print it)
                        println!("Worker {} received: {} {}", id, string.0, string.1);
                        steghide_embed(&string.1, &string.0, &shared_string_clone);
                    }
                    Err(_) => break, // Exit the loop if the channel is closed
                }
            }
        });

        workers.push(worker);
    }

    // Send some strings to be processed
    for (i, value) in image_paths.iter().enumerate() {
        tx.send((paths[i].clone(), value.clone())).unwrap();
    }

    // Drop the sender so that workers will stop after processing all tasks
    drop(tx);

    // Wait for all worker threads to finish
    for worker in workers {
        worker.join().unwrap();
    }

    /////////////////////////////////////////////
    println!("Done!");
}
