mod utils;

use clap::{Parser, Subcommand};
use utils::init::create_folder;
use utils::branch::create_branch;
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
    BackFill,
    Init,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add => println!("add was invoked"),
        Commands::Commit => println!("commit was invoked"),
        // take the arg from the user and then pass it here
        Commands::Checkout => create_branch("hi".to_string()),
        Commands::Merge => println!("merge was invoked"),
        Commands::BackFill => println!("BackFill was invoked"),
        Commands::Init => { create_folder().expect("failed to create .jerry directory"); },
    }
}