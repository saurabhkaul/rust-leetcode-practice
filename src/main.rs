mod problems;
use crate::problems::remove_n_from_end_link_list::ListNode;
use problems::{remove_n_from_end_link_list, reverse_integer};

fn main() {
    println!("{}", reverse_integer::run(1534236469));
    let mut new_list: Box<ListNode> = Box::new(ListNode { val: 0, next: None });
    for x in (1..20) {
        let new_node = ListNode::new(x);
        new_list.next(new_node)
    }
    println!("{:?}", remove_n_from_end_link_list::run(Some(new_list), 2))
}
