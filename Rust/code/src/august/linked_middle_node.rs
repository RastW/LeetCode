// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

// 返回链表中间节点
// 使用亏快慢指针，快指针走两步慢指针走一步，最终快指针走完的时候满指针走到中间节点
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut slow, mut fast) = (&head, &head);
    while fast.as_ref().is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &(fast.as_ref().unwrap().next.as_ref().unwrap().next);
    }
    slow.clone()
}