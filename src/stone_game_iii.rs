// https://leetcode.com/problems/stone-game-iii/submissions/

// One of way of thinking is to go with minimax algo and try to find the root value with it by
// summing up all the values
// Then we could subtract that value from the sum of the array to find the score of other player.
// Then we can compare to provide the solution
//
// The other of approaching this problem is by thinking the best solution of the 3 options in the
// last step and comparing it with the possible previous 3 options. In short we try to the
// difference of the future step with past step and see which provides the maximum result. As, both
// the players would have tried to reduce the overall value of both of their opponent to win or
// tie. This the approach followed here.

struct Solution;

impl Solution {
    fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let vals = &mut [0; 4];

        for i in (0..stone_value.len()).rev() {
            vals[3] = vals[2];
            vals[2] = vals[1];
            vals[1] = vals[0];

            let took1 = stone_value.get(i).unwrap();
            let took2 = took1 + stone_value.get(i + 1).unwrap_or(&0);
            let took3 = took2 + stone_value.get(i + 2).unwrap_or(&0);

            vals[0] = std::cmp::max(
                took1 - vals[1],
                std::cmp::max(took2 - vals[2], took3 - vals[3]),
            );
        }

        match vals[0].cmp(&0) {
            std::cmp::Ordering::Equal => "Tie".into(),
            std::cmp::Ordering::Greater => "Alice".into(),
            std::cmp::Ordering::Less => "Bob".into(),
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 7]), "Bob".to_owned());
    assert_eq!(
        Solution::stone_game_iii(vec![1, 2, 3, -9]),
        "Alice".to_owned()
    );
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 6]), "Tie".to_owned());
    assert_eq!(
        Solution::stone_game_iii(vec![1, 2, 3, -1, -2, -3, 7]),
        "Alice".to_owned()
    );
}
