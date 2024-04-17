// https://leetcode.com/problems/add-two-numbers/


// Definition for singly-linked list.
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

pub fn iterate_over_the_lists_and_build_new(list_1: Option<Box<ListNode>>, list_2: Option<Box<ListNode>>, final_list: Box<ListNode>, carry_over:i32) ->Option<Box<ListNode>>{
    match (list_1,list_2) {
        (Some(node_1),Some(node_2))=>{
            let sum = node_1.val+node_2.val + carry_over; //1..9 + 1..9
            if sum > 9{
                let last_digit = sum & 0xF;
                *final_list.next = Some(
                    Box::new(ListNode{
                        val: last_digit,
                        next: None,
                    })

                );
                iterate_over_the_lists_and_build_new(node_1.next,node_2.next,final_list.next.unwrap(),1)
            }else {
                *final_list.unwrap().next = Some(
                    Box::new(ListNode{
                        val: sum,
                        next: None,
                    })

                );
                iterate_over_the_lists_and_build_new(node_1.next,node_2.next,final_list.next.unwrap(),0)
            }
        },
        (None,Some(node_2))=>{
            let sum = node_2.val + carry_over; //0..9 + 0..9
            if sum > 9{
                let last_digit = sum & 0xF;
                *final_list.unwrap().next = Some(
                    Box::new(ListNode{
                        val: last_digit,
                        next: None,
                    })

                );
                iterate_over_the_lists_and_build_new(None,node_2.next,final_list.next.unwrap(),1)
            }else {
                *final_list.unwrap().next = Some(
                    Box::new(ListNode{
                        val: sum,
                        next: None,
                    })

                );
                iterate_over_the_lists_and_build_new(None,node_2.next,final_list.next.unwrap(),0)
            }

        },
        (Some(node_1),None)=>{
            let sum = node_1.val+ carry_over; //1..9 + 1..9
            if sum > 9{
                let last_digit = sum & 0xF;
                *final_list.unwrap().next = Some(
                    Box::new(ListNode{
                        val: last_digit,
                        next: None,
                    })

                );
                iterate_over_the_lists_and_build_new(node_1.next,None,final_list.next.unwrap(),1)
            }else {
                *final_list.unwrap().next = Some(
                    Box::new(ListNode{
                        val: sum,
                        next: None,
                    })

                );
                iterate_over_the_lists_and_build_new(node_1.next,None,final_list.next.unwrap(),0)
            }

        },
        (None,None)=>{
            Some(final_list)

        }
    }
}


impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let final_list_init = Box::new(ListNode{ val: 0, next: None });
        iterate_over_the_lists_and_build_new(l1,l2,final_list_init,0)
    }
}