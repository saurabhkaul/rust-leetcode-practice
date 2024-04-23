//https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}




fn remove_nth_from_end_recur(left: &Option<Box<ListNode>>, right: &Option<Box<ListNode>>) ->Option<Box<ListNode>>{
    match (left,right) {
        (Some(left), Some(right)) => {
            Some(Box::new(ListNode{
                val: left.val,
                next: remove_nth_from_end_recur(&left.next,&right.next),
            }))
        },
        (Some(l),None)=>{
            l.next.clone()
        }
        _ => unreachable!()
    }
}



impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        //create our right pointer
        let left = &head;
        let mut right = &head;
        for _ in 0..n{
            if let Some(node) = right{
                right = &node.next;
            }
        }
        remove_nth_from_end_recur(left,right)
    }
}