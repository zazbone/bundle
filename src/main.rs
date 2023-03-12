use bundle::file_info::FileInfo;
use bundle::Bundle;
use clap::{Parser, Subcommand};
use serde_json::from_str as json_loads;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Kind,
}

#[derive(Debug, Subcommand)]
enum Kind {
    Init,
    New {
        path: PathBuf,
    },
    Default {
        path: PathBuf,
    },
    Add {
        #[arg(long, short)]
        name: String,
        #[arg(long, short)]
        path: PathBuf,
        #[arg(long, short)]
        interactive: bool,
        #[arg(long, short)]
        meta: Option<String>,
    },
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    match args.command {
        Kind::Init => {
            let cwd = env::current_dir()?;
            let _bundle = Bundle::create(cwd)?;
        }
        Kind::New { path } => {
            fs::create_dir(&path)?;
            let _bundle = Bundle::create_unchecked(path);
        }
        Kind::Default { path } => {
            let bundle = Bundle::open(path)?;
            bundle.set_default()?;
        }
        Kind::Add {
            name,
            path,
            interactive,
            meta,
        } => {
            let bundle = Bundle::open(path)?;
            let info = if interactive {
                unimplemented!("Oskour");
            } else {
                let meta_data: HashMap<String, String> = if let Some(meta_value) = meta {
                    json_loads(&meta_value).unwrap()
                } else {
                    HashMap::default()
                };
                FileInfo::new(name, meta_data)
            };
            match bundle.mkfile(&info, true) {
                Ok(path) => println!("File created at {}", path.to_str().unwrap_or("")),
                Err(()) => panic!("Failed to create file."),
            };
        }
    }
    Ok(())
}
