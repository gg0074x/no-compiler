use actions::{compile, decompile};
use args::{Args, Commands};
use clap::Parser;
use tokio::main;
use std::process::ExitCode;

mod args;
mod bandb;
mod actions;

#[main]
async fn main() -> ExitCode {
    let args: Args = Args::parse();

    let format = args
        .format
        .as_deref()
        .unwrap_or("out");

    let result = match args.cmd {
        Commands::Compile { files } => compile(&files, format),
        Commands::Decompile { files } => decompile(&files, format),
        Commands::Lsp => {
            no_compiler_lsp::run().await;
            Ok(())
        }
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("\x1B[31mError running. {err:#}, exiting.\x1B[0m");
            ExitCode::FAILURE
        }
    }
}
