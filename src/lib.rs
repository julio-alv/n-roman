
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn to_roman(num: i32) -> *const c_char {
    if num <= 0 {
        return CString::new("Invalid number").unwrap().into_raw();
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

    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn from_roman(roman: *const c_char) -> i32 {
    
    let roman_chars: Vec<char> = unsafe {
        let c_str = CString::from_raw(roman as *mut c_char);
        c_str.to_string_lossy().into_owned().chars().collect()
    };
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

#[no_mangle]
pub extern "C" fn string_to_number(s: *const c_char) -> i32 {
    unsafe {
        let c_str = CStr::from_ptr(s);
        match c_str.to_str() {
            Ok(string) => match string.parse::<i32>() {
                Ok(number) => number,
                Err(_) => -1, // return -1 if the string cannot be converted to a number
            },
            Err(_) => -1, // return -1 if the input string is not a valid C string
        }
    }
}