// crates/dustviz/src/main.rs

use std::process;

use clap::Parser;

use crate::app::{run, AppConfig};
use crate::cli::{Cli, Command};
use crate::util::diagnostics::Diagnostic;

mod app;
mod cli;
mod input;
mod model;
mod util;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Parse { input } => run(AppConfig { input }).map(|_| ()),
        Command::Render { input, .. } => run(AppConfig { input }).map(|_| ()),
    };

    if let Err(err) = result {
        report_error(err);
        process::exit(1);
    }
}

fn report_error(err: Diagnostic) {
    eprintln!("error: {err}");
}