pub fn one_file_capacity(photo_path: &str) -> u64 {
    let _output = std::process::Command::new("steghide")
            .arg("--info")
            .arg(photo_path)
            .output()
            .unwrap();

    let s = String::from_utf8_lossy(&_output.stdout);
    let capacity_line = s.lines()
        .find(|line| line.contains("capacity"))
        .unwrap();

    let capacity_value = capacity_line.split(':')
        .nth(1)
        .unwrap()
        .trim();

    let mut parts = capacity_value.split_whitespace();
    let value_str = parts.next().unwrap();
    let prefix = parts.next().unwrap();
    let value: f64 = value_str.parse().unwrap();

    // Determine the multiplier based on the prefix
    let multiplier = match prefix {
        "Byte" => 1.0,
        "KB" => 1000.0,
        "MB" => 1_000_000.0,
        "GB" => 1_000_000_000.0,
        _ => 1.0, // Default multiplier if an unknown prefix is encountered
    };

    // Calculate the result by multiplying the value with the multiplier
    (value * multiplier) as u64
}


pub trait MulCapacity {
    /**
     * `files` are paths to any file `steghide` can support.
     */ 
    fn capacity(files: &Vec<String>) -> u64;
}

/*
 * Return capacity of several files assuming the data will be scrambled. This means that the amount of
 * data that can be stored is equal to the smallest file times the number of files, as the
 * scrambled method must write an equal file size to each image.
*/
pub struct MulScrambledCapacity;


/*
 * Return capacity of several files assuming the data is split as efficiently as possible between
 * every image as chunks.
*/
pub struct MulFullCapacity;



impl MulCapacity for MulScrambledCapacity {
    fn capacity(files: &Vec<String>) -> u64 {
        let mut smallest_file_size: u64 = std::u64::MAX;

        for file in files {
            println!("Finding capacity of {}", file);
            let capacity: u64 = one_file_capacity(file);
            smallest_file_size = std::cmp::min(smallest_file_size, capacity - 8);
        }

        smallest_file_size * (files.len() as u64)
    }
}



impl MulCapacity for MulFullCapacity {
    fn capacity(files: &Vec<String>) -> u64 {
        let mut total_file_size: u64 = 0;

        for file in files {
            println!("Finding capacity of {}", file);
            let capacity: u64 = one_file_capacity(file);
            total_file_size += capacity - 8;
        }

        total_file_size
    }
}

