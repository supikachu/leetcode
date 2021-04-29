#[allow(dead_code)]
pub struct Solution;

use crate::common::linked_list::ListNode;
#[allow(dead_code)]
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;
        while head.is_some() {
            match head.as_mut().unwrap().next.take() {
                mut next @ Some(_) => {
                    let temp = head.take();
                    head = next.as_mut().unwrap().next.take();
                    tail.as_mut().unwrap().next = next.take();
                    tail = &mut tail.as_mut().unwrap().next;
                    tail.as_mut().unwrap().next = temp;
                    tail = &mut tail.as_mut().unwrap().next;
                }
                None => {
                    tail.as_mut().unwrap().next = head.take();
                }
            }
        }
        dummy_head.unwrap().next
    }
}
