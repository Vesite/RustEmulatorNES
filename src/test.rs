
use std::fs::File;
use std::io::{self, Read};

fn read_file_one_byte_at_a_time(filename: &str) -> io::Result<()> {

    // Open the file, read-only
    let mut file = File::open(filename)?;
    let mut buffer = [0u8, 1]; // Will hold 1 byte
    let mut all_bytes = String::new();

    // Read the file
    while let Ok(bytes_read) = file.read(&mut buffer) {

        if bytes_read == 0 {
            break; // End of file
        }

        // Process the bytes
        // println!("{}", buffer[0]);
        all_bytes.push_str(&format!("{:02x}", buffer[0]));

    }

    println!("{}", all_bytes);

    Ok(())

}

fn main() {
    let filename = "test_roms/cpu.nes";
    if let Err(e) = read_file_one_byte_at_a_time(filename) {
        eprintln!("Error reading file: {}", e)
    }
}

struct CPU {
    A: u8,
    X: u8,
    Y: u8,
    PC: u16,
    S: u8,
    P: u8,
}
