

// "", ".", ",", "-", "~", "+", "=", "@"
const ASCIIS:[&'static str;8] = ["", ".", ",", "-", "~", "+", "=", "@"];

pub fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 32;

    ASCIIS[index as usize]
}