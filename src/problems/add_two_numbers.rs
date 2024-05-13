// https://leetcode.com/problems/add-two-numbers/

use crate::problems::Solution;

// Definition for singly-linked list.
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn iterate_over_the_lists_and_build_new(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(l1), Some(l2)) => {
            let sum = l1.val + l2.val + carry;
            // println!(
            //     "l1.val {0},l2.val {1}, carry {carry:#?}, sum {sum:#?}",
            //     l1.val, l2.val
            // );
            if sum > 9 {
                let last_digit = sum % 10;
                Some(Box::new(ListNode {
                    val: last_digit,
                    next: iterate_over_the_lists_and_build_new(l1.next, l2.next, 1),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: sum,
                    next: iterate_over_the_lists_and_build_new(l1.next, l2.next, 0),
                }))
            }
        }
        (Some(l1), None) => {
            let sum = l1.val + carry;
            // println!("l1.val {0},None, carry {carry:#?}, sum {sum:#?}", l1.val);
            if sum > 9 {
                let last_digit = sum % 10;
                Some(Box::new(ListNode {
                    val: last_digit,
                    next: iterate_over_the_lists_and_build_new(l1.next, None, 1),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: sum,
                    next: iterate_over_the_lists_and_build_new(l1.next, None, 0),
                }))
            }
        }
        (None, Some(l2)) => {
            let sum = l2.val + carry;
            // println!("None, l2.val {0}, carry {carry:#?}, sum {sum:#?}", l2.val);
            if sum > 9 {
                let last_digit = sum % 10;
                Some(Box::new(ListNode {
                    val: last_digit,
                    next: iterate_over_the_lists_and_build_new(None, l2.next, 1),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: sum,
                    next: iterate_over_the_lists_and_build_new(None, l2.next, 0),
                }))
            }
        }
        //both lists have finished, but we could have a carry
        (None, None) => {
            if carry > 0 {
                Some(Box::new(ListNode {
                    val: carry,
                    next: None,
                }))
            } else {
                None
            }
        }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        iterate_over_the_lists_and_build_new(l1, l2, 0)
    }
}
