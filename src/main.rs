use std::process::Command;
use std::fs::File;
use std::io::Read;


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



/**
 * `data_file` is path to an open file handle to the data you are splitting.
 *
 * `chunk_size` is the size to read from the stream and write to `tmp_file`
 *
 * `tmp_file`, if it exists before the call, will get deleted and overwritten.
 *
 * Returns bytes written to `tmp_file`. If returned is 0 or less than chunk_size, the stream ended.
 */ 
fn write_next_chunk(data_file: File, chunk_size: u64, tmp_file: u64) -> u64 {

    /*
    let path = Path::new(tmp_file);
    if path.exists() {
        remove_file(path)?;
    }

    let mut file = File::create(tmp_file)?;
    file.write_all(&chunk)?;
    file.flush()?;
    */

    0
}

fn main() {
    let test_path = "test_image.jpg";
    let test_path_2 = "test_image_2.jpg";
    let _data_path = "random_data_input";
    let passphrase = "yourmom";

    storage_capacity(test_path_2);
    //embed_file(test_path, _data_path, passphrase);
    //extract_file(test_path, passphrase);
    //


    let mut file = File::open(_data_path).unwrap();
    let chunk_size: usize = 8000;
    let mut chunk: Vec<u8> = Vec::with_capacity(chunk_size);
    let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk).unwrap();
    println!("Wrote {} bytes to tmpfile.", n);
    let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk).unwrap();
    println!("Wrote {} bytes to tmpfile.", n);

}
