use std::fs;
use std::path::PathBuf;
use crate::{export_html, init_board};

fn clean() {
    fs::remove_dir_all("./.temp").unwrap_or_default();
}

#[cfg(test)]

#[test]
fn init_project() {
    clean();
    let source = PathBuf::from("./.temp");
    init_board(&source, false).expect("An error occurred!");
    let csv_path = source.join("board.csv");
    fs::read(&csv_path).expect("Did not create csv file!");
    let config_path = source.join("config.toml");
    fs::read(&config_path).expect("Did not create config file!");
    clean();
    assert!(true);
}

#[test]
fn export_project(){
    clean();
    let source = PathBuf::from("./.temp");
    init_board(&source, false).expect("An error occurred!");
    export_html(Some(source),None).expect("An error occured!");
}