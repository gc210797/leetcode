struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut first = 0;
        let mut last = height.len() - 1;
        let mut area = 0;
        while first < last {
            let tmp = (last - first) as i32 * std::cmp::min(height[first], height[last]);
            
            if tmp > area {
                area = tmp;
            }
            
            if height[first] > height[last] {
                last -=1 ;
            } else {
                first += 1;
            }
        }
        
        area as i32
    }
}
