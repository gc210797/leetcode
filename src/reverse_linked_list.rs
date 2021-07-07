// https://leetcode.com/problems/reverse-linked-list/
// https://leetcode.com/problems/reverse-linked-list-ii/

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

struct Solution;

impl Solution {
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head.take();
        let mut prev = None;
        while let Some(mut x) = curr {
            let next = x.next.take();
            x.next = prev.take();
            prev = Some(x);
            curr = next;
        }
        prev
    }

    fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut curr = head.as_mut();
        let mut i = 1;

        if left == 1 {
            return Self::reverse_count(head, right - left + 1);
        }

        while let Some(x) = curr {
            i += 1;

            if i == left {
                x.next = Self::reverse_count(x.next.take(), right - left + 1);
                break;
            } else {
                curr = x.next.as_mut();
            }
        }

        head
    }

    fn reverse_count(mut head: Option<Box<ListNode>>, count: i32) -> Option<Box<ListNode>> {
        let mut curr = head.take();
        let mut prev: Option<Box<ListNode>> = None;
        let mut i = 0;

        while let Some(mut x) = curr {
            if i == count {
                let mut last_append = prev.as_mut();
                while let Some(y) = last_append {
                    if y.next.is_none() {
                        y.next = Some(x);
                        break;
                    }
                    last_append = y.next.as_mut();
                }
                break;
            }
            i += 1;
            let next = x.next.take();
            x.next = prev.take();
            prev = Some(x);
            curr = next;
        }

        prev
    }
}

#[test]
fn list_test() {
    let node5 = Some(Box::new(ListNode {
        val: 1,
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

    println!("{:?}", Solution::reverse_between(node5, 2, 4));
}
