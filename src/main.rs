use std::io;
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
    Add {
        meta: String,
    },
    Get {
        meta: String,
    },
    Remove {
        meta: String,
        #[clap(long, short)]
        clear: bool,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Create => {
            let bundle = match bundle::Bundle::create(args.path) {
                Ok(v) => v,
                Err(e) => handle_io_error(e),
            };
            match bundle.save() {
                Ok(()) => {}
                Err(e) => handle_io_error(e),
            };
        }
        Commands::Add { meta } => {
            let mut bundle = match bundle::Bundle::open(args.path) {
                Ok(v) => v,
                Err(e) => handle_io_error(e),
            };
            let field_meta = match serde_json::from_str(&meta) {
                Ok(v) => v,
                Err(e) => handle_serde_error(e),
            };
            let file_path = match bundle.new_field(&field_meta) {
                Ok(v) => v,
                Err(_e) => panic!("Dyn error not yet handeled"),
            };
            match std::fs::create_dir(&file_path) {
                Ok(_) => println!("{}", file_path.display()),
                Err(e) => handle_io_error(e),
            }

            match bundle.save() {
                Ok(()) => {}
                Err(e) => handle_io_error(e),
            };
        }

        Commands::Get { meta } => {
            let mut bundle = match bundle::Bundle::open(args.path) {
                Ok(v) => v,
                Err(e) => handle_io_error(e),
            };
            let field_meta = match serde_json::from_str(&meta) {
                Ok(v) => v,
                Err(e) => handle_serde_error(e),
            };
            let file_path = match bundle.get_field(&field_meta) {
                Some(v) => v,
                None => {
                    eprintln!("No existing fields match the given metadata.");
                    std::process::exit(1)
                }
            };
            println!("{}", &file_path.display());
            match bundle.save() {
                Ok(()) => {}
                Err(e) => handle_io_error(e),
            };
        }
        Commands::Remove { meta, clear } => {
            let mut bundle = match bundle::Bundle::open(args.path) {
                Ok(v) => v,
                Err(e) => handle_io_error(e),
            };
            let field_meta = match serde_json::from_str(&meta) {
                Ok(v) => v,
                Err(e) => handle_serde_error(e),
            };
            let file_path = match bundle.rm_field(&field_meta) {
                Some(v) => v,
                None => {
                    eprintln!("No existing fields match the given metadata.");
                    eprintln!("No fields deleted.");
                    std::process::exit(1)
                }
            };
            if clear {
                match std::fs::remove_dir_all(&file_path) {
                    Ok(_) => {}
                    Err(e) => handle_io_error(e),
                };
            }
            println!("{}", &file_path.display());
            match bundle.save() {
                Ok(()) => {}
                Err(e) => handle_io_error(e),
            };
        }
    };
}

fn handle_io_error(err: io::Error) -> ! {
    match err {
        _ => eprintln!("{}", err.to_string()),
    }
    std::process::exit(1);
}

fn handle_serde_error(err: serde_json::Error) -> ! {
    match err.classify() {
        _ => eprintln!("{}", err.to_string()),
    }
    std::process::exit(1);
}
