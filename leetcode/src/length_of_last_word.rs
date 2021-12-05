pub fn length_of_last_word_basic(s: String) -> i32 {
    match s.split_whitespace().last() {
        Some(word) => word.len() as i32,
        None => 0 as i32,
    }
}

pub fn length_of_last_word(s: String) -> i32 {
    let mut count = 0;
    for c in s.chars().rev() {
        if c == ' ' {
            if count > 0 {
                return count;
            }
        } else {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(super::length_of_last_word_basic("".to_string()), 0);
        assert_eq!(super::length_of_last_word_basic("hello".to_string()), 5);
        assert_eq!(
            super::length_of_last_word_basic("hello world".to_string()),
            5
        );
        assert_eq!(
            super::length_of_last_word_basic("hello dumass".to_string()),
            6
        );
        assert_eq!(
            super::length_of_last_word_basic("hello    and welcome  hero".to_string()),
            4
        );
    }

    #[test]
    fn test_advanced() {
        assert_eq!(super::length_of_last_word("".to_string()), 0);
        assert_eq!(super::length_of_last_word("hello".to_string()), 5);
        assert_eq!(super::length_of_last_word("hello world".to_string()), 5);
        assert_eq!(super::length_of_last_word("hello dumass".to_string()), 6);
        assert_eq!(
            super::length_of_last_word("hello    and welcome  hero".to_string()),
            4
        );
        assert_eq!(
            super::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }
}
