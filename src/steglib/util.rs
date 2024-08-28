use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/**
 * Create path to `file_path` if it doesn't exist, then write binary data to file.
*/
pub fn write_data_to_file(file_path: &str, data: Vec<u8>) {
    let path = Path::new(file_path);

    // Create directories if they don't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .expect(&format!("Failed to create directory for {}", file_path));
        }
    }

    // Open the file in write mode (this will create the file if it doesn't exist)
    let mut file = File::create(path).expect(&format!("Failed to create {}", file_path));

    // Write the data to the file
    file.write_all(&data)
        .expect(&format!("Failed to write data to {}", file_path));
}

pub fn find_jpg_images(dir: &Path, images: &mut Vec<String>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("read_dir failed") {
            let entry = entry.expect("entry failed");
            let path = entry.path();

            if path.is_dir() {
                // Recursively search through the subdirectory
                find_jpg_images(&path, images);
            } else {
                let extension = path.extension().and_then(|s| s.to_str());
                if extension == Some("jpg")
                    || extension == Some("JPG")
                    || extension == Some("JPEG")
                    || extension == Some("jpeg")
                {
                    images.push(
                        path.canonicalize()
                            .expect("canonicalize failed")
                            .to_string_lossy()
                            .to_string(),
                    );
                }
            }
        }
    }
}
