// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// impl ListNode {
//     pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         if let Some(ref mut node) = head {
//             let tmp_node = node;
//             node.next = Some(*node);
//             node = tmp_node;
//             return Some(Box::new(*node));
//         } else {
//             return None;
//         }
//     }
// }
