use crate::steglib::util::{write_data_to_file};
use std::fs::File;
use std::io::Read;
use tempfile::TempDir;

fn embed_file(photo_path: &str, embedded_path: &str, passphrase: &str) {
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
 * Scrambles a file into pieces, and puts those pieces into separate images. For example:
 *
 * Input file: this is a text
 * 
 * If there are three images:
 *
 * #1: tss s
 * #2: h  tt
 * #3: iiae
 *
 * The purpose of splitting it is to pretty heavily corrupt the file if any single image were to go
 * missing. Sure, the amount of damage depends on what type of data is being encoded, but it is
 * much better than storing the file into huge pieces.
 */
pub fn atomizize(input_file: &str, image_paths: &Vec<String>, passphrase: &str) {
    // Load file in memory
    let mut file = File::open(input_file).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);


    // Initialize places for scrambled memory
    let mut scrambled_content: Vec<Vec<u8>> = Vec::new();
    for _ in image_paths {
        scrambled_content.push(Vec::new());
    }

    // Scramble file into those buckets
    let mut next_bin: usize = 0;
    for number in buffer {
        scrambled_content[next_bin].push(number);

        next_bin += 1;
        next_bin = next_bin % image_paths.len();
    }

    
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    println!("Temporary directory path: {:?}", temp_path);

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
        embed_file(image, file_path.to_str().unwrap(), passphrase);
        tmp += 1;
    }
    println!("Done!");
}

