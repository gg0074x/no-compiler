use std::{num::{ParseIntError, TryFromIntError}, str::FromStr};
use thiserror::Error;
use super::bits::Bits;

pub struct Byte {
    value: u8,
}

#[derive(Debug, Error)]
pub enum ByteConversionError {
    #[error("Cannot convert byte to digit")]
    Conversion,

    #[error("Cannot downcast value to u8")]
    Cast(#[from] TryFromIntError),

    #[error("{0:?}")]
    Creation(#[from] ByteCreationError)
}

#[derive(Debug, Error)]
pub enum ByteCreationError {
    #[error("Couldn't parse byte value")]
    ParseError(#[from] ParseIntError)
}

impl Byte {
    pub fn from_bits(bits: &Bits) -> Result<Self, ByteCreationError> {
        Self::from_iter(bits.value())
    }

    pub fn from_iter(iter: &[u8]) -> Result<Self, ByteCreationError> {
        Ok(Self {
            value: u8::from_str_radix(
                iter
                    .iter()
                    .map(|&byte| byte.to_string())
                    .collect::<String>()
                    .as_str(),
                2
            )?
        })
    }

    pub fn value(&self) -> &u8 {
        &self.value
    }
}

impl FromStr for Byte {
    type Err = ByteConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = Vec::new();

        for byte in s.bytes() {
            vec.push(
                u8::try_from(
                    (byte as char)
                        .to_digit(10)
                        .ok_or(ByteConversionError::Conversion)?
                )?
            )
        }

        Ok(Self::from_iter(&vec)?)
    }
}
