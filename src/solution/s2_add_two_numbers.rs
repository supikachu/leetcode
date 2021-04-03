#[allow(dead_code)]
struct Solution {}

use crate::common::linked_list::ListNode;
#[allow(dead_code)]
#[allow(unused_variables)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l, mut r) = (l1, l2);
        let (mut lhs, mut rhs, mut sum);
        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        let (mut left_flag, mut right_flag, mut carry) = (false, false, false);
        loop {
            lhs = if let Some(node) = l {
                l = node.next;
                node.val
            } else {
                left_flag = true;
                0
            };
            rhs = if let Some(node) = r {
                r = node.next;
                node.val
            } else {
                right_flag = true;
                0
            };
            if left_flag && right_flag && !carry {
                return head.unwrap().next;
            }
            sum = lhs + rhs + if carry { 1 } else { 0 };
            sum = if sum >= 10 {
                carry = true;
                sum - 10
            } else {
                carry = false;
                sum
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
}

#[cfg(test)]
mod tests {}
