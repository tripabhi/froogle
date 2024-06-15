use clap::Parser;
use clap::ValueHint;
use log::LevelFilter;
use simple_logger::SimpleLogger;

use std::env::current_dir;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::model::cache;
use crate::server;
use crate::util;

pub mod dirs;
pub mod worker;

/// A local search engine
#[derive(Debug, Clone, Parser)]
#[command(name = "Froogle", bin_name = "froogle")]
#[command(version, about, long_about=None, rename_all = "kebab-case")]
pub struct Cli {
    /// Search path, defaults to current directory
    #[arg(value_parser, value_hint = ValueHint::DirPath)]
    path: Option<PathBuf>,

    /// Verbose output with debug logs
    #[arg(short, long, value_parser)]
    verbose: bool,
}

fn enable_logger(level: LevelFilter) {
    SimpleLogger::new()
        .with_level(LevelFilter::Warn)
        .with_module_level("froogle", level)
        .init()
        .map_err(|err| {
            eprintln!("Error while setting logger : {err}");
        })
        .ok();
}

fn _print_banner() {
    println!();
    println!();
    println!();
    println!("   ███████╗ ███████═╗  ██████╗   ██████╗   ██████╗  ██╗     ███████╗ ");
    println!("   ██╔════╝ ██╔═══██║ ██╔═══██╗ ██╔═══██╗ ██╔════╝  ██║     ██╔════╝ ");
    println!("   ██████╗  ████████║ ██║   ██║ ██║   ██║ ██║       ██║     ██████╗  ");
    println!("   ██╔═══╝  ██╔═██║   ██║   ██║ ██║   ██║ ██║█████╗ ██║     ██╔═══╝  ");
    println!("   ██║      ██║  ██║  ╚██████╔╝ ╚██████╔╝ ╚███╔╝██║ ██████╗ ███████╗ ");
    println!("   ╚═╝      ╚═╝  ╚═╝   ╚═════╝   ╚═════╝   ╚═════╝   ╚════╝ ╚══════╝");
    println!();
    println!();
    println!();
}

impl Cli {
    pub fn run(&self) -> Result<(), ()> {
        let level = if self.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };

        enable_logger(level);

        let search_path = match &self.path {
            Some(path) => util::expand_path(path.to_string_lossy())?,
            None => current_dir().map_err(|err| {
                log::error!("Failed to get current_dir : {err}");
            })?,
        };

        let model = Arc::new(Mutex::new(
            cache::get_model(&search_path).unwrap_or_default(),
        ));

        worker::start_indexing(&search_path, Arc::clone(&model));

        server::start("localhost:6969", Arc::clone(&model))?;

        Ok(())
    }
}
