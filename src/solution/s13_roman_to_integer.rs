#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let _ = s
            .chars()
            .map(|ch| match ch {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                _ => 1000,
            })
            .fold(0, |previous, current| {
                if previous < current {
                    ans += current - 2 * previous;
                } else {
                    ans += current;
                }
                current
            });
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sol_s13() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }
}
