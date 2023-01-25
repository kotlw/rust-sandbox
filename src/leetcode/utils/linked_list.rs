use crate::leetcode::definition::singly_linked_list::ListNode;

pub fn from_iter<const N: usize>(arr: [i32; N]) -> Option<Box<ListNode>> {
    let mut root: Option<Box<ListNode>> = None;

    for i in arr.into_iter().rev() {
        root = Some(Box::new(ListNode { val: i, next: root }));
    }

    return root
}