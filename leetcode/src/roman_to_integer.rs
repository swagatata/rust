pub fn roman_to_int(s: String) -> i32 {
    let mut total: i32 = 0;
    let mut last_biggest: i32 = 0;
    for c in s.chars().rev() {
        // println!("{}", c);
        let digit = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Invalid input"),
        };
        if digit >= last_biggest {
            total += digit;
            last_biggest = digit;
        } else {
            total -= digit;
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_easy_roman_to_int() {
        assert_eq!(super::roman_to_int("I".to_string()), 1);
        assert_eq!(super::roman_to_int("II".to_string()), 2);
        assert_eq!(super::roman_to_int("III".to_string()), 3);
        assert_eq!(super::roman_to_int("V".to_string()), 5);
        assert_eq!(super::roman_to_int("X".to_string()), 10);
        assert_eq!(super::roman_to_int("L".to_string()), 50);
        assert_eq!(super::roman_to_int("C".to_string()), 100);
        assert_eq!(super::roman_to_int("D".to_string()), 500);
    }

    #[test]
    fn test_hard_roman_to_int() {
        assert_eq!(super::roman_to_int("VIII".to_string()), 8);

        assert_eq!(super::roman_to_int("IV".to_string()), 4);
        assert_eq!(super::roman_to_int("IX".to_string()), 9);

        assert_eq!(super::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(super::roman_to_int("LIX".to_string()), 59);

        assert_eq!(super::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
