//https://leetcode.com/problems/insertion-sort-list/

use super::ListNode;

struct Solution;

impl Solution {
    fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = Box::new(ListNode::new(-1));
        let mut prev = &mut dummy;

        while let Some(mut node) = head {
            let tmp = node.next.take();

            if prev.val > node.val {
                prev = &mut dummy;
            }

            while prev.next.is_some() && prev.next.as_ref().unwrap().val < node.val {
                prev = prev.next.as_mut().unwrap();
            }

            node.next = prev.next.take();
            prev.next = Some(node);

            head = tmp;
        }

        dummy.next
    }
}

#[test]
fn test() {
    let node5 = Some(Box::new(ListNode {
        val: -2,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));

    println!("{:?}", Solution::insertion_sort_list(node5));
}
