use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Output extensions
    #[arg(short, long)]
    pub format: Option<String>,

    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Compile files into actual binary.
    #[clap(alias = "build")]
    Compile {
        /// Input files to compile.
        #[arg(value_name = "FILE PATH", index = 1)]
        files: Vec<PathBuf>,
    },

    /// Decompile files into readable binary.
    #[clap(alias = "decomp")]
    Decompile {
        /// Input files to decompile.
        #[arg(value_name = "FILE PATH", index = 1)]
        files: Vec<PathBuf>,
    },

    /// Start a LSP within the program STDIO.
    Lsp,
}
