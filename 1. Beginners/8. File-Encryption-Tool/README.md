# File Encryption Tool

A simple command-line tool for encrypting and decrypting files using AES-128 encryption.

## Features

- File encryption using AES-128
- File decryption using AES-128
- Simple command-line interface
- Error handling and user feedback

## Requirements

- Rust 2021 edition or later
- Cargo package manager

## Installation

1. Clone the repository
2. Navigate to the project directory:
   ```bash
   cd 8. File-Encryption-Tool
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Encrypting a File

```bash
cargo run -- encrypt -i input.txt -o encrypted.dat -k "your16bytekey!!"
```

### Decrypting a File

```bash
cargo run -- decrypt -i encrypted.dat -o decrypted.txt -k "your16bytekey!!"
```

### Command Line Arguments

- `-i, --input`: Path to the input file
- `-o, --output`: Path to the output file
- `-k, --key`: 16-byte encryption/decryption key

## Important Notes

- The encryption key must be exactly 16 bytes long
- Keep your encryption key secure and don't share it
- Make sure to back up your original files before encryption
- The same key must be used for both encryption and decryption

## Learning Objectives

This project demonstrates:
- File I/O operations in Rust
- Command-line argument parsing
- Error handling with anyhow
- Basic cryptography concepts
- Working with binary data
- Structuring a Rust CLI application

## Example

```bash
# Create a test file
echo "Hello, World!" > test.txt

# Encrypt the file
cargo run -- encrypt -i test.txt -o encrypted.dat -k "supersecretkey!!"

# Decrypt the file
cargo run -- decrypt -i encrypted.dat -o decrypted.txt -k "supersecretkey!!"
``` 