
pub use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum KeySym {
    KEY_NONE,
    KEY_a,
    KEY_b,
    KEY_c,
    KEY_d,
    KEY_e,
    KEY_f,
    KEY_g,
    KEY_h,
    KEY_i,
    KEY_j,
    KEY_k,
    KEY_l,
    KEY_m,
    KEY_n,
    KEY_o,
    KEY_p,
    KEY_q,
    KEY_r,
    KEY_s,
    KEY_t,
    KEY_u,
    KEY_v,
    KEY_w,
    KEY_x,
    KEY_y,
    KEY_z,
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_I,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_M,
    KEY_N,
    KEY_O,
    KEY_P,
    KEY_Q,
    KEY_R,
    KEY_S,
    KEY_T,
    KEY_U,
    KEY_V,
    KEY_W,
    KEY_X,
    KEY_Y,
    KEY_Z,
}

impl FromStr for KeySym {

    type Err = ();

    fn from_str(input: &str) -> Result<KeySym, Self::Err> {
        match input {
            "a" => Ok(KeySym::KEY_a),
            "b" => Ok(KeySym::KEY_b),
            "c" => Ok(KeySym::KEY_c),
            "d" => Ok(KeySym::KEY_d),
            "e" => Ok(KeySym::KEY_e),
            "f" => Ok(KeySym::KEY_f),
            "g" => Ok(KeySym::KEY_g),
            "h" => Ok(KeySym::KEY_h),
            "i" => Ok(KeySym::KEY_i),
            "j" => Ok(KeySym::KEY_j),
            "k" => Ok(KeySym::KEY_k),
            "l" => Ok(KeySym::KEY_l),
            "m" => Ok(KeySym::KEY_m),
            "n" => Ok(KeySym::KEY_n),
            "o" => Ok(KeySym::KEY_o),
            "p" => Ok(KeySym::KEY_p),
            "q" => Ok(KeySym::KEY_q),
            "r" => Ok(KeySym::KEY_r),
            "s" => Ok(KeySym::KEY_s),
            "t" => Ok(KeySym::KEY_t),
            "u" => Ok(KeySym::KEY_u),
            "v" => Ok(KeySym::KEY_v),
            "w" => Ok(KeySym::KEY_w),
            "x" => Ok(KeySym::KEY_x),
            "y" => Ok(KeySym::KEY_y),
            "z" => Ok(KeySym::KEY_z),
            "A" => Ok(KeySym::KEY_A),
            "B" => Ok(KeySym::KEY_B),
            "C" => Ok(KeySym::KEY_C),
            "D" => Ok(KeySym::KEY_D),
            "E" => Ok(KeySym::KEY_E),
            "F" => Ok(KeySym::KEY_F),
            "G" => Ok(KeySym::KEY_G),
            "H" => Ok(KeySym::KEY_H),
            "I" => Ok(KeySym::KEY_I),
            "J" => Ok(KeySym::KEY_J),
            "K" => Ok(KeySym::KEY_K),
            "L" => Ok(KeySym::KEY_L),
            "M" => Ok(KeySym::KEY_M),
            "N" => Ok(KeySym::KEY_N),
            "O" => Ok(KeySym::KEY_O),
            "P" => Ok(KeySym::KEY_P),
            "Q" => Ok(KeySym::KEY_Q),
            "R" => Ok(KeySym::KEY_R),
            "S" => Ok(KeySym::KEY_S),
            "T" => Ok(KeySym::KEY_T),
            "U" => Ok(KeySym::KEY_U),
            "V" => Ok(KeySym::KEY_V),
            "W" => Ok(KeySym::KEY_W),
            "X" => Ok(KeySym::KEY_X),
            "Y" => Ok(KeySym::KEY_Y),
            "Z" => Ok(KeySym::KEY_Z),
            _ => Err(()),
        }
    }
}

impl KeySym {
    pub fn as_char(&self) -> Option<char> {
        char::from_u32(
            (match self {
                KeySym::KEY_A => 0x41,
                KeySym::KEY_B => 0x42,
                KeySym::KEY_C => 0x43,
                KeySym::KEY_D => 0x44,
                KeySym::KEY_E => 0x45,
                KeySym::KEY_F => 0x46,
                KeySym::KEY_G => 0x47,
                KeySym::KEY_H => 0x48,
                KeySym::KEY_I => 0x49,
                KeySym::KEY_J => 0x4a,
                KeySym::KEY_K => 0x4b,
                KeySym::KEY_L => 0x4c,
                KeySym::KEY_M => 0x4d,
                KeySym::KEY_N => 0x4e,
                KeySym::KEY_O => 0x4f,
                KeySym::KEY_P => 0x50,
                KeySym::KEY_Q => 0x51,
                KeySym::KEY_R => 0x52,
                KeySym::KEY_S => 0x53,
                KeySym::KEY_T => 0x54,
                KeySym::KEY_U => 0x55,
                KeySym::KEY_V => 0x56,
                KeySym::KEY_W => 0x57,
                KeySym::KEY_X => 0x58,
                KeySym::KEY_Y => 0x59,
                KeySym::KEY_Z => 0x5a,
                KeySym::KEY_a => 0x61,
                KeySym::KEY_b => 0x62,
                KeySym::KEY_c => 0x63,
                KeySym::KEY_d => 0x64,
                KeySym::KEY_e => 0x65,
                KeySym::KEY_f => 0x66,
                KeySym::KEY_g => 0x67,
                KeySym::KEY_h => 0x68,
                KeySym::KEY_i => 0x69,
                KeySym::KEY_j => 0x6a,
                KeySym::KEY_k => 0x6b,
                KeySym::KEY_l => 0x6c,
                KeySym::KEY_m => 0x6d,
                KeySym::KEY_n => 0x6e,
                KeySym::KEY_o => 0x6f,
                KeySym::KEY_p => 0x70,
                KeySym::KEY_q => 0x71,
                KeySym::KEY_r => 0x72,
                KeySym::KEY_s => 0x73,
                KeySym::KEY_t => 0x74,
                KeySym::KEY_u => 0x75,
                KeySym::KEY_v => 0x76,
                KeySym::KEY_w => 0x77,
                KeySym::KEY_x => 0x78,
                KeySym::KEY_y => 0x79,
                KeySym::KEY_z => 0x7a,
                _ => 0x00,
            } as u32)
        )
    }
}