use std::collections::HashMap;
/*
1. 两数之和
给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，
并返回它们的数组下标。
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
你可以按任意顺序返回答案。
*/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    if nums.len() == 0 {
        return result;
    }
    let mut value_to_index:HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for (index, value) in nums.iter().enumerate() {
        if value_to_index.contains_key(&(target - value)) {
            result.push(value_to_index[&(target - value)] as i32);
            result.push(index as i32);
            break;
        }
        value_to_index.insert(*value, index);
    }
    result
}

/*
19. 删除链表的倒数第 N 个结点
给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
*/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    head
}

/*
21. 合并两个有序链表
将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
*/
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    list1
}

/*
23. 合并K个升序链表
给你一个链表数组，每个链表都已经按升序排列。
请你将所有链表合并到一个升序链表中，返回合并后的链表。
*/
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.len() == 0 {
        return None;
    } else {
        lists[0].clone()
    }
}

/*
86. 分隔链表
给你一个链表的头节点 head 和一个特定值 x ，请你对链表进行分隔，使得所有 小于 x 的节点都出现在 大于或等于 x 的节点之前。
你应当 保留 两个分区中每个节点的初始相对位置。
*/
pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    head
}

/*
876. 链表的中间结点
给定一个头结点为 head 的非空单链表，返回链表的中间结点。
如果有两个中间结点，则返回第二个中间结点。
*/
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

}

fn main() {
    let nums = vec![3,2,4];
    let target = 6;
    let result = two_sum(nums, target);
    println!("two_sum: {:?}", result);
}
