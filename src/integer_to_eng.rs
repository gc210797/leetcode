//https://leetcode.com/problems/integer-to-english-words/

struct Solution;

const WORDS: &[&str] = &[
    "Zero ",
    "One ",
    "Two ",
    "Three ",
    "Four ",
    "Five ",
    "Six ",
    "Seven ",
    "Eight ",
    "Nine ",
    "Ten ",
    "Eleven ",
    "Twelve ",
    "Thirteen ",
    "Fourteen ",
    "Fifteen ",
    "Sixteen ",
    "Seventeen ",
    "Eighteen ",
    "Nineteen ",
    "Twenty ",
    "Thirty ",
    "Forty ",
    "Fifty ",
    "Sixty ",
    "Seventy ",
    "Eighty ",
    "Ninety ",
    "Hundred ",
    "Thousand ",
    "Million ",
    "Billion ",
];
impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        Solution::solution(num, 1_000_000_000)
    }

    pub fn suffix(div: i32) -> &'static str {
        match div {
            1_000_000_000 => WORDS[31],
            1_000_000 => WORDS[30],
            1_000 => WORDS[29],
            100 => WORDS[28],
            20 => WORDS[20],
            30 => WORDS[21],
            40 => WORDS[22],
            50 => WORDS[23],
            60 => WORDS[24],
            70 => WORDS[25],
            80 => WORDS[26],
            90 => WORDS[27],
            _ => {
                println!("{}", div);
                panic!("Invalid");
            }
        }
    }

    fn solution(mut num: i32, mut div: i32) -> String {
        let mut ans = "".to_owned();
        if num == 0 {
            ans.push_str(WORDS[0]);
        }

        while num > 0 {
            let n = if div > 10 || num > 19 {
                num / div
            } else {
                div = 1;
                num
            };
            if n > 0 {
                match div {
                    10 => ans.push_str(Solution::suffix(n * 10)),
                    1 => ans.push_str(WORDS[n as usize]),
                    _ => {
                        ans.push_str(Solution::solution(n, div).as_str());
                        ans.push(' ');
                        ans.push_str(Solution::suffix(div));
                    }
                }
            }
            num %= div;
            if div > 1000 {
                div /= 1000;
            } else {
                div /= 10;
            }
        }

        ans.pop();
        ans
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::number_to_words(123),
        "One Hundred Twenty Three".to_owned()
    );

    assert_eq!(
        Solution::number_to_words(12345),
        "Twelve Thousand Three Hundred Forty Five".to_owned()
    );
    assert_eq!(
        Solution::number_to_words(1234567),
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_owned()
    );
    assert_eq!(
        Solution::number_to_words(1234567891),
        "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One".to_owned()
    );
    assert_eq!(Solution::number_to_words(0), "Zero".to_owned());
    assert_eq!(Solution::number_to_words(12), "Twelve".to_owned());
}
