use std::{fs::File, io::Read, path::Path};

use clap::{Parser, Subcommand};

use rpf_rs::{prelude::Archive, Result};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Info { file: String },
}

fn info<P>(file: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut file = File::open(file)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;

    let mut archive = Archive::from(&data);
    let header = archive.read_header()?;

    println!("Size: {} Bytes", data.len());
    println!("Magic: {} Bytes", header.magic);

    if header.is_valid_archive() {
        println!("Entry Count: {} Bytes", header.entry_count);
        println!("Names Length: {} Bytes", header.names_length);
        println!("Encryption: {} Bytes", header.encryption_type);
    } else {
        println!("This is not an valid RPF archive.")
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Info { file } => {
            let res = info(file);
            if let Err(reason) = res {
                println!("Error: {}", reason);
            }
        }
    }

    Ok(())
}
