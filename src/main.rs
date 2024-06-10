mod z;

mod bitvec;
use bitvec::BitVec;

mod grid_lock;
use grid_lock::{GridLock, N, P};

use dialoguer::{theme::ColorfulTheme, Select, Input};

use serde_pickle::{SerOptions, DeOptions};

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
                println!("Saving Secret Key");
                let mut secret_key_file = std::fs::File::create("sk").expect("Failed to create secret key file");
                serde_pickle::to_writer(
                    &mut secret_key_file,
                    &secret_key,
                    SerOptions::default(),
                ).expect("Failed to write secret key to file");
                println!("Secret Key saved to sk");
                let public_key = grid_lock.gen_public_key(&secret_key);
                println!("Public Key Generated");
                println!("Saving Public Key");
                let mut public_key_file = std::fs::File::create("pk").expect("Failed to create public key file");
                serde_pickle::to_writer(
                    &mut public_key_file,
                    &public_key,
                    SerOptions::default(),
                ).expect("Failed to write public key to file");
                println!("Public Key saved to pk");
            }
            1 => {
                let mut pk_file = std::fs::File::open("pk").expect("Failed to open public key file");
                let public_key: Vec<(Vec<z::Z>, z::Z)> = serde_pickle::from_reader(
                    &mut pk_file,
                    DeOptions::default(),
                ).expect("Failed to read public key from file");
                println!("Public Key Loaded");
                let encryptee_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Path to file to encrypt")
                    .interact_text()
                    .unwrap();
                let encryptee = BitVec::from_bytes(std::fs::read(&encryptee_path).unwrap());
                let encrypted = grid_lock.encrypt(&public_key, encryptee);
                println!("Encryption Complete");
                let mut encripted_file = std::fs::File::create(format!("{}.gl", &encryptee_path)).expect("Failed to create encrypted file");
                serde_pickle::to_writer(
                    &mut encripted_file,
                    &encrypted,
                    SerOptions::default(),
                ).expect("Failed to write encrypted file to disk");
                println!("Encrypted file saved to {}.gl", &encryptee_path);
            }
            2 => {
                let mut sk_file = std::fs::File::open("sk").expect("Failed to open secret key file");
                let secret_key: Vec<z::Z> = serde_pickle::from_reader(
                    &mut sk_file,
                    DeOptions::default(),
                ).expect("Failed to read secret key from file");
                println!("Secret Key Loaded");
                let encrypted_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Path to file to decrypt")
                    .interact_text()
                    .unwrap();
                let mut encrypted_file = std::fs::File::open(&encrypted_path).expect("Failed to open encrypted file");
                let encrypted: Vec<(Vec<z::Z>, z::Z)> = serde_pickle::from_reader(
                    &mut encrypted_file,
                    DeOptions::default(),
                ).expect("Failed to read encrypted file from disk");
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
