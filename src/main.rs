use std::path::PathBuf;

use clap::Parser;

#[tokio::main]
async fn main() {
    println!("testing...");
}

#[derive(Parser, Debug)]
struct Args {
    steam_path: PathBuf,
}
