use std::process::Command;
use std::io::Read;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;



fn storage_capacity(photo_path: &str) -> u64 {
    let _output = Command::new("steghide")
            .arg("--info")
            .arg(photo_path)
            .output()
            .expect("Command failed to start");

    let s = String::from_utf8_lossy(&_output.stdout);
    let capacity_line = s.lines()
        .find(|line| line.contains("capacity"))
        .expect("No capacity line found");

    let capacity_value = capacity_line.split(':')
        .nth(1)
        .expect("No value after colon")
        .trim();

    let mut parts = capacity_value.split_whitespace();
    let value_str = parts.next().expect("No value found");
    let prefix = parts.next().expect("No prefix found");
    let value: f64 = value_str.parse().expect("Failed to parse value");

    // Determine the multiplier based on the prefix
    let multiplier = match prefix {
        "Byte" => 1.0,
        "KB" => 1000.0,
        "MB" => 1_000_000.0,
        "GB" => 1_000_000_000.0,
        _ => 1.0, // Default multiplier if an unknown prefix is encountered
    };

    // Calculate the result by multiplying the value with the multiplier
    let result = (value * multiplier) as u64;

    result
}

fn embed_file(photo_path: &str, embedded_path: &str, passphrase: &str) {
    let _output = Command::new("steghide")
            .arg("embed")
            .arg("-cf")
            .arg(photo_path)
            .arg("-ef")
            .arg(embedded_path)
            .arg("-p")
            .arg(passphrase)
            .output()
            .expect("Command failed to start");


    println!("Embedded {} into {}", embedded_path, photo_path);
}

fn extract_file(photo_path: &str, passphrase: &str) {
    let _output = Command::new("steghide")
            .arg("extract")
            .arg("-sf")
            .arg(photo_path)
            .arg("-p")
            .arg(passphrase)
            .output()
            .expect("Command failed to start");


    println!("Extracted from {}", photo_path);
}

fn write_data_to_file(file_path: &str, data: Vec<u8>) -> io::Result<()> {
    let path = Path::new(file_path);
    
    // Create directories if they don't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    // Open the file in write mode (this will create the file if it doesn't exist)
    let mut file = File::create(path)?;

    // Write the data to the file
    file.write_all(&data)?;

    Ok(())
}


fn main() {
    let _data_path = "random_data_input";
    let passphrase = "yourmom";
    let mut images = Vec::new();
    images.push("test_images/test_image.jpg");
    images.push("test_images/test_image_2.jpg");
    images.push("test_images/test_image_3.jpg");


    let mut total_space_bytes: u64 = 0;

    for image in &images {
        println!("The capacity of {} is {} bytes", image, storage_capacity(image));
        total_space_bytes += storage_capacity(image);
    }
    println!("The total capacity of your drive is {} bytes", total_space_bytes);


    //embed_file(test_path, _data_path, passphrase);
    //extract_file(test_path, passphrase);
    //


    let mut file = File::open(_data_path).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer);


    let mut scrambled_content: Vec<Vec<u8>> = Vec::new();
    for image in &images {
        scrambled_content.push(Vec::new());
    }

    let mut output: String = String::new();
    let mut next_bin: usize = 0;
    for number in buffer {
        scrambled_content[next_bin].push(number);

        next_bin += 1;
        next_bin = next_bin % images.len();
    }


    /*
     *  Read file in chunks
     *
    let mut file = File::open(_data_path).unwrap();
    let chunk_size: usize = 8000;
    let mut chunk: Vec<u8> = Vec::with_capacity(chunk_size);
    let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk).unwrap();
    println!("Wrote {} bytes to tmpfile.", n);
    let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk).unwrap();
    println!("Wrote {} bytes to tmpfile.", n);
    */
}
