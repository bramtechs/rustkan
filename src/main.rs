#![recursion_limit = "512"]

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use typed_html::{dom::DOMTree, html, text};

static CSS_FILE: &'static str = include_str!("kanban.css");
static CSS_RESET_FILE: &'static str = include_str!("reset.css");

#[derive(Serialize, Deserialize, Debug)]
struct Board {
    title: String,
    description: String,
    author: String,
}

impl Board {
    fn template() -> Board {
        Board {
            title: "My cool project".to_string(),
            description: "Insert description here".to_string(),
            author: "Anonymous".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {
    command: String,
    path: Option<PathBuf>,
    path2: Option<PathBuf>,
}

fn export_html(source_path: Option<PathBuf>, dest_path: Option<PathBuf>) -> Result<(), String> {
    let current = std::env::current_exe().map_err(|e| e.to_string())?;
    let source_path = source_path.unwrap_or(current);
    let dest_path = dest_path.unwrap_or(source_path.clone());

    // load config
    let config_path = source_path.join("config.toml");
    let config_content = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let config: Board = toml::from_str(&config_content).map_err(|e| e.to_string())?;

    // load csv
    let csv_path = source_path.join("board.csv");
    let mut csv = csv::Reader::from_path(csv_path).map_err(|e| e.to_string())?;

    // parse csv
    let mut items = Vec::new();
    let headers = csv.headers().map_err(|e| e.to_string())?.clone();

    // add headers
    let mut id = 0;
    for _ in headers.iter() {
        let mut array = Vec::<String>::new();
        array.push(
            headers
                .get(id)
                .expect("Could not determine header")
                .to_string(),
        );
        items.push(array);
        id += 1;
    }

    // add cells
    let mut id = 0;
    csv.records().for_each(|r| {
        id = 0;
        for column in r.iter() {
            for col in column.iter() {
                if !col.eq_ignore_ascii_case("") {
                    // ignore empty cells
                    items
                        .get_mut(id)
                        .expect("Could not get column list")
                        .push(col.to_string());
                }
                id += 1;
            }
        }
    });

    // get curren date
    let date_time = chrono::offset::Utc::now();
    let date_time_string = date_time.format("%d/%m/%Y %H:%M UTC").to_string();

    let html: DOMTree<String> = html! (
        <html>
            <head>
                <title>{text!(config.title.clone())}</title>
                <link rel="stylesheet" href="reset.css"/>
                <link rel="stylesheet" href="kanban.css"/>
            </head>
            <body>
                <header>
                    <h1>{text!(config.title)}</h1>
                    <p>{text!(config.description)}</p>
                    <time>"Last edited on "{text!(date_time_string)}</time>
                </header>
                <ul>
                {
                    items.iter().map(|c| html!(
                        <li>
                            <ul>
                            {
                                c.iter().map(|e| html!(
                                    <li>{text!(e)}</li>
                                ))
                            }
                            </ul>
                        </li>
                    ))
                }
                </ul>
                <footer>
                        <p>"Statically generated with "<a href="https://github.com/bramtechs/rust-html-kanboard" target="_blank">"with Rust!"</a></p>
                </footer>
            </body>
        </html>
    );

    // write generated html
    let html_string = html.to_string();
    let html_path = source_path.join("kanboard.html");
    fs::write(html_path, html_string).map_err(|e| e.to_string())?;

    // write template css file
    let css_path = dest_path.join("kanban.css");
    fs::write(css_path, CSS_FILE).map_err(|e| e.to_string())?;

    // write reset css file
    let css_reset_path = dest_path.join("reset.css");
    fs::write(css_reset_path, CSS_RESET_FILE).map_err(|e| e.to_string())?;
    Ok(())
}

fn init_board(path: PathBuf, force: bool) -> Result<(), String> {
    fs::create_dir_all(path.clone()).map_err(|e| e.to_string())?;

    // check if dir empty
    let count = fs::read_dir(path.clone())
        .map_err(|e| e.to_string())
        .unwrap()
        .count();

    if count == 0 || force {
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
        csv.write_record(&[
            "Implement feature",
            "Rewrite in Rust",
            "Fix bug",
            "Host repo",
        ])
        .map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Err("Target directory not empty".to_string())
    }
}

fn main() {
    let args = Cli::parse();

    match args.command.as_str() {
        "init" => {
            if args.path.is_some() {
                init_board(args.path.unwrap(), false)
                    .map_err(|e| println!("{}", e.as_str()))
                    .ok();
            } else {
                println!("No path specified, pass '.' as argument for current path.");
            }
        }
        "export" => {
            export_html(args.path, args.path2)
                .map_err(|e| println!("{}", e.to_string()))
                .ok();
        }
        _ => println!("Unknown command given"),
    }
}
