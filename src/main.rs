use std::io::Read;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use tempfile::TempDir;
use std::path::PathBuf;
use clap::{Arg, Command, Subcommand, Parser};


fn storage_capacity(photo_path: &str) -> u64 {
    let _output = std::process::Command::new("steghide")
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
fn atomizize(input_file: &str, image_paths: &Vec<String>, passphrase: &str) {
    // Load file in memory
    let mut file = File::open(input_file).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer);


    // Initialize places for scrambled memory
    let mut scrambled_content: Vec<Vec<u8>> = Vec::new();
    for image in image_paths {
        scrambled_content.push(Vec::new());
    }

    // Scramble file into those buckets
    let output: String = String::new();
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
        write_data_to_file(file_path.to_str().unwrap(), file);
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


/**
 * Reconstructs singular file from a list of image_paths and a passphrase. It is VERY important
 * that the order of the images in `image_paths` correspond to the file parts that were used in
 * first construction.
*/
fn reconstruct(image_paths: &Vec<String>, passphrase: &str, output_path: &str) {
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
        file.read_to_end(&mut piece);

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
    write_data_to_file(output_path, unified_piece);
}















#[derive(Subcommand)]
enum Commands {
    Extract {
        image_dir: String,
        passphrase: String,
        output_file: String,
    },
    Embed {
        image_dir: String,
        passphrase: String,
        input_file: String,
    },
    Capacity {
        image_dir: String,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn find_jpg_images(dir: &Path, images: &mut Vec<String>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("read_dir failed") {
            let entry = entry.expect("entry failed");
            let path = entry.path();

            if path.is_dir() {
                // Recursively search through the subdirectory
                find_jpg_images(&path, images);
            } else if path.extension().and_then(|s| s.to_str()) == Some("jpg") {
                // If it's a JPG file, add its absolute path to the vector
                images.push(path.canonicalize().expect("canonicalize failed").to_string_lossy().to_string());
            }
        }
    }
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Extract {
            image_dir,
            passphrase,
            output_file,
        } => {
            let mut images: Vec<String> = Vec::new();
            let image_path = Path::new(image_dir);
            find_jpg_images(image_path, &mut images);

            if ! image_path.is_dir() {
                println!("{} is not a directory. Please try again.",  image_dir);
                std::process::exit(1);
            }

            reconstruct(&images, passphrase, output_file);

        }
        Commands::Embed {
            image_dir,
            passphrase,
            input_file,
        } => {
            let mut images: Vec<String> = Vec::new();
            let image_path = Path::new(image_dir);
            find_jpg_images(image_path, &mut images);

            if ! image_path.is_dir() {
                println!("{} is not a directory. Please try again.",  image_dir);
                std::process::exit(1);
            }


            atomizize(input_file, &images, passphrase);
        }

        Commands::Capacity {
            image_dir,
        } => {
            let mut total_space_bytes: u64 = 0;
            let mut images: Vec<String> = Vec::new();
            let image_path = Path::new(image_dir);
            find_jpg_images(image_path, &mut images);

            if ! image_path.is_dir() {
                println!("{} is not a directory. Please try again.",  image_dir);
                std::process::exit(1);
            }

            for image in &images {
                println!("{}\t {}", storage_capacity(image), image);
                total_space_bytes += storage_capacity(image);
            }
            println!("The total capacity of your drive is {} bytes", total_space_bytes);
        }
    }



}
