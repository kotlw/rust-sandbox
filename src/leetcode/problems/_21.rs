use crate::leetcode::definition::singly_linked_list::ListNode;
#[allow(unused_imports)]
use crate::leetcode::utils::linked_list;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = list1.clone();
        let mut l2 = list2.clone();

        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;

        while l1.is_some() || l2.is_some() {
            match (l1.clone(), l2.clone()) {
                (Some(n), None) => {
                    current.next = Some(Box::new(ListNode::new(n.val)));
                    l1 = n.next;
                }
                (None, Some(n)) => {
                    current.next = Some(Box::new(ListNode::new(n.val)));
                    l2 = n.next;
                }
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        current.next = Some(Box::new(ListNode::new(n1.val)));
                        l1 = n1.next;
                    } else {
                        current.next = Some(Box::new(ListNode::new(n2.val)));
                        l2 = n2.next;
                    }
                },
                _ => break,
            }

            current = current.next.as_mut().unwrap();
        }

        return dummy.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        expected_result: Option<Box<ListNode>>,
    ) {
        let res = Solution::merge_two_lists(l1, l2);
        assert_eq!(res, expected_result);
    }

    #[test]
    fn case_1() {
        let l1 = linked_list::from_iter([1, 2, 4]);
        let l2 = linked_list::from_iter([1, 3, 4]);
        let res = linked_list::from_iter([1, 1, 2, 3, 4, 4]);
        run(l1, l2, res);
    }

    #[test]
    fn case_2() {
        let l1 = linked_list::from_iter([]);
        let l2 = linked_list::from_iter([]);
        let res = linked_list::from_iter([]);
        run(l1, l2, res);
    }

    #[test]
    fn case_3() {
        let l1 = linked_list::from_iter([]);
        let l2 = linked_list::from_iter([0]);
        let res = linked_list::from_iter([0]);
        run(l1, l2, res);
    }
}
