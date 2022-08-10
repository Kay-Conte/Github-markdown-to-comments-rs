use std::{
    io::{self, Write},
    path::PathBuf,
};

use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::{fs::OpenOptions, io::stdin};

use tokio::fs::File;

const OUTPUT_DIR: &str = "output.txt";

async fn get_input() -> String {
    let mut buffer = BufReader::new(stdin()).lines();

    let line = buffer.next_line().await;

    if let Ok(line) = line {
        if let Some(line) = line {
            return line;
        }
    }

    String::new()
}

async fn generate_file(input: PathBuf) {
    let path = std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("No executable parent directory?")
        .join(PathBuf::from(OUTPUT_DIR));

    let mut options = OpenOptions::new();

    options.write(true).create(true).truncate(true);

    let mut output_file = options.open(path.as_path()).await.unwrap();

    let mut input_buffer =
        BufReader::new(File::open(input).await.expect("Failed to open input file")).lines();

    while let Some(line) = input_buffer.next_line().await.expect("Failed to read file") {
        let _ = output_file
            .write_all(format!("//! {}\n", line).as_bytes())
            .await;
    }
}

#[tokio::main]
async fn main() {
    print!("Enter the path of your github markdown file: ");
    io::stdout().flush().unwrap();

    let input = get_input().await;

    generate_file(PathBuf::from(&input)).await;

    println!("Generated \"output.txt\" file in the root directory");
}
