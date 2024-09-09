use std::{
    fs::{read, read_to_string, File},
    io::Write,
    path::PathBuf,
    vec,
};

mod args;
mod bandb;

use args::{Args, Commands};
use bandb::{bits, byte};
use bits::Bits;
use byte::Byte;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let format = args.format.as_deref().unwrap_or("out");

    match args.cmd {
        Commands::Compile { files } => compile(&files, format),
        Commands::Decompile { files } => decompile(&files, format),
        Commands::Lsp => no_compiler_lsp::run().await
    }
}

fn compile(paths: &[PathBuf], format: &str) {
    let curr_dir = std::env::current_dir().expect("Cannot get current path");
    for path in paths {
        let mut file = curr_dir.clone();
        let file_ext = path
            .extension()
            .map(|n| n.to_str().unwrap())
            .expect("Cannot get extension of file");
        let file_name = path
            .file_name()
            .map(|n| n.to_str().unwrap().replace(file_ext, ""))
            .expect("Cannot get file name of file");
        let code = read_to_string(path).expect("Cannot read file as string");

        let bytes = make_bytes(&code);
        file.set_file_name(file_name);
        file.set_file_name(format);
        let mut file = File::create(file).expect("Cannot create file");

        let _ = file.write(&bytes).expect("Bytes cannot be written");
    }
}

fn decompile(paths: &[PathBuf], format: &str) {
    let curr_dir = std::env::current_dir().expect("Cannot get current path");
    for path in paths {
        let mut file = curr_dir.clone();
        let file_ext = path
            .extension()
            .map(|n| n.to_str().unwrap())
            .expect("Cannot get extension of file");
        let file_name = path
            .file_name()
            .map(|n| n.to_str().unwrap().replace(file_ext, ""))
            .expect("Cannot get file name of file");
        let code = read(path).expect("Cannot read file");

        file.set_file_name(file_name);
        file.set_file_name(format);
        let mut file = File::create(file).expect("Cannot create file");

        let mut decompiled = vec![];
        for b in code {
            for c in format!("{b:08b}").chars() {
                decompiled.push(c as u8);
            }
            decompiled.push(b' ');
        }
        let _ = file.write(&decompiled).expect("Bytes cannot be written");
    }
}

fn make_bytes(code: &str) -> Vec<u8> {
    let chars: Vec<char> = code.chars().collect();
    let mut buff: Vec<u8> = vec![];
    let mut bytes: Vec<u8> = vec![];

    for c in 0..=code.len() {
        if buff.len() == 8 {
            bytes.push(Byte::from_bits(&Bits::from_vec(buff)).value);
            buff = vec![];
        } else {
            if chars.get(c).is_none() {
                return bytes;
            }
            match chars.get(c).expect("Cannot index chars") {
                '1' | '0' => {
                    buff.push(
                        u8::try_from(
                            chars
                                .get(c)
                                .expect("Cannot index chars")
                                .to_digit(10)
                                .expect("Cannot convert Vec to Digit"),
                        )
                        .expect("Cannot cast u32 as u8"),
                    );
                }
                _ => {}
            }
        }
    }

    bytes
}
