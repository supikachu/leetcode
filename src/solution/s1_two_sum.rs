#[allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;
#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut map: HashMap<i32, usize> = HashMap::new();
        while i < nums.len() {
            if let Some(&index) = map.get(&(target - nums[i])) {
                return vec![index as i32, i as i32];
            } else {
                map.insert(nums[i], i);
            }
            i += 1;
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sol_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
