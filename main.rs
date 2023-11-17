use std::io::{self, BufRead};
use bs58;

fn base58_to_wallet() -> Vec<u8> {
    println!("Enter your base58-encoded wallet strings (press Ctrl-D on Unix or Ctrl-Z on Windows to finish):");
    let stdin = io::stdin();
    let mut wallet = Vec::new();

    for line in stdin.lock().lines() {
        match line {
            Ok(base58) => {
                match bs58::decode(&base58).into_vec() {
                    Ok(decoded_bytes) => {
                        wallet.extend(decoded_bytes);
                    }
                    Err(e) => {
                        eprintln!("Error decoding base58: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
            }
        }
    }

    wallet
}

fn main() {
    let wallet = base58_to_wallet();
    println!("Decoded wallet bytes: {:?}", wallet);
}