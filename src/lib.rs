mod atoi;
mod insertion_sort_list;
mod integer_to_eng;
mod integer_to_roman;
mod most_water_container;
mod palindrome_number;
mod reg_exp_matching;
mod reverse_linked_list;
mod roman_to_int;
mod sort_linked_list;
mod trapping_rainwater;
mod trapping_rainwater_2d;
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
