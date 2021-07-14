//https://leetcode.com/problems/regular-expression-matching/

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let string = s.chars().collect::<Vec<char>>();
        let pattern = p.chars().collect::<Vec<char>>();
        let mut matrix = vec![vec![false; pattern.len() + 1]; string.len() + 1];

        matrix[0][0] = true;

        for j in 2..=pattern.len() {
            if pattern[j - 1] == '*' {
                matrix[0][j] = matrix[0][j - 2];
            }
        }

        for i in 1..=string.len() {
            for j in 1..=pattern.len() {
                if string[i - 1] == pattern[j - 1] || pattern[j - 1] == '.' {
                    matrix[i][j] = matrix[i - 1][j - 1];
                }

                if pattern[j - 1] == '*' {
                    matrix[i][j] = if pattern[j - 2] == '.' || pattern[j - 2] == string[i - 1] {
                        matrix[i - 1][j] || matrix[i][j - 2]
                    } else {
                        matrix[i][j - 2]
                    };
                }
            }
        }

        matrix[string.len()][pattern.len()]
    }
}

#[test]
fn test() {
    //assert!(Solution::is_match("".into(), "****".into()));
    //assert!(Solution::is_match("".into(), ".*".into()));
    //assert!(Solution::is_match("b".into(), "c*a*b".into()));

    //assert!(!Solution::is_match("cab".into(), "*d".into()));
    //assert!(!Solution::is_match("aa".into(), "a".into()));
    //assert!(Solution::is_match("aa".into(), "a*".into()));
    //assert!(Solution::is_match("ab".into(), ".*".into()));
    //assert!(Solution::is_match("aab".into(), "c*a*b".into()));
    assert!(Solution::is_match(
        "mississippi".into(),
        "mis*is*p*.".into()
    ));
}
