//https://leetcode.com/problems/wildcard-matching/

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let string: Vec<_> = s.chars().collect();
        let pattern: Vec<_> = p.chars().collect();
        let slen = string.len();
        let plen = pattern.len();
        
        let mut matrix = vec![vec![false; plen + 1]; slen + 1];
        
        matrix[0][0] = true;
        
        for i in 1..=plen {
            if pattern[i - 1] == '*' {
                matrix[0][i] = matrix[0][i - 1];
            }
        }
        
        for i in 1..=slen {
            for j in 1..=plen {
                if string[i - 1] == pattern[j - 1] || pattern[j - 1] == '?' {
                    matrix[i][j] = matrix[i - 1][j - 1];
                }
                
                if pattern[j - 1] == '*' {
                    matrix[i][j] = matrix[i - 1][j] || matrix[i][j - 1];
                }
            }
        }
        
        matrix[slen][plen]
    }
}
