use crate::steglib::util::write_data_to_file;
use crate::steglib::split::Split;
use std::fs::File;
use std::io::Read;
use tempfile::TempDir;

fn steghide_extract(photo_path: &str, output_path: &str, passphrase: &str) {
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
pub fn mul_extract<T: Split>(image_paths: &Vec<String>, passphrase: &str, output_path: &str) {
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
        steghide_extract(image, file_path_str, passphrase);
    

        let mut file = File::open(file_path_str).unwrap();
        let mut piece: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut piece);

        total_pieces += 1;
        scrambled_pieces.push(piece);
    }

    // The scrambled_pieces may not necessarily be in order. Use the first 8 bytes of each image to
    // sort correctly.
    let mut sorted_pieces: Vec<Vec<u8>> = vec![Vec::new(); scrambled_pieces.len()];

    for piece in scrambled_pieces {
        if piece.len() == 8 {
            continue;
        }

        let index = u64::from_be_bytes(
            piece[0..8].try_into().expect("Slice with incorrect length")
        ) as usize;

        // Place the inner vector in the correct position in the sorted vector
        println!("This piece will go in index {}", index);
        sorted_pieces[index] = piece[8..].to_vec(); // Remove leading 8 bits
        total_size += piece.len() - 8;
    }


    println!("Size of all images is {}", total_size);
    println!("There are {} images to sift through", total_pieces);

    println!("Loaded all scrambled_pieces");

    let unified_piece: Vec<u8> = T::join(sorted_pieces);


    println!("Descrambled pieces into one file. Writing...");
    let _ = write_data_to_file(output_path, unified_piece);
}


