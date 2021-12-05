pub fn climb_stairs(n: i32) -> i32 {    
    match n {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => {
            let mut distinct_ways = vec![0; (n+1)as usize];
            distinct_ways[1] = 1;
            distinct_ways[2] = 2;
            for i in 3..n+1 {
                distinct_ways[i as usize] = distinct_ways[(i-1) as usize] + distinct_ways[(i-2) as usize];
            }
        
            distinct_ways[n as usize]
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_cases() {
        assert_eq!(super::climb_stairs(1), 1);
        assert_eq!(super::climb_stairs(2), 2);
        assert_eq!(super::climb_stairs(3), 3);
    }
}