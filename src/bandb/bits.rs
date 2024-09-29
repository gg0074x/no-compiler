use std::{num::TryFromIntError, str::FromStr};
use thiserror::Error;

pub struct Bits {
    value: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum BitsConversionError {
    #[error("Cannot convert byte to digit")]
    ConversionError,

    #[error("Cannot downcast value to u8")]
    CastError(#[from] TryFromIntError),
}

impl Bits {
    pub fn from_iter(bits: &[u8]) -> Self {
        Bits { value: bits.into() }
    }

    pub fn value(&self) -> &Vec<u8> {
        &self.value
    }
}

impl FromStr for Bits {
    type Err = BitsConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = Vec::new();

        for b in s.bytes() {
            vec.push(
                u8::try_from(
                    (b as char)
                        .to_digit(10)
                        .ok_or(BitsConversionError::ConversionError)?
                )?,
            );
        }

        Ok(Self {
            value: vec
        })
    }
}
