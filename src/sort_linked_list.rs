// https://leetcode.com/submissions/detail/518882115/

use super::ListNode;

struct Solution;

impl Solution {
    fn length(head: &Option<Box<ListNode>>) -> u32 {
        let mut count = 0;
        let mut curr = head.as_ref();

        while let Some(x) = curr {
            curr = x.next.as_ref();

            count += 1;
        }

        count
    }

    fn merge_list(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut it1 = list1.take();
        let mut it2 = list2.take();
        let mut head = ListNode::new(0);
        let mut res = &mut head;

        while it1.is_some() && it2.is_some() {
            let val1 = it1.as_ref().unwrap().val;
            let val2 = it2.as_ref().unwrap().val;

            if val1 < val2 {
                let next = it1.as_mut().unwrap().next.take();
                res.next = it1.take();
                it1 = next;
            } else {
                let next = it2.as_mut().unwrap().next.take();
                res.next = it2.take();
                it2 = next;
            }

            res = res.next.as_mut().unwrap();
        }

        if it1.is_some() {
            res.next = it1.take();
        } else {
            res.next = it2.take();
        }

        head.next
    }

    fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Self::length(&head);

        if len < 2 {
            return head;
        }

        let mut ptr = head.as_mut();
        for _ in 0..(len / 2) - 1 {
            if let Some(node) = ptr {
                ptr = node.next.as_mut();
            }
        }

        Self::merge_list(
            Self::sort_list(ptr.unwrap().next.take()),
            Self::sort_list(head),
        )
    }
}
