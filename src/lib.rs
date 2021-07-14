mod atoi;
mod insertion_sort_list;
mod most_water_container;
mod palindrome_number;
mod reg_exp_matching;
mod reverse_linked_list;
mod sort_linked_list;
mod trapping_rainwater;
mod wildcard_matching;
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
