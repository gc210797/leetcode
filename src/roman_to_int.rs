//https://leetcode.com/problems/roman-to-integer/

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let roman_to_num = |c: char| -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("Invalid roman integer"),
            }
        };

        let mut prev = 0;
        for c in s.chars() {
            let curr = roman_to_num(c);
            if curr <= prev {
                ans += curr;
            } else {
                ans += curr - 2 * prev;
            }

            prev = curr;
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::roman_to_int("III".into()), 3);
    assert_eq!(Solution::roman_to_int("IV".into()), 4);
    assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
}
