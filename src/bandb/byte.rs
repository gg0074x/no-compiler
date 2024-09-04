pub struct Byte {
    pub value: u8,
}

use crate::bits::Bits;

impl Byte {
    pub fn from_bits(bits: &Bits) -> Self {
        let byte = convert_vec_to_u8(&bits.value);
        Byte { value: byte }
    }

    pub fn from_str(str: &str) -> Self {
        let mut vec: Vec<u8> = vec![];
        for b in str.bytes() {
            vec.push(u8::try_from((b as char).to_digit(10).expect("Cannot convert vec to digit")).expect("Cannot cast u32 to u8"));
        }
        let byte = convert_vec_to_u8(&vec);
        Byte { value: byte }
    }
}

fn convert_vec_to_u8(vec: &[u8]) -> u8 {
    let binary_chain: String = vec.iter().map(|&b| b.to_string()).collect();
    u8::from_str_radix(&binary_chain, 2).expect("Invalid binary slice")
}
