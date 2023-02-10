use anyhow::anyhow;
use chacha20poly1305::{
    aead::{stream, Aead, NewAead},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, RngCore};
// use std::{
//     fs::{self, File},
//     io::{Read, Write},
// };
use std::fs;
use std::io::{stdin,stdout,Write};
use std::env;

fn main() -> Result<(), anyhow::Error> {
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

    let mut name = String::new();
    println!("Enter the name of the file :");
    //io::stdin().read_line(&mut name).expect("failed to readline");
    let _=stdout().flush();
    stdin().read_line(&mut name).expect("Did not enter a correct string");
    if let Some('\n')=name.chars().next_back() {
        name.pop();
    }
    if let Some('\r')=name.chars().next_back() {
        name.pop();
    }
    let mut filename: &str = name.as_str();

    for file in fs::read_dir("./").unwrap() {
        println!("{}", file.unwrap().path().display());
    }



    println!("Encrypting {}...", name);
    encrypt_small_file(
        filename,
        "encrypted.encrypted",
        &key,
        &nonce,
    )?;

    println!("Decrypting {}...", name);
    decrypt_small_file(
        filename,
        "decrypted.decrypted",
        &key,
        &nonce,
    )?;


    println!("Finit");
    Ok(())
}

fn encrypt_small_file(
    filepath: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let file_data = fs::read(filepath)?;

    let encrypted_file = cipher
        .encrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {}", err))?;

    fs::write(&dist, encrypted_file)?;

    Ok(())
}

fn decrypt_small_file(
    encrypted_file_path: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let file_data = fs::read(encrypted_file_path)?;

    let decrypted_file = cipher
        .decrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

    fs::write(&dist, decrypted_file)?;

    Ok(())
}