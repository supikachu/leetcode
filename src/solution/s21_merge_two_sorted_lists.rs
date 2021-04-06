#[allow(dead_code)]
struct Solution {}

use crate::common::linked_list::ListNode;
#[allow(dead_code)]
#[allow(unused_variables)]
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            l2
        } else if l2.is_none() {
            l1
        } else {
            let mut l1 = l1;
            let mut l2 = l2;
            let mut l = Some(Box::new(ListNode::new(0)));
            let mut tail = &mut l;
            loop {
                match &mut l1.take() {
                    list1 @ Some(_) => match &mut l2.take() {
                        list2 @ Some(_) => {
                            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                                l1 = list1.as_mut().unwrap().next.take();
                                tail.as_mut().unwrap().next = list1.take();
                                tail = &mut tail.as_mut().unwrap().next;
                                l2 = list2.take();
                            } else {
                                l2 = list2.as_mut().unwrap().next.take();
                                tail.as_mut().unwrap().next = list2.take();
                                tail = &mut tail.as_mut().unwrap().next;
                                l1 = list1.take();
                            }
                        }
                        None => {
                            tail.as_mut().unwrap().next = list1.take();
                            break;
                        }
                    },
                    None => {
                        tail.as_mut().unwrap().next = l2.take();
                        break;
                    }
                }
            }
            l.unwrap().next
        }
    }
}
