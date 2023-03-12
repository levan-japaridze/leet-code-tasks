// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        while lists.len() > 1 {
            let mut new_lists = vec![];
            for i in (0..lists.len()).step_by(2) {
                if i + 1 < lists.len() {
                    let merged = Self::merge_two_lists(lists[i].take(), lists[i+1].take());
                    new_lists.push(merged);
                } else {
                    new_lists.push(lists[i].take());
                }
            }
            lists = new_lists;
        }
        lists.pop().unwrap_or(None)
    }
    
    fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        while l1.is_some() && l2.is_some() {
            let (val1, val2) = (l1.as_ref().unwrap().val, l2.as_ref().unwrap().val);
            if val1 <= val2 {
                let node = l1.take().unwrap();
                tail.next = Some(node);
                tail = tail.next.as_deref_mut().unwrap();
                l1 = tail.next.take();
            } else {
                let node = l2.take().unwrap();
                tail.next = Some(node);
                tail = tail.next.as_deref_mut().unwrap();
                l2 = tail.next.take();
            }
        }
        if l1.is_some() {
            tail.next = l1;
        } else if l2.is_some() {
            tail.next = l2;
        }
        dummy.next.take()
    }
}
