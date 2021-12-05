pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    use std::cmp;

    if nums.is_empty() {
        return 0;
    }
    let mut current_num = nums[0];    
    let mut current_index = 1;
    for i in 1..nums.len() {
        match current_num.cmp(&nums[i]) {
            cmp::Ordering::Less => {
                nums[current_index] = nums[i];
                current_index += 1;
                current_num = nums[i];
            },
            cmp::Ordering::Equal => {},
            cmp::Ordering::Greater => panic!(),
        }
    }
    current_index as i32
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_cases() {
        assert_eq!(super::remove_duplicates(&mut vec![]), 0);
        assert_eq!(super::remove_duplicates(&mut vec![1]), 1);
        assert_eq!(super::remove_duplicates(&mut vec![1,1,2]), 2);
        assert_eq!(super::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
        assert_eq!(super::remove_duplicates(&mut vec![0,1,2,3,4]), 5);
    }
}