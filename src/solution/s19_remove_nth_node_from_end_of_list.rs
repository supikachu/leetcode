#[allow(dead_code)]
struct Solution {}

use crate::common::linked_list::ListNode;
#[allow(dead_code)]
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let target = Solution::len(&head) - n;
        let mut list = head;
        let mut head;
        let mut dummy_node = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_node;
        let mut count = 0;
        while count != target {
            head = list.as_mut().unwrap().next.take();
            tail.as_mut().unwrap().next = list.take();
            list = head.take();
            tail = &mut tail.as_mut().unwrap().next;
            count += 1;
        }
        tail.as_mut().unwrap().next = list.as_mut().unwrap().next.take();
        dummy_node.unwrap().next
    }
    fn len(l: &Option<Box<ListNode>>) -> i32 {
        let mut len = 0;
        let mut list = l;
        while list.is_some() {
            len += 1;
            list = &list.as_ref().unwrap().next;
        }
        len
    }
}
