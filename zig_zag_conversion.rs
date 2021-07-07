struct solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() == 1 || num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }
        // 2x - 2i -3 down
        // 2i - 1 up
        let len = s.len();
        let mut ret_str = String::from("");
        let mut s_chars = s.as_str();
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
                if(count < len) {
                    //println!("count = {}, chars = {:?}", count, s_chars);
                    ret_str.push_str(s_chars.get(count as usize .. (count + 1) as usize).unwrap());
                    //println!("down = {}, up ={}, count = {}", down, up, count);
                }
            }
            //println!();
        }
        
        ret_str
    }
}
