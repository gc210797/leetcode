mod atoi;
mod insertion_sort_list;
mod palindrome_number;
mod reg_exp_matching;
mod reverse_linked_list;
mod sort_linked_list;
mod zig_zag_conversion;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}
