use clap::Parser;
use cli::Cli;

pub mod cli;
pub mod model;
pub mod parser;
pub mod server;
pub mod snowball;

pub mod util;

// Search for this "Hola amigos"

fn main() {
    let cli = Cli::parse();
    let _ = cli.run();
}
