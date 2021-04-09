#[allow(dead_code)]
struct Solution {}

use crate::common::linked_list::ListNode;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[allow(dead_code)]
#[allow(unused_variables)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut priority_queue = BinaryHeap::new();
        lists.into_iter().for_each(|node| {
            if let Some(_) = node {
                priority_queue.push(Reverse(node));
            }
        });
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        while !priority_queue.is_empty() {
            let mut node = priority_queue.pop().unwrap().0;
            match node.as_mut().unwrap().next.take() {
                nex @ Some(_) => {
                    priority_queue.push(Reverse(nex));
                    tail.as_mut().unwrap().next = node.take();
                    tail = &mut tail.as_mut().unwrap().next;
                }
                None => {
                    tail.as_mut().unwrap().next = node.take();
                    tail = &mut tail.as_mut().unwrap().next;
                }
            }
        }
        head.unwrap().next
    }
}
