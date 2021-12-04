pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }

    let mut digits = vec![];
    let mut remainder = x;
    while remainder > 0 {
        digits.push(remainder % 10);
        remainder /= 10;
    }

    for i in 0..(digits.len() / 2) {
        if digits[i] != digits[digits.len() - 1 - i] {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod decode_tets {
    #[test]
    fn test_is_palindrome() {
        assert!(!super::is_palindrome(-1));
        assert!(super::is_palindrome(1));
        assert!(super::is_palindrome(9));
        assert!(super::is_palindrome(11));
        assert!(!super::is_palindrome(10));
        assert!(super::is_palindrome(12321));
    }
}
