pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    match nums.is_empty() {
        true => 0,
        false => {
            let mut running_index = 0;
            for i in 0..nums.len() {
                if nums[i] != val {
                    nums[running_index] = nums[i];
                    running_index += 1;
                }
            }
            running_index as i32
        }
    }       
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_cases() {
        assert_eq!(super::remove_element(&mut vec![], 1), 0);
        assert_eq!(super::remove_element(&mut vec![3,2,2,3], 3), 2);
        assert_eq!(super::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2), 5);
    }
}