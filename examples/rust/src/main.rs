use n_roman::{from_roman, to_roman};

fn main() {
    println!("3999 in Roman numerals is {}", to_roman(3999));
    println!("MMMCMXCIX in decimal is {}", from_roman("MMMCMXCIX"));
}
