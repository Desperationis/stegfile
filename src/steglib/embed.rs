use crate::steglib::util::{write_data_to_file};
use crate::steglib::split::{Split};
use crate::steglib::capacity::one_file_capacity;
use tempfile::TempDir;

fn steghide_embed(photo_path: &str, embedded_path: &str, passphrase: &str) {
    let _output = std::process::Command::new("steghide")
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


/**
 * Embed data from a buffer into multiple files using the chosen split method.
*/ 
pub fn mul_embed<T: Split>(input_buffer: Vec<u8>, image_paths: &Vec<String>, passphrase: &str) {
    // Load file in memory
    let mut capacities: Vec<u64> = Vec::new();
    for image in image_paths {
        capacities.push(one_file_capacity(image));
    }

    let mut scrambled_content = T::split(input_buffer, capacities);
    
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    println!("Temporary directory path: {:?}", temp_path);



    // VERY IMPORTANT: Prepend 8 bytes (64-bit, 18,446,744,073,709,551,616 possible values) to the
    // beginning of each bucket to mark the bucket#. This is needed for reconstruction.
    fn prepend_u64(vec: &mut Vec<u8>, value: u64) {
        let bytes = value.to_be_bytes(); // or use to_le_bytes() for little-endian
        vec.splice(0..0, bytes.iter().copied());
    }

    let mut index: u64 = 0;
    for bucket in &mut scrambled_content {
        prepend_u64(bucket, index);
        index += 1;
    }





    // Dump buckets into files
    let mut index: usize = 0;
    for file in scrambled_content {
        println!("Writing to file_part_{}", index);
        let file_path = temp_path.join(format!("file_part_{}", index));
        let _ = write_data_to_file(file_path.to_str().unwrap(), file);
        index += 1;
    }

    let mut tmp: usize = 0;
    for image in image_paths {
        let file_path = temp_path.join(format!("file_part_{}", tmp));
        steghide_embed(image, file_path.to_str().unwrap(), passphrase);
        tmp += 1;
    }
    println!("Done!");
}

