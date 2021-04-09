#[allow(dead_code)]
struct Solution {}

// [2,2,2,2]
use std::collections::HashMap;
#[allow(dead_code)]
#[allow(unused_variables)]
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut nums = 0;
        let mut map = HashMap::new();
        for ans in &answers {
            let count = map.entry(ans).or_insert(0);
            *count += 1;
        }
        for (kind, num) in map {
            let k = kind + 1;
            if kind + 1 < num {
                nums = nums + (((num as f32) / (k as f32)).ceil() as i32) * k;
            } else {
                nums = nums + k;
            }
        }
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sol_s781() {
        assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
        assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
    }
}
