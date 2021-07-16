//https://leetcode.com/problems/trapping-rain-water-ii/

use std::cmp::max;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node(i32, usize, usize);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        if height_map.is_empty() {
            return 0;
        }

        let mut ans = 0;
        let hml = height_map.len();
        let hmlr = height_map[0].len();
        let mut visited = vec![vec![false; hmlr]; hml];
        let mut heap = BinaryHeap::new();
        let mut maximum = 0;
        let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for i in 0..hmlr {
            heap.push(Node(height_map[0][i], 0, i));
            heap.push(Node(height_map[hml - 1][i], hml - 1, i));
            visited[0][i] = true;
            visited[hml - 1][i] = true;
        }

        for i in 0..hml {
            heap.push(Node(height_map[i][0], i, 0));
            heap.push(Node(height_map[i][hmlr - 1], i, hmlr - 1));
            visited[i][0] = true;
            visited[i][hmlr - 1] = true;
        }

        while let Some(node) = heap.pop() {
            maximum = max(node.0, maximum);

            for i in 0..dirs.len() {
                let (x, y) = (node.1 as i32 + dirs[i].0, node.2 as i32 + dirs[i].1);

                if x > 0
                    && x < hml as i32 - 1
                    && y > 0
                    && y < hmlr as i32 - 1
                    && !visited[x as usize][y as usize]
                {
                    ans += max(0, maximum - height_map[x as usize][y as usize]);
                    heap.push(Node(
                        height_map[x as usize][y as usize],
                        x as usize,
                        y as usize,
                    ));
                    visited[x as usize][y as usize] = true;
                }
            }
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::trap_rain_water(vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1]
        ]),
        4
    );
}
