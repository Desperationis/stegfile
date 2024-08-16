pub fn storage_capacity(photo_path: &str) -> u64 {
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

