use std::{
    fs::{read, read_to_string, File},
    io::Write,
    vec,
};
mod bandb;

use bandb::{bits, byte};
use bits::Bits;
use byte::Byte;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_name = "FILE PATH", index = 1, help = "Input file to compile", required = true)]
    path: Vec<String>,

    #[arg(short, long, help = "Output file path")]
    output: Option<String>,

    #[arg(short, long, help = "File extension")]
    format: Option<String>,

    #[arg(long = "decomp", help = "Decompile input file")]
    decompile: bool,
}

fn main() {
    let args: Args = Args::parse();

    let paths: Vec<String> = args.path;

    let output: String = args.output.unwrap_or(String::from("a"));
    let format: String = args.format.unwrap_or(String::from("out"));

    if args.decompile{
        decompile(&paths, &output, &format);
    }else {
        compile(&paths, &output, &format);
    }
}

fn compile(paths: &[String], output: &str, format: &str) {
    for (i, path) in paths.iter().enumerate() {
        let code: String = read_to_string(path).expect("Cannot read file as string");

        let bytes: Vec<u8> = make_bytes(&code);
        let mut file_name: String = format!("{output}{i}.{format}");
        if i == 0 {
            file_name = format!("{output}.{format}");
        }
        let mut file: File = File::create(file_name).expect("Cannot create file");

        let _written = file.write(&bytes).expect("Bytes cannot be written");
    }
}

fn decompile(paths: &[String], output: &str, format: &str) {
    for (i, path) in paths.iter().enumerate() {
        let code: Vec<u8> = read(path).expect("Cannot read file");

        let mut file_name: String = format!("{output}{i}.{format}");
        if i == 0 {
            file_name = format!("{output}.{format}");
        }
        let mut file: File = File::create(file_name).expect("Cannot create file");

        let mut decompiled: Vec<u8> = vec![];
        for b in code {
            for c in format!("{b:08b}").chars(){
                decompiled.push(c as u8);
            }
            decompiled.push(b' ');
        }
        let _written = file.write(&decompiled).expect("Bytes cannot be written");
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
