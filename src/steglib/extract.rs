use crate::steglib::util::write_data_to_file;
use std::fs::File;
use std::io::Read;
use tempfile::TempDir;

fn extract_file(photo_path: &str, output_path: &str, passphrase: &str) {
    let _output = std::process::Command::new("steghide")
            .arg("extract")
            .arg("-sf")
            .arg(photo_path)
            .arg("-p")
            .arg(passphrase)
            .arg("-xf")
            .arg(output_path)
            .output()
            .expect("Command failed to start");


    println!("Extracted from {}", photo_path);
}



/**
 * Reconstructs singular file from a list of image_paths and a passphrase. It is VERY important
 * that the order of the images in `image_paths` correspond to the file parts that were used in
 * first construction.
*/
pub fn reconstruct(image_paths: &Vec<String>, passphrase: &str, output_path: &str) {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    println!("Temporary directory path: {:?}", temp_path);

    let mut scrambled_pieces: Vec<Vec<u8>> = Vec::new();
    let mut total_size: usize = 0;
    let mut total_pieces: usize = 0;

    for image in image_paths {
        // First, get the secret files from the image
        let file_path = temp_path.join(format!("tmp_{}", total_pieces));
        let file_path_str = file_path.to_str().unwrap();
        extract_file(image, file_path_str, passphrase);
    

        let mut file = File::open(file_path_str).unwrap();
        let mut piece: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut piece);

        total_size += piece.len();
        total_pieces += 1;
        scrambled_pieces.push(piece);
    }

    println!("Size of all images is {}", total_size);
    println!("There are {} images to sift through", total_pieces);

    println!("Loaded all scrambled_pieces");

    let mut unified_piece: Vec<u8> = vec![0; total_size];
    println!("size of unified_piece is reserved to be {}", unified_piece.len());
    let mut offset: usize = 0;
    for piece in scrambled_pieces {

        let mut piece_num: usize = 0;
        println!("offset is {offset}");
        for byte in piece {
            unified_piece[offset + piece_num * total_pieces] = byte;
            piece_num += 1;
        }

        offset += 1;
    }


    println!("Descrambled pieces into one file. Writing...");
    let _ = write_data_to_file(output_path, unified_piece);
}

