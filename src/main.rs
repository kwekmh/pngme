use libpngme::commands;
use clap::{Parser, Subcommand};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(value_name = "File Path")]
        file_path: String,
        #[arg(value_name = "Chunk Type")]
        chunk_type: String,
        #[arg(value_name = "Message")]
        message: String,
        #[arg(value_name = "Output File")]
        output_file: Option<String>
    },
    Decode {
        #[arg(value_name = "File Path")]
        file_path: String,
        #[arg(value_name = "Chunk Type")]
        chunk_type: String,
    },
    Remove {
        #[arg(value_name = "File Path")]
        file_path: String,
        #[arg(value_name = "Chunk Type")]
        chunk_type: String,
    },
    Print {
        #[arg(value_name = "File Path")]
        file_path: String,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Encode { file_path, chunk_type, message, output_file }) => {
            commands::encode(file_path, chunk_type, message, output_file)?;
        }
        Some(Commands::Decode { file_path, chunk_type }) => {
            if let Some(data) = commands::decode(file_path, chunk_type)? {
                println!("Decoded: {}", data);
            }
        }
        Some(Commands::Remove { file_path, chunk_type }) => {
            commands::remove(file_path, chunk_type)?;
        }
        Some(Commands::Print { file_path}) => {
            commands::print(file_path)?;
        }
        _ => {}
    }

    Ok(())
}