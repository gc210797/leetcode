//https://leetcode.com/problems/insertion-sort-list/

use super::ListNode;

struct Solution;

impl Solution {
    fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = ListNode::new(-1);
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

    fn sorted_insert(head: Option<Box<ListNode>>, v: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        dummy.next = head;
        let mut prev = &mut dummy;

        while prev.next.is_some() && prev.next.as_ref().unwrap().val < v {
            prev = prev.next.as_mut().unwrap();
        }

        let mut new_node = Box::new(ListNode::new(v));
        new_node.next = prev.next.take();
        prev.next = Some(new_node);

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

#[test]
fn test1() {
    let node1 = None;
    let node2 = Solution::sorted_insert(node1, 5);

    println!("{:?}", node2);

    let node3 = Solution::sorted_insert(node2, 3);

    println!("{:?}", node3);

    let node4 = Solution::sorted_insert(node3, 4);

    println!("{:?}", node4);
}
