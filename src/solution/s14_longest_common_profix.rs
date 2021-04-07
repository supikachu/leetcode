use std::ops::Deref;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_profix(strs: Vec<String>) -> String {
        if strs.contains(&"".to_string()) || strs.len() == 0 {
            String::from("")
        } else {
            String::from(strs.iter().fold(strs[0].deref(), |profix, s| {
                Solution::common_profix(profix, s).unwrap_or("")
            }))
        }
    }

    fn common_profix<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
        if s1.len() + s2.len() <= 1 {
            return None;
        }
        let mut i = 1;
        while i <= s2.len() && s1.starts_with(&s2[..i]) {
            i += 1;
        }
        if i == 1 {
            None
        } else {
            i -= 1;
            Some(&s2[..i])
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sol_s14() {
        // assert_eq!(Solution::common_profix("abc", "abd"), Some("a"));
        assert_eq!(
            Solution::longest_common_profix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
    }
}
