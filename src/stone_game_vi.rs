use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut bob = 0;
        let mut alice = 0;

        let mut composite_values: Vec<(i32, usize)> = alice_values
            .iter()
            .zip(bob_values.iter())
            .enumerate()
            .map(|x| (x.1 .0 + x.1 .1, x.0))
            .collect();
        composite_values.sort_by(|a, b| b.0.cmp(&a.0));
        let mut turn = false;

        for x in composite_values {
            if !turn {
                alice += alice_values[x.1];
            } else {
                bob += bob_values[x.1];
            }

            turn = !turn;
        }

        match alice.cmp(&bob) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::stone_game_vi(vec![1, 3], vec![2, 1]), 1);
    assert_eq!(Solution::stone_game_vi(vec![1, 2], vec![3, 1]), 0);
    assert_eq!(Solution::stone_game_vi(vec![2, 4, 3], vec![1, 6, 7]), -1);
    assert_eq!(Solution::stone_game_vi(vec![17, 34], vec![34, 17]), 0);
}
