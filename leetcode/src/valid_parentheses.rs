mod valid_parentheses {
    // This is the full solution 
    // Uses a stack which keeps track of each open bracket
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return false;
        }
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            match c {
                '(' => stack.push(c),
                '{' => stack.push(c),
                '[' => stack.push(c),
                ')' => match stack.pop() {
                    Some('(') => continue,
                    None => return false,
                    _ => return false,
                },
                '}' => match stack.pop() {
                    Some('{') => continue,
                    None => return false,
                    _ => return false,
                },
                ']' => match stack.pop() {
                    Some('[') => continue,
                    None => return false,
                    _ => return false,
                },
                _ => panic!(),
            }
        }
        stack.is_empty()
    }

    fn turn_bracket(c: char) -> char {
        match c {
            '}' => '{',
            ')' => '(',
            ']' => '[',
            '{' => '}',
            '(' => ')',
            '[' => ']',
            _ => panic!(),
        }
    }

    // This is an alternative solution which counts instances of a bracket instead of storing all instances
    // use minimal memory
    pub fn is_valid_advanced(s: String) -> bool {
        use std::cmp;

        if s.is_empty() {
            return false;
        }
        let mut stack: Vec<(char, i32)> = vec![];
        for c in s.chars() {
            // println!("Current char is {}", c);
            // println!("Stack size is {}", stack.len());
            // if stack.len() > 0 {
            //     println!(
            //         "Stack last element is ({}, {})",
            //         stack.last().unwrap().0,
            //         stack.last().unwrap().1
            //     );
            // }
            match c {
                '(' | '{' | '[' => {
                    if stack.is_empty() {
                        stack.push((c, 1));
                    } else {
                        let (character, count) = stack.pop().unwrap();
                        if character == c {
                            stack.push((c, count + 1));
                        } else {
                            stack.push((character, count));
                            stack.push((c, 1));
                        }
                    }
                    // println!("Pushed {} to stack", c);
                }
                ')' | '}' | ']' => match stack.pop() {
                    Some((character, count)) => {
                        // println!("popped char is {}", character);
                        match c.cmp(&turn_bracket(character)) {
                            cmp::Ordering::Equal => {}
                            cmp::Ordering::Less => return false,
                            cmp::Ordering::Greater => return false,
                        }
                        // println!("debug count {}", count);
                        match count.cmp(&1) {
                            cmp::Ordering::Equal => {}
                            cmp::Ordering::Greater => {
                                stack.push((character, count - 1));
                            }
                            cmp::Ordering::Less => panic!(),
                        }
                    }
                    None => {
                        // println!("none");
                        return false;
                    }
                },
                _ => panic!(),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_easy_parentheses() {
        assert!(!super::valid_parentheses::is_valid("".to_string()));
        assert!(super::valid_parentheses::is_valid("()".to_string()));
        assert!(super::valid_parentheses::is_valid("{}".to_string()));
        assert!(super::valid_parentheses::is_valid("[]".to_string()));
    }
    #[test]
    fn test_negative_parentheses() {
        assert!(!super::valid_parentheses::is_valid(")".to_string()));
        assert!(!super::valid_parentheses::is_valid("]".to_string()));
        assert!(!super::valid_parentheses::is_valid("()]".to_string()));
        assert!(!super::valid_parentheses::is_valid("(])".to_string()));
        assert!(!super::valid_parentheses::is_valid("(".to_string()));
    }
    #[test]
    fn test_positive_parentheses() {
        assert!(super::valid_parentheses::is_valid("({[]})".to_string()));
        assert!(super::valid_parentheses::is_valid("()[]{}".to_string()));
        assert!(super::valid_parentheses::is_valid("([]{})".to_string()));

        assert!(super::valid_parentheses::is_valid(
            "((())[{()([])}]{})".to_string()
        ));
    }
}

#[cfg(test)]
mod advanced_is_valid_tests {
    #[test]
    fn test_easy_parentheses() {
        assert!(!super::valid_parentheses::is_valid_advanced("".to_string()));
        assert!(super::valid_parentheses::is_valid_advanced(
            "()".to_string()
        ));
        assert!(super::valid_parentheses::is_valid_advanced(
            "{}".to_string()
        ));
        assert!(super::valid_parentheses::is_valid_advanced(
            "[]".to_string()
        ));
    }
    #[test]
    fn test_negative_parentheses() {
        assert!(!super::valid_parentheses::is_valid_advanced(
            ")".to_string()
        ));
        assert!(!super::valid_parentheses::is_valid_advanced(
            "]".to_string()
        ));
        assert!(!super::valid_parentheses::is_valid_advanced(
            "()]".to_string()
        ));
        assert!(!super::valid_parentheses::is_valid_advanced(
            "(])".to_string()
        ));
        assert!(!super::valid_parentheses::is_valid_advanced(
            "(".to_string()
        ));
    }
    #[test]
    fn test_positive_parentheses() {
        assert!(super::valid_parentheses::is_valid_advanced(
            "({[]})".to_string()
        ));
        assert!(super::valid_parentheses::is_valid_advanced(
            "{[]}".to_string()
        ));
        assert!(super::valid_parentheses::is_valid_advanced(
            "([]{})".to_string()
        ));

        assert!(super::valid_parentheses::is_valid_advanced(
            "((())[{()([])}]{})".to_string()
        ));
    }
}
