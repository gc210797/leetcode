//https://leetcode.com/problems/string-to-integer-atoi/
//Really bad solution, can be improved darastically

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        if chars.is_empty() {
            return 0;
        }

        let mut ret: i32 = 0;
        let mut sign = false;
        let mut start = false;

        for i in 0..len {
            if (chars[i] == '-' || chars[i] == '+') && start {
                break;
            }

            if (chars[i] == '-' || chars[i] == '+') && i < (len - 1) && chars[i + 1].is_numeric() {
                if chars[i] == '-' {
                    sign = true;
                }
                continue;
            }

            if chars[i].is_alphabetic() || chars[i] == '.' || chars[i] == '+' || chars[i] == '-' {
                break;
            }

            if chars[i].is_whitespace() {
                if start {
                    break;
                }
                continue;
            }

            if !start {
                start = true;
            }

            match ret.checked_mul(10) {
                Some(ref num) => match num.checked_add(chars[i].to_digit(10).unwrap() as i32) {
                    Some(ref other_num) => ret = *other_num,
                    None => {
                        if sign {
                            return std::i32::MIN;
                        } else {
                            return std::i32::MAX;
                        };
                    }
                },
                None => {
                    if sign {
                        return std::i32::MIN;
                    } else {
                        return std::i32::MAX;
                    };
                }
            }
        }

        if sign {
            -ret
        } else {
            ret
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::my_atoi("-+12".into()), 12);
}
