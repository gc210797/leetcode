//https://leetcode.com/problems/palindrome-number/
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut tmp = x;
        let mut res = 0;

        while tmp > 0 {
            res *= 10;

            res += tmp % 10;

            tmp /= 10;
        }

        x == res
    }
}

#[test]
fn test() {
    println!("{}", std::i32::MAX);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(std::i32::MAX), false);
}
