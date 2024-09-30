use std::{env::current_dir, fs::{read, read_to_string, File}, io::{Error as IoError, Write}, num::TryFromIntError, path::PathBuf};
use thiserror::Error;
use crate::structures::{bits::Bits, byte::{Byte, ByteCreationError}};

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Error geting current path: {0:#}")]
    CurrentPath(IoError),

    #[error("Path is not a file")]
    StemRetrieval,

    #[error("Error reading file: {0:#}")]
    FileRead(IoError),

    #[error("Error creating output file: {0:#}")]
    OutputCreation(IoError),

    #[error("Error writing output file: {0:#}")]
    WriteOutput(IoError),

    #[error("{0:#}")]
    MakeBytes(#[from] MakeBytesError),

    #[error("No paths specified")]
    NoInput
}

pub fn compile(paths: &[PathBuf], format: &str) -> Result<(), CompilerError> {
    if paths.is_empty() {
        return Err(CompilerError::NoInput);
    }

    let curr_dir = current_dir()
        .map_err(CompilerError::CurrentPath)?;

    for path in paths {
        let mut file = curr_dir
            .clone();

        let file_name = path
            .file_stem()
            .ok_or(CompilerError::StemRetrieval)?;

        let code = read_to_string(path)
            .map_err(CompilerError::FileRead)?;

        file.push(file_name);
        file.set_extension(format);

        let mut file = File::create(file)
            .map_err(CompilerError::OutputCreation)?;

        file
            .write(&make_bytes(&code)?)
            .map_err(CompilerError::WriteOutput)?;
    }

    Ok(())
}

pub fn decompile(paths: &[PathBuf], format: &str) -> Result<(), CompilerError> {
    if paths.is_empty() {
        return Err(CompilerError::NoInput);
    }

    let curr_dir = current_dir()
        .map_err(CompilerError::CurrentPath)?;

    for path in paths {
        let mut file = curr_dir
            .clone();

        let file_name = path
            .file_stem()
            .ok_or(CompilerError::StemRetrieval)?;

        let code = read(path)
            .map_err(CompilerError::FileRead)?;

        file.push(file_name);
        file.set_extension(format);

        let mut file = File::create(file)
            .map_err(CompilerError::OutputCreation)?;

        let mut decompiled = Vec::new();

        for b in code {
            for c in format!("{b:08b}").chars() {
                decompiled.push(c as u8);
            }

            decompiled.push(b' ');
        }

        file
            .write(&decompiled)
            .map_err(CompilerError::WriteOutput)?;
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum MakeBytesError {
    #[error("{0:#}")]
    ByteCreation(#[from] ByteCreationError),

    #[error("{0:#}")]
    Cast(#[from] TryFromIntError),

    #[error("Error converting value to digit")]
    Conversion
}

fn make_bytes(code: &str) -> Result<Vec<u8>, MakeBytesError> {
    let chars: Vec<char> = code
        .chars()
        .collect();

    let mut buff: [u8; 8] = [0; 8];
    let mut buff_index = 0;

    let mut bytes = Vec::new();

    for c in 0..=code.len() {
        if buff_index == 8 {
            bytes.push(
                Byte::from_bits(
                    &Bits::from_iter(buff)
                )?
                    .value()
                    .to_owned()
            );

            buff_index = 0;

        } else {
            if chars.get(c).is_none() {
                return Ok(bytes);
            }

            let chars = chars
                .get(c)
                .unwrap();

            match chars {
                '1' | '0' => {
                    buff[buff_index] = u8::try_from(
                        chars
                            .to_digit(10)
                            .ok_or(MakeBytesError::Conversion)?
                    )?;
                }
                _ => {}
            }
        }
    }

    Ok(bytes)
}
