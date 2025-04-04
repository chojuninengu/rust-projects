use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use crypto::aes::Aes128;
use crypto::blockmodes::CbcMode;
use crypto::buffer::{ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::symmetriccipher::BlockEncryptor;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a file
    Encrypt {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Encryption key (16 bytes)
        #[arg(short, long)]
        key: String,
    },

    /// Decrypt a file
    Decrypt {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Decryption key (16 bytes)
        #[arg(short, long)]
        key: String,
    },
}

fn encrypt_file(input_path: &PathBuf, output_path: &PathBuf, key: &str) -> Result<()> {
    let input_data = fs::read(input_path)
        .with_context(|| format!("Failed to read input file: {:?}", input_path))?;

    let key_bytes = key.as_bytes();
    if key_bytes.len() != 16 {
        return Err(anyhow::anyhow!("Key must be exactly 16 bytes long"));
    }

    let mut key_array = [0u8; 16];
    key_array.copy_from_slice(key_bytes);

    let aes = Aes128::new(&key_array);
    let mut encryptor = CbcMode::new(aes, [0u8; 16]);

    let mut output_data = Vec::new();
    let mut read_buffer = RefReadBuffer::new(&input_data);
    let mut write_buffer = RefWriteBuffer::new(&mut output_data);

    encryptor
        .encrypt(&mut read_buffer, &mut write_buffer, true)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    fs::write(output_path, output_data)
        .with_context(|| format!("Failed to write output file: {:?}", output_path))?;

    println!("File encrypted successfully!");
    Ok(())
}

fn decrypt_file(input_path: &PathBuf, output_path: &PathBuf, key: &str) -> Result<()> {
    let input_data = fs::read(input_path)
        .with_context(|| format!("Failed to read input file: {:?}", input_path))?;

    let key_bytes = key.as_bytes();
    if key_bytes.len() != 16 {
        return Err(anyhow::anyhow!("Key must be exactly 16 bytes long"));
    }

    let mut key_array = [0u8; 16];
    key_array.copy_from_slice(key_bytes);

    let aes = Aes128::new(&key_array);
    let mut decryptor = CbcMode::new(aes, [0u8; 16]);

    let mut output_data = Vec::new();
    let mut read_buffer = RefReadBuffer::new(&input_data);
    let mut write_buffer = RefWriteBuffer::new(&mut output_data);

    decryptor
        .decrypt(&mut read_buffer, &mut write_buffer, true)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    fs::write(output_path, output_data)
        .with_context(|| format!("Failed to write output file: {:?}", output_path))?;

    println!("File decrypted successfully!");
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { input, output, key } => {
            encrypt_file(&input, &output, &key)?;
        }
        Commands::Decrypt { input, output, key } => {
            decrypt_file(&input, &output, &key)?;
        }
    }

    Ok(())
}
