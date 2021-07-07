// https://leetcode.com/problems/zigzag-conversion/

struct Solution;

/// This solution is based on how much we have to go down
/// and how much we have to come up. This up or down value
/// will provide the index we have to iterate through for the
/// next character in the new sequence.
/// So, now if we look at 2*x - 2*i - 3, this can be seen as per row
/// how much we should go down and when we hit the bottom, how much we
/// should come up. This can be seen as:
/// x - i - 1(go down) + x - i - 1 - 1(come up) = 2x - 2i - 3(total value)
/// Now the this will work if we have to go in down direction of zig-zag and
/// then go up or down. Now if our index with down direction ends at the
/// direction of going up then we will have to use different way as fewer
/// indices will be spaned. Up can be calculated as:
/// 2 * i - 1.
///
/// To figure the direction of the move if up or down, we can use:
/// 2 * x - i
/// if we have failed to cross the current index then we must be in the
/// up direction.

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() == 1 || num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }
        // x is number of rows
        // 2x - 2i -3 down
        // 2x - i up
        let len = s.len();
        let mut ret_str = String::from("");
        let s_chars = s.as_str();
        for i in 0..num_rows {
            let mut count: usize = i as usize;
            let mut flag = false;
            ret_str.push_str(s_chars.get(i as usize..(i + 1) as usize).unwrap());
            while count < len - 1 {
                let down = 2 * num_rows - 2 * i - 3;
                let up = 2 * i - 1;
                if flag || down < 0 {
                    count += (up + 1) as usize;
                    //if up < num_rows {
                    flag = false;
                    //}
                } else {
                    count += (down + 1) as usize;
                    if (down + 1) <= (2 * num_rows - i) && up > 0 {
                        flag = true;
                    }
                }
                if count < len {
                    ret_str.push_str(s_chars.get(count as usize..(count + 1) as usize).unwrap());
                }
            }
        }

        ret_str
    }
}
