# Rust-File-Encryptor Overview

My program has four main functionalities in which the user inputs what action they want to perform. These features include:
 - generating a random password of letters, numbers, and special characters
 - create a hash of an inputted password
 - encrypt a file using symmetric encryption
 - decrypt a file

My purpose for writing this software was (in addition to learning rust,) to inform users about the importance of password protection by providing an easy way to generate a good password, hashing passwords, and encrypting/decrypting files.

{Provide a link to your YouTube demonstration. It should be a 4-5 minute demo of the software running and a walkthrough of the code. Focus should be on sharing what you learned about the language syntax.}

[Software Demo Video](https://www.loom.com/share/b77d579fecb64873b2c93679ce165d87)

# Development Environment
The main tool I used to develop the software was [cargo](https://doc.rust-lang.org/cargo/), the default package manager that comes with Rust. Cargo made it easy to manage the dependencies and compile/build the project. 

This program was written in [Rust](https://www.rust-lang.org/), a performance-focused systems programming language. Rust provides the control and performance of low-level languages with the expressiveness and high-level abstractions of modern programming languages. I used a number of Rust languages, including:

- [rand](https://docs.rs/rand/latest/rand/) (random number generation library)

- [sha2](https://crates.io/crates/sha2) (SHA-2 family of cryptographic hash functions)

- [anyhow](https://docs.rs/anyhow/latest/anyhow/) (error-handling library)

- [fs](https://doc.rust-lang.org/std/fs/) (library fpr reading and writing to the file system)

- [getopts](https://docs.rs/getopts/latest/getopts/) (commandline arguments)

- [ring](https://docs.rs/ring/latest/ring/) (rust cryptography library)

# Useful Websites
Listed below is a few websites that I found helpful while developing my program:

- [Rust Programming Language Book](https://doc.rust-lang.org/book/index.html)
- [Rust Crypto - Cryptographic algorithms written in pure Rust](https://github.com/RustCrypto)
- [Rust Cookbook - Cryptography](https://rust-lang-nursery.github.io/rust-cookbook/cryptography/encryption.html)

# Future Work
Some potential additions for this program may be:
- Multiple different hash functions (MD5, LANMAN, etc.)
- Asymmetric encryption
- Connect the parts (store hash of generated password in an encrypted file)