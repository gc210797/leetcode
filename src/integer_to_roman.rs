//https://leetcode.com/problems/integer-to-roman/

struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut ans = "".to_owned();

        //let thousands = vec!["", "M", "MM", "MMM"];
        //let hundreds = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        //let tens = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        //let ones = vec![
        //    "", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X",
        //];

        //ans.push_str(thousands[(num / 1000) as usize]);
        //ans.push_str(hundreds[((num % 1000) / 100) as usize]);
        //ans.push_str(tens[((num % 100) / 10) as usize]);
        //ans.push_str(ones[(num % 10) as usize]);

        let mut romans: &[char] = &['\0', '\0', 'M', 'D', 'C', 'L', 'X', 'V', 'I'];
        let mut div = 1000;

        while num > 0 {
            match num / div {
                n @ 0..=3 => {
                    for _ in 0..n {
                        ans.push(romans[2]);
                    }
                }
                4 => {
                    ans.push(romans[2]);
                    ans.push(romans[1]);
                }
                n @ 5..=8 => {
                    ans.push(romans[1]);
                    for _ in 0..(n % 5) {
                        ans.push(romans[2]);
                    }
                }
                9 => {
                    ans.push(romans[2]);
                    ans.push(romans[0]);
                }
                _ => {}
            }

            num %= div;
            div /= 10;
            romans = &romans[2..];
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::int_to_roman(3), "III".to_owned());
    assert_eq!(Solution::int_to_roman(4), "IV".to_owned());
    assert_eq!(Solution::int_to_roman(9), "IX".to_owned());
    assert_eq!(Solution::int_to_roman(58), "LVIII".to_owned());
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_owned());
}
