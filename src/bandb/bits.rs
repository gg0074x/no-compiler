pub struct Bits {
    pub value: Vec<u8>,
}

impl Bits {
    pub fn from_vec(bits: Vec<u8>) -> Self {
        Bits { value: bits }
    }

    pub fn from_str(str: &str) -> Self {
        let mut vec: Vec<u8> = vec![];
        for b in str.bytes() {
            vec.push(
                u8::try_from(
                    (b as char)
                        .to_digit(10)
                        .expect("Cannot convert vec to digit"),
                )
                .expect("Cannot cast u32 to u8"),
            );
        }
        Bits { value: vec }
    }
}
