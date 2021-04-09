use std::cmp::Ordering;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn push(&mut self, val: i32) {
        let _node = Box::new(ListNode {
            val,
            next: self.next.take(),
        });
        self.next = Some(_node);
        self.val += 1;
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.next.take().map(|node| {
            self.next = node.next;
            self.val -= 1;
            node.val
        })
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut link = ListNode::new(0);
        link.push(1);
        link.push(2);
        assert_eq!(Some(2), link.pop());
        assert_eq!(Some(1), link.pop());
    }
}
