use std::{num::{ParseIntError, TryFromIntError}, str::FromStr};
use thiserror::Error;
use crate::traits::try_from_iter::TryFromIterator;

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

    #[error("{0:#}")]
    Creation(#[from] ByteCreationError)
}

#[derive(Debug, Error)]
pub enum ByteCreationError {
    #[error("Couldn't parse byte value")]
    ParseError(#[from] ParseIntError)
}

impl Byte {
    pub fn from_bits(bits: &Bits) -> Result<Self, ByteCreationError> {
        Self::try_from_iter(
            bits
                .value()
                .to_owned()
        )
    }

    pub fn value(&self) -> &u8 {
        &self.value
    }
}

impl TryFromIterator<u8> for Byte {
    type Error = ByteCreationError;

    fn try_from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Result<Self, Self::Error> {
        Ok(Self {
            value: u8::from_str_radix(
                iter
                    .into_iter()
                    .map(|byte| byte.to_string())
                    .collect::<String>()
                    .as_str(),
                2
            )?
        })
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

        Ok(Self::try_from_iter(vec)?)
    }
}
