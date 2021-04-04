#[allow(dead_code)]
struct Solution {}

use crate::common::linked_list::ListNode;
#[allow(dead_code)]
#[allow(unused_variables)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut rev_head = Some(Box::new(ListNode::new(0)));
        let tail = &mut rev_head;
        while let node @ Some(_) = &mut head.take() {
            head = node.as_mut().unwrap().next.take();
            node.as_mut().unwrap().next = tail.as_mut().unwrap().next.take();
            tail.as_mut().unwrap().next = node.take();
        }
        rev_head.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sol_s206() {
        println!("{:?}", Solution::reverse_list(None));
        assert_eq!(1, 1);
    }
}
