

// "", ".", ",", "-", "~", "+", "=", "@"
const ASCIIS:[&'static str;8] = ["", ".", ",", "-", "~", "+", "=", "@"];

pub fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 32;

    ASCIIS[index as usize]
}

pub fn get_byte_ascii<'a>(intent: u8) -> &'a [u8]{
    let index = intent/32;

    ASCIIS[index as usize].as_bytes()
}