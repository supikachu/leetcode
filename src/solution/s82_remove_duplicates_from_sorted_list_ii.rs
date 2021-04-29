#[allow(dead_code)]
pub struct Solution;

use crate::common::linked_list::ListNode;
#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy_head = Some(Box::new(ListNode::new(101)));
        let mut tail = &mut dummy_head;
        let mut pre_node = None;
        while head.is_some() {
            if pre_node.is_none() {
                match head.as_mut().unwrap().next.take() {
                    mut _next @ Some(_) => {
                        pre_node = head.take();
                        if pre_node.as_ref().unwrap().val != _next.as_ref().unwrap().val {
                            tail.as_mut().unwrap().next = pre_node.take();
                            tail = &mut tail.as_mut().unwrap().next;
                            head = _next;
                        } else {
                            head = _next.as_mut().unwrap().next.take();
                        }
                    }
                    None => {
                        tail.as_mut().unwrap().next = head.take();
                        tail = &mut tail.as_mut().unwrap().next;
                    }
                }
            } else {
                if pre_node.as_ref().unwrap().val != head.as_ref().unwrap().val {
                    pre_node.take();
                } else {
                    head = head.as_mut().unwrap().next.take();
                }
            }
        }
        dummy_head.unwrap().next
    }
}
