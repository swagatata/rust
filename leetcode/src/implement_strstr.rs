struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let index_pairs: Vec<_> = haystack.match_indices(&needle).collect();
        if index_pairs.len() == 0 {
            return -1;
        }
        return index_pairs[0].0 as i32;
    }
}