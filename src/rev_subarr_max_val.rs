//https://leetcode.com/problems/reverse-subarray-to-maximize-array-value/

// Secret for this solution lies in the below equation
// let S be the total sum, then:
// with some reversed subarray(in index range = [L, R]) we will get the below modified sum
// S - |A[L - 1] - A[L]| - |A[R] - A[R + 1]| + |A[L - 1] - A[R]| + |A[R + 1] - A[L]|
// So to maximize the sum we will have to maximize (-|A[L - 1] - A[L]| - |A[R] - A[R + 1]| + |A[L - 1] - A[R]| + |A[R + 1] - A[L]|)
// Now, try to find the solution if we try to reverse [0, L] and [R+1, N]
// We will also try to see if we can reverse [L, R], in that case try will all cases of with
// removing the || operator i.e check if the leftend is bigger or if rightend is bigger. This will
// provide why we need to have 2 * max_right - min_left

struct Solution;

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut diff = 0;

        for i in 0..nums.len() - 1 {
            sum += (nums[i] - nums[i + 1]).abs();
        }

        let mut left_min = std::i32::MAX;
        let mut right_max = std::i32::MIN;
        for i in 0..nums.len() - 1 {
            diff = std::cmp::max(
                diff,
                (nums[i + 1] - nums[0]).abs() - (nums[i] - nums[i + 1]).abs(),
            );
            diff = std::cmp::max(
                diff,
                (nums[i] - nums[nums.len() - 1]).abs() - (nums[i + 1] - nums[i]).abs(),
            );

            left_min = std::cmp::min(left_min, std::cmp::max(nums[i], nums[i + 1]));
            right_max = std::cmp::max(right_max, std::cmp::min(nums[i], nums[i + 1]));
        }

        diff = std::cmp::max(diff, 2 * (right_max - left_min));

        sum + diff
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_value_after_reverse(vec![2, 3, 1, 5, 4]), 10);
    assert_eq!(
        Solution::max_value_after_reverse(vec![2, 4, 9, 24, 2, 1, 10]),
        68
    );
}
