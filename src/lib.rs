pub fn to_roman(num: i32) -> String {
    if num <= 0 {
        return String::new();
    }

    let mut result: String = String::new();
    let mut n: i32 = num;

    for (value, symbol) in [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ] {
        while n >= value {
            result.push_str(symbol);
            n -= value;
        }
    }

    result
}

pub fn from_roman(roman: &str) -> i32 {
    let roman_chars: Vec<char> = roman.chars().collect();
    let mut num: i32 = 0;

    for i in 0..roman_chars.len() {
        let cur_val = match roman_chars[i] {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0, // invalid character
        };

        let next_val = if i + 1 < roman_chars.len() {
            match roman_chars[i + 1] {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0, // invalid character
            }
        } else {
            0
        };

        if cur_val < next_val {
            num -= cur_val;
        } else {
            num += cur_val;
        }
    }

    num
}


#[cfg(test)]
mod tests {
    use super::{from_roman, to_roman};

    #[test]
    fn three_thousand_nine_hundred_ninety_nine() {
        assert_eq!(to_roman(3999), "MMMCMXCIX");
        assert_eq!(from_roman("MMMCMXCIX"), 3999);
    }

    #[test]
    fn one_thousand() {
        assert_eq!(to_roman(1000), "M");
        assert_eq!(from_roman("M"), 1000);
    }

    #[test]
    fn nine_hundred() {
        assert_eq!(to_roman(900), "CM");
        assert_eq!(from_roman("CM"), 900);
    }

    #[test]
    fn five_hundred() {
        assert_eq!(to_roman(500), "D");
        assert_eq!(from_roman("D"), 500);
    }

    #[test]
    fn four_hundred() {
        assert_eq!(to_roman(400), "CD");
        assert_eq!(from_roman("CD"), 400);
    }

    #[test]
    fn one_hundred() {
        assert_eq!(to_roman(100), "C");
        assert_eq!(from_roman("C"), 100);
    }

    #[test]
    fn ninety() {
        assert_eq!(to_roman(90), "XC");
        assert_eq!(from_roman("XC"), 90);
    }

    #[test]
    fn fifty() {
        assert_eq!(to_roman(50), "L");
        assert_eq!(from_roman("L"), 50);
    }

    #[test]
    fn forty() {
        assert_eq!(to_roman(40), "XL");
        assert_eq!(from_roman("XL"), 40);
    }

    #[test]
    fn ten() {
        assert_eq!(to_roman(10), "X");
        assert_eq!(from_roman("X"), 10);
    }

    #[test]
    fn nine() {
        assert_eq!(to_roman(9), "IX");
        assert_eq!(from_roman("IX"), 9);
    }

    #[test]
    fn five() {
        assert_eq!(to_roman(5), "V");
        assert_eq!(from_roman("V"), 5);
    }

    #[test]
    fn four() {
        assert_eq!(to_roman(4), "IV");
        assert_eq!(from_roman("IV"), 4);
    }

    #[test]
    fn one() {
        assert_eq!(to_roman(1), "I");
        assert_eq!(from_roman("I"), 1);
    }
}
