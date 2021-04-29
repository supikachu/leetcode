#[allow(dead_code)]
pub struct Solution;

use crate::common::linked_list::ListNode;
#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy_head = Some(Box::new(ListNode::new(101)));
        let mut tail = &mut dummy_head;
        while head.is_some() {
            if tail.as_ref().unwrap().val != head.as_ref().unwrap().val {
                let temp = head.as_mut().unwrap().next.take();
                tail.as_mut().unwrap().next = head.take();
                tail = &mut tail.as_mut().unwrap().next;
                head = temp;
            } else {
                head = head.as_mut().unwrap().next.take();
            }
        }
        dummy_head.unwrap().next
    }
}
