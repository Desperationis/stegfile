use crate::steglib::util::write_data_to_file;
use crate::steglib::split::Split;
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
            .arg("-Z")
            .arg("-N")
            .output()
            .expect("Command failed to start");


    println!("Embedded {} into {}", embedded_path, photo_path);
}


/**
 * Embed data from a buffer into multiple files using the chosen split method.
*/ 
pub fn mul_embed<T: Split>(input_buffer: Vec<u8>, image_paths: &Vec<String>, passphrase: &str) {
    // Get max byte capacity of each image
    let mut capacities: Vec<u64> = Vec::new();
    for image in image_paths {
        capacities.push(one_file_capacity(image));
    }

    // Split content 
    let split_content = T::split_to_bins(&input_buffer, &capacities);

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
    let mut tmp: usize = 0;
    for image in image_paths {
        let file_path = temp_path.join(format!("file_part_{}", tmp));
        steghide_embed(image, file_path.to_str().unwrap(), passphrase);
        tmp += 1;
    }
    println!("Done!");
}

