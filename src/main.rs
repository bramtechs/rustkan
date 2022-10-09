use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
struct Board {
    title: String,
    description: String,
    author: String,
    colors: Vec<String>,
}

impl Board {
    fn template() -> Board {
        Board {
            title: "My cool project".to_string(),
            description: "Insert description here".to_string(),
            author: "Anonymous".to_string(),
            colors: vec![
                "red".to_string(),
                "yellow".to_string(),
                "orange".to_string(),
                "green".to_string(),
            ],
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {
    command: String,
    path: Option<PathBuf>,
    path2: Option<PathBuf>,
}

fn export_html(source_path: Option<PathBuf>, path: Option<PathBuf>) {}

fn init_board(path: PathBuf) -> Result<(), String> {
    fs::create_dir_all(path.clone()).map_err(|e| e.to_string())?;

    // check if dir empty
    let count = fs::read_dir(path.clone())
        .map_err(|e| e.to_string())
        .unwrap()
        .count();

    if count == 0 {
        // write config.toml
        let toml_path = path.join("config.toml");

        let template = Board::template();
        let toml_string = toml::to_string_pretty(&template).map_err(|e| e.to_string())?;

        fs::write(toml_path, toml_string).map_err(|e| e.to_string())?;

        // write template csv
        let csv_path = path.join("board.csv");
        let mut csv = csv::Writer::from_path(csv_path).map_err(|e| e.to_string())?;

        csv.write_record(&["TODO", "In-progress", "Backburner", "Done"])
            .map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Err("Target directory not empty".to_string())
    }
}

fn main() {
    let args = Cli::parse();

    if args.path2.is_some() {
        match args.command.as_str() {
            "init" => {
                if args.path.is_some() {
                    init_board(args.path.unwrap())
                        .map_err(|e| println!("{}", e.as_str()))
                        .ok();
                } else {
                    println!("No path specified, pass '.' as argument for current path.");
                }
            }
            "export" => {
                export_html(args.path, args.path2);
            }
            _ => println!("Unknown command given"),
        }
    }
}
