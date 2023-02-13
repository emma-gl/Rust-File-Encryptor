// import needed libraries
use anyhow::anyhow;
use chacha20poly1305::{
    aead::{Aead, Error, NewAead, generic_array::typenum::Unsigned},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, RngCore};
use std::fs;
use std::io::{stdin,stdout,Write};
use sha2::{Digest, Sha256};
use rand::prelude::*;

//create list of all possible characters (for generate_password function)
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}\\|;:\'\",.<>/?`~";

fn main() -> Result<(), anyhow::Error> {
    // generate key. nonce, and tags
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 24];
    let tag = [0u8];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

    //get choice from user
    let generate_hash_encrypt_or_decrypt: String = get_input("Would you like to:\n \
     - generate a password\n \
     - hash a password\n \
     - encrypt a file\n \
     - decrypt a file");
    //read files in directory
    for file in fs::read_dir("./").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
    //call function respective to user's input
    if generate_hash_encrypt_or_decrypt == "generate"{
        //generate random password string and print it
        let password = generate_random_string(16);
        println!("Generated password: {}", password);
    } else if generate_hash_encrypt_or_decrypt == "hash"{
        //get password from user
        let input = get_input("What is the password you would like to hash?");
        // apply hash function to password
        let hashed = hash(input.as_str());
        // display hashed password
        println!("The SHA-256 hash of {} is {}", input, hashed);
    }
    else{
        //get filename from user
        let name = get_input("What is the filename?");
        let filename: &str = name.as_str();
        if generate_hash_encrypt_or_decrypt == "encrypt"{
            println!("Encrypting {}...", name);
            //encrypt inputted file
            encrypt_small_file(
                filename,
                "encrypted.encrypted",
                &key,
                &nonce,
            )?;
            } else{
            println!("Decrypting {}...", name);
            // decrypt inputted file
            decrypt_small_file(
                filename,
                "decrypted.decrypted",
                &key,
                &nonce,
                &tag
            )?;
            }
    }
    //"done" signal 
    Ok(())
}

fn get_input(prompt: &str)-> String {
    //displays prompt to user and returns response with white space removed
    let mut answer = String::new();
    println!("{}", prompt);
    let _= stdout().flush();
    stdin().read_line(&mut answer).expect("Did not enter a correct string");
    if let Some('\n')=answer.chars().next_back() {
        answer.pop();
    }
    if let Some('\r')=answer.chars().next_back() {
        answer.pop();
    }
    return answer;
}

fn encrypt_small_file(filepath: &str,dist: &str,key: &[u8; 32],nonce: &[u8; 24],) -> Result<(), anyhow::Error> {
    //read in file contents and apply cipher function to it
    let cipher = XChaCha20Poly1305::new(key.into());
    let file_data = fs::read(filepath)?;
    let encrypted_file = cipher
        .encrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {}", err))?;
    //save encrypted file data to new file
    fs::write(&dist, encrypted_file)?;

    Ok(())
}

fn decrypt_small_file(decrypted_file_path: &str, dist: &str, key: &[u8; 32], nonce: &[u8; 24], tag: &[u8]) -> Result<(), anyhow::Error> {
    //read in encrypted file contents and decrypt it using cipher used
    let cipher = XChaCha20Poly1305::new(key.into());
    let file_data = fs::read(decrypted_file_path)?;
    let decrypted_file = match cipher.decrypt(nonce.into(), &file_data[file_data.len()-16..]) {
        Ok(v) => v,
        Err(e) => return Err(anyhow!("Decrypting small file: {}", e)),
    };
    //save decrypted file data to new file
    fs::write(dist, decrypted_file)?;

    Ok(())
}

fn hash(input: &str) -> String {
    //apply sha256 hashing algorithm to passed-in string
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn generate_random_string(length: usize) -> String {
    //add random character from CHARSET to a string according to the desired length
    let mut rng = rand::thread_rng();
    let mut password = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.gen::<usize>() % CHARSET.len();
        password.push(CHARSET[idx] as char);
    }
    password
}