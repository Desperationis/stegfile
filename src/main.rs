// Lookup custom library
mod steglib;
use std::fs::File;
use std::io::Read;
use steglib::capacity::{MulCapacity, MulFullCapacity, MulScrambledCapacity};
use steglib::cli::{Cli, Commands, SplitModeEnum};
use steglib::embed::mul_embed;
use steglib::extract::mul_extract;
use steglib::split::{SplitChunks, SplitScrambled};
use steglib::util::find_jpg_images;

use clap::Parser;
use std::path::Path;

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

            if !image_path.is_dir() {
                println!("{} is not a directory. Please try again.", image_dir);
                std::process::exit(1);
            }
            println!("Found {} JPG images.", images.len());

            match &cli.split_mode {
                SplitModeEnum::Scrambled => {
                    mul_extract::<SplitScrambled>(&images, passphrase, output_file);
                }
                SplitModeEnum::Full => {
                    mul_extract::<SplitChunks>(&images, passphrase, output_file);
                }
            }
        }
        Commands::Embed {
            image_dir,
            passphrase,
            input_file,
        } => {
            let mut images: Vec<String> = Vec::new();
            let image_path = Path::new(image_dir);
            find_jpg_images(image_path, &mut images);

            if !image_path.is_dir() {
                println!("{} is not a directory. Please try again.", image_dir);
                std::process::exit(1);
            }

            let mut file = File::open(input_file).unwrap();
            let mut buffer: Vec<u8> = Vec::new();
            let _ = file.read_to_end(&mut buffer);

            match &cli.split_mode {
                SplitModeEnum::Scrambled => {
                    mul_embed::<SplitScrambled>(buffer, &images, passphrase);
                }
                SplitModeEnum::Full => {
                    mul_embed::<SplitChunks>(buffer, &images, passphrase);
                }
            }
        }

        Commands::Capacity { image_dir } => {
            let mut images: Vec<String> = Vec::new();
            let image_path = Path::new(image_dir);
            if !image_path.is_dir() {
                println!("{} is not a directory. Please try again.", image_dir);
                std::process::exit(1);
            }
            println!("Searching scrambled images...");
            find_jpg_images(image_path, &mut images);
            println!("Done.");

            let scrambled_capacity = MulScrambledCapacity::capacity(&images);
            let full_capacity = MulFullCapacity::capacity(&images);

            println!("Capacity using scrambled egg: {}", scrambled_capacity);
            println!("Capacity using whole egg: {}", full_capacity);
        }
    }
}
