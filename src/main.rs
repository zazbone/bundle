use std::path::PathBuf;

use clap::{Parser, Subcommand};

use bundle;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    path: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create,
    Add { meta: String },
    Get { meta: String },
    Remove { meta: String },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Create => {
            let bundle = bundle::Bundle::create(args.path).expect("Failed to create bundle");
            bundle.save().expect("Failed to save bundle");
        }
        Commands::Add { meta } => {
            let mut bundle = bundle::Bundle::open(args.path).unwrap();
            let file_path = bundle.new_field(&serde_json::from_str(&meta).unwrap());
            println!("{}", &file_path.display());
            let _ = std::fs::create_dir(file_path);
            bundle.save().unwrap();
        }

        Commands::Get { meta } => {
            let mut bundle = bundle::Bundle::open(args.path).unwrap();
            let file_path = bundle
                .get_field(&serde_json::from_str(&meta).unwrap())
                .expect("Queried meta doesn't exist.");
            println!("{}", &file_path.display());
            bundle.save().unwrap();
        }
        Commands::Remove { meta } => {
            let mut bundle = bundle::Bundle::open(args.path).unwrap();
            let file_path = bundle
                .rm_field(&serde_json::from_str(&meta).unwrap())
                .expect("Field to remove doesn't exist.");
            println!("{}", &file_path.display());
            bundle.save().unwrap();
        }
    };
}
