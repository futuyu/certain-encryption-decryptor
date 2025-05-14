
use simple_rijndael::impls::RijndaelCbc;
use simple_rijndael::paddings::ZeroPadding;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

fn main() -> std::io::Result<()> {
    println!("Certain encryption decryptor starting...");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];
    let output_filename = format!("{}.dec", filename);

    let key_str = b"levelup321";
    let mut key = [0u8; 32];
    key[..key_str.len()].copy_from_slice(key_str);
    let iv = key;

    // Read encrypted data
    let mut input_file = File::open(filename)?;
    let mut data = Vec::new();
    input_file.read_to_end(&mut data)?;
    println!("Read {} bytes from {}", data.len(), filename);

    // Initialize cipher with 32-byte block size and ZeroPadding
    let cipher = RijndaelCbc::<ZeroPadding>::new(&key, 32)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Cipher init error: {:?}", e)))?;

    println!("Decrypting {}...", filename);
    // Decrypt data
    let decrypted_data = cipher.decrypt(&iv, data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Decryption error: {:?}", e)))?;
    println!("Decryption successful!");

    // Write decrypted data
    let mut output_file = File::create(&output_filename)?;
    output_file.write_all(&decrypted_data)?;
    println!("Written decrypted data to {}", output_filename);

    Ok(())
}

