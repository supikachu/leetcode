#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = Vec::new();
        for i in 0..(text1.len() + 1) {
            let mut v = Vec::new();
            for j in 0..(text2.len() + 1) {
                v.push(0);
            }
            dp.push(v);
        }
        for i in 1..(text1.len() + 1) {
            for j in 1..(text2.len() + 1) {
                if text1[i - 1..i] == text2[j - 1..j] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
                }
            }
        }
        dp[text1.len()][text2.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sol_s1143() {
        assert_eq!(
            Solution::longest_common_subsequence(String::from("abcde"), String::from("ace")),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence(String::from("abc"), String::from("abc")),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence(String::from("abc"), String::from("def")),
            0
        );
    }
}
