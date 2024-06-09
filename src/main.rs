mod z;

mod bitvec;
use bitvec::BitVec;

mod grid_lock;
use grid_lock::{GridLock, N, P};

use dialoguer::{theme::ColorfulTheme, Select, Input};

fn main() {
    let mut grid_lock = GridLock::new();
    let selections = &["Generate Key Pair", "Encrypt", "Decrypt", "Exit"];
    println!("Welcome to the GridLock Cryptosystem");
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .items(&selections[..])
            .default(0)
            .interact()
            .unwrap();
        match selection {
            0 => {
                let secret_key = grid_lock.gen_secret_key();
                println!("Secret Key Generated");
                let encoded_secret_key = ron::to_string(&secret_key).unwrap();
                println!("Saving Secret Key");
                std::fs::write("sk.ron", encoded_secret_key).expect("Failed to write secret key to file");
                println!("Secret Key saved to sk.ron");
                let public_key = grid_lock.gen_public_key(&secret_key);
                println!("Public Key Generated");
                let encoded_public_key = ron::to_string(&public_key).unwrap();
                println!("Saving Public Key");
                std::fs::write("pk.ron", encoded_public_key).expect("Failed to write public key to file");
                println!("Public Key saved to pk.ron");
            }
            1 => {
                let encoded_public_key = std::fs::read_to_string("pk.ron").expect("Failed to read public key from file");
                let public_key: Vec<(Vec<z::Z>, z::Z)> = ron::from_str(&encoded_public_key).unwrap();
                println!("Public Key Loaded");
                let encryptee_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Path to file to encrypt")
                    .interact_text()
                    .unwrap();
                let encryptee = BitVec::from_bytes(std::fs::read(encryptee_path).unwrap());
                let encrypted = grid_lock.encrypt(&public_key, encryptee);
                println!("Encryption Complete");
                let encoded_encrypted = ron::to_string(&encrypted).unwrap();
                let encrypted_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Path to save encrypted file")
                    .interact_text()
                    .unwrap();
                std::fs::write(&encrypted_path, encoded_encrypted).expect("Failed to write encrypted file to disk");
                println!("Encrypted file saved to {}", encrypted_path);
            }
            2 => {
                let encoded_secret_key = std::fs::read_to_string("sk.ron").expect("Failed to read secret key from file");
                let secret_key: Vec<z::Z> = ron::from_str(&encoded_secret_key).unwrap();
                println!("Secret Key Loaded");
                let encrypted_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Path to file to decrypt")
                    .interact_text()
                    .unwrap();
                let encoded_encrypted = std::fs::read_to_string(encrypted_path).expect("Failed to read encrypted file from disk");
                let encrypted: Vec<(Vec<z::Z>, z::Z)> = ron::from_str(&encoded_encrypted).unwrap();
                let decrypted = grid_lock.decrypt(&secret_key, &encrypted);
                println!("Decryption Complete");
                let decrypted_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Path to save decrypted file")
                    .interact_text()
                    .unwrap();
                std::fs::write(&decrypted_path, decrypted.to_bytes()).expect("Failed to write decrypted file to disk");
                println!("Decrypted file saved to {}", decrypted_path);
            }
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid selection");
            }
        } 
    }
}
