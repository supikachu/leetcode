#[allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;
#[allow(dead_code)]
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut map: HashMap<_, usize> = HashMap::new();
        let v: Vec<_> = s.chars().collect();
        let (mut left, mut right, mut mode) = (0, 0, 0);
        while right < v.len() {
            let count = map.entry(v[right]).or_default();
            *count += 1;
            mode = mode.max(*count);
            right += 1;
            if right - left > k as usize + mode {
                *map.get_mut(&v[left]).unwrap() -= 1;
                left += 1;
            }
        }
        (right - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sol_s424() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }
}
