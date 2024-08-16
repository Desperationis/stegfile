// Lookup custom library
mod steglib;
use steglib::cli::{Cli, Commands};
use steglib::capacity::storage_capacity;
use steglib::embed::atomizize;
use steglib::extract::reconstruct;
use steglib::util::find_jpg_images;

use std::path::Path;
use clap::Parser;


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
