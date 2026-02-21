mod jerry_utils;

use clap::{Parser, Subcommand};
use jerry_utils::folder_maker::create_folder;

#[derive(Parser)]
#[command(name = "jerry")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add,
    Commit,
    Checkout,
    Merge,
    Head,
    Init,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add => println!("add was invoked"),
        Commands::Commit => println!("commit was invoked"),
        Commands::Checkout => println!("checkout was invoked"),
        Commands::Merge => println!("merge was invoked"),
        Commands::Head => println!("head was invoked"),
        Commands::Init => { create_folder().expect("failed to create .jerry directory"); },
    }
}