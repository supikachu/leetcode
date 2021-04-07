use std::collections::HashMap;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let map: HashMap<_, _> = [')', ']', '}'].iter().zip(['(', '[', '{'].iter()).collect();
        let mut v = Vec::new();
        for ch in s.chars() {
            match map.get(&ch) {
                Some(c) => match v.pop() {
                    Some(st) => {
                        if **c != st {
                            return false;
                        }
                    }
                    None => return false,
                },
                None => v.push(ch),
            }
        }
        v.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sol_s20() {
        assert!(!Solution::is_valid("){".to_string()));
    }
}
