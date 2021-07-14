//https://leetcode.com/problems/trapping-rain-water/


impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut result = 0;
        
        let mut low = 0;
        let mut high = height.len() - 1;
        let mut right_max = 0;
        let mut left_max = 0;
        
        while low < high {
            if height[low] < height[high] {
                if height[low] > left_max {
                    left_max = height[low];
                } else {
                    result += left_max - height[low]
                }
                low += 1;
            } else {
                if height[high] > right_max {
                    right_max = height[high];
                } else {
                    result += right_max - height[high];
                }
                high -= 1;
            }
        }
        result
    }
}
