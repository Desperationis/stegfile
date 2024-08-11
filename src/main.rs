use std::process::Command;

fn embed_file(photo_path: &str, embedded_path: &str, passphrase: &str) {
    Command::new("steghide")
            .arg("embed")
            .arg("-cf")
            .arg(photo_path)
            .arg("-ef")
            .arg(embedded_path)
            .arg("-p")
            .arg(passphrase)
            .spawn()
            .expect("Command failed to start");


    println!("Embedded {} into {}", embedded_path, photo_path);
}

fn extract_file(photo_path: &str, passphrase: &str) {
    Command::new("steghide")
            .arg("extract")
            .arg("-sf")
            .arg(photo_path)
            .arg("-p")
            .arg(passphrase)
            .spawn()
            .expect("Command failed to start");


    println!("Extracted from {}", photo_path);
}

fn main() {
    let test_path = "test_image.jpg";
    let data_path = "random_data_input";
    let passphrase = "yourmom";
    //embed_file(test_path, data_path, passphrase);
    extract_file(test_path, passphrase);
}
