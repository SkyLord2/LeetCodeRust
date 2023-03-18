//! ```
//! // 1. mut, &, &mut的区别
//! let a = "xxx".to_string();　　
//! // 含义：a绑定到字符串资源A上，拥有资源A的所有权

//! let mut a = "xxx".to_string();　
//! // 含义：a绑定到字符串资源A上，拥有资源A的所有权，同时a还可绑定到新的资源上面去（更新绑定的能力，但新旧资源类型要同）；

//! // value
//! let b = a;
//! // 含义：a绑定的资源A转移给b，b拥有这个资源A

//! let b = &a;　　
//! // 含义：a绑定的资源A借给b使用，b只有资源A的读权限

//! let b = &mut a;　　
//! // 含义：a绑定的资源A借给b使用，b有资源A的读写权限

//! let mut b = &mut a;　　
//! // 含义：a绑定的资源A借给b使用，b有资源A的读写权限。同时，b可绑定到新的资源上面去（更新绑定的能力）
//! 
//! // 2. unwrap()方法：
//! // 在确认Option不为None的情况下，可以用unwrap方法拆解出其中的值，并获取值的所有权，
//! // 如果为None，就会引发panic。这里要强调的是，unwrap会消费Option本身的值，后面就不能再用了。
//! let a = Some(Box::new(5));
//! let d = a.unwrap();
//! // println!("{:?}", a); // cannot use 'a' again.
//! println!("{:?}", d);
//! // 但这里有一个所有权的限制，因为涉及到其内部值的所有权的转移，所以只有Option原始变量本身可以调用unwrap方法，
//! // 其引用（包括可变引用）均不可调用。这和unwrap的实现方法有关系，因为其消费了原始变量。下方代码不可编译：
//! let mut a = Some(Box::new(5));
//! let b = &mut a;
//! let c = b.unwrap(); // Error! cannot move out of `*b` which is behind a mutable reference
//! // 3. take()方法：
//! // take方法可以从Option中取出值，为原来的Option变量留下None值，但原来的变量还有效（但实际上没什么用处了，因为只剩下None了）。
//! // take方法的原始值或者引用必须为mut类型。强调一下，借用情况下可以调用take方法。或者说指向Option的可变引用（指针）可以调用take方法。
//! fn main() {
//!     let mut b = &mut a;
//!     let c = &mut b;
//!     let d = c.take();
//!     println!("{:?}", c);//None
//!     println!("{:?}", d);//Some(5)
//!     println!("{:?}", a);//None
//! }
//! // 4.as_ref()方法：
//! // 但上面这两个方法，都改变了Option变量本身。如果不改变或受到限制无法改变Option本身时，
//! // 只想借用或可变借用其中unwrap的值应该怎么办呢？这也是在实现链表时遇到的让人掉坑的问题。幸好Option提供了 as_ref 和 as_mut方法。
//! fn as_ref(&self) -> Option<&T>
//! // 接受一个不可变引用，不获取所有权，返回一个Option枚举，其中的T为不可变引用类型，不获取所有权。
//! // 该方法将对Option的引用变为对Option所包含对象的不可变引用，并且返回一个新的Option。对这个新的Option进行unwrap操作，
//! // 可以获得原Option所包含的对象的不可变引用。原始Option变量及其引用，均可调用as_ref方法，有点克隆的味道。
//! let a = Some(Box::new(5));
//! let b = a.as_ref();
//! let c = b.unwrap();
//! println!("{:?}", a);//Some(5)
//! println!("{:?}", b);//Some(5)
//! println!("{:?}", c);//5
//! // 5. as_mut()方法
//! // 与as_ref类似，as_mut只是返回了Option包含的数据的可变引用，
//! fn main() {
//!     let mut a = Some(Point { x: 5, y: 5 });
//!     // let b = a.as_mut();
//!     let b = &mut a;
//!     let c = b.as_mut(); // c is a new Option;
//!     let d = c.unwrap();
//!     d.x += 10;
//!     let e = &mut d.y;
//!     *e += 20;
//!     println!("{:?}", d.x);//15
//!     // println!("{:?}", c); // c is not available because of method call of "unwrap".
//!     println!("{:?}", b);//Some(Point { x: 15, y: 25 })
//!     println!("{:?}", a);//Some(Point { x: 15, y: 25 })
//! }
//! // 通过以上代码可以看出，在未改变Option变量a的情况下，通过as_mut方法，改变了其包含的数据的值。
//! // 这个能力对于编写链表时，尤其是节点的插入、删除时，可以灵活的操作指向下一个节点的指针。
//! // 说明一点，调用as_mut方法时，Option变量本身或其引用，必须为可变的，即mut类型
//! fn main() {
//!     let mut a = Some(5);
//!     let b = a.as_mut().unwrap();
//!     *b += 10;
//!     println!("b = {:?}", b);//b = 15
//!     println!("a = {:?}", a);//a=Some(15)
//! }
//! 
//! // 6. if let和while let
//! if let Some(a_T) = a_option { 
//!     //a_T生命周期仅在花括号内生效。
//!     //当a_option不为None时满足条件。
//! }
//! 
//! while let Some(a_T) = a_option { 
//!     //当a_option 为None时退出循环。
//! }
//! // ？操作符的作用就是简化Result枚举的
//!     let f = File::open("username.txt"); 
//!     let mut f = match f {
//!     Ok(file) => file,
//!     Err(e) => return Err(e),
//! };
//! //有了？运算符，就可以改成下面一行话
//! let mut f = File::open("username.txt")?;
//! 
//! ```

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::{ Ord, Ordering, PartialOrd };
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

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
/// 采用双指针算法，快指针需要前移n+1(添加虚拟头节点)个节点，慢指针开始移动
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // 增加一个虚拟头节点，以防要删除的是第一个节点
    let mut dummy = Some(Box::new(ListNode{val: 0, next: head}));
    let mut slow_p = &mut dummy;
    let mut fast_p = &slow_p.clone();
    // 增加了一个虚拟头节点，故需要前移 n + 1 个节点
    for _ in 0..=n {
        if let Some(fast_node) = fast_p {
            fast_p = &fast_node.next;
        } else {
            return None;
        }
    }

    while fast_p.is_some() {
        fast_p = &fast_p.as_ref().unwrap().next;
        slow_p = &mut slow_p.as_mut().unwrap().next;
    }
    // 要删除倒数第 n 个节点，就得获得倒数第 n + 1 个节点的引用
    let remove_p = &mut slow_p.as_mut().unwrap().next;
    slow_p.as_mut().unwrap().next = remove_p.as_mut().unwrap().next.take();
    dummy.unwrap().next
}

/*
21. 合并两个有序链表
将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
*/
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;
    // 虚拟头节点
    let mut ret = ListNode::new(0);
    // 可变引用
    let mut ret_r = &mut ret;

    while let (Some(node1), Some(node2)) = (list1.as_mut(), list2.as_mut()) {
        if node1.val <= node2.val {
            let list1_tail = node1.next.take();
            ret_r.next = list1.take();
            list1 = list1_tail;
        } else {
            let list2_tail = node2.next.take();
            ret_r.next = list2.take();
            list2 = list2_tail;
        }
        ret_r = ret_r.next.as_mut().unwrap();
    }

    ret_r.next = list1.or(list2);

    ret.next
}

pub fn merge_two_lists_iter (list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (None, l2) => l2,
        (l1, None) => l1,
        (Some(mut l1), Some(mut l2)) => {
            if l1.val <= l2.val {
                l1.next = self::merge_two_lists_iter(l1.next, Some(l2));
                Some(l1)
            } else {
                 l2.next = self::merge_two_lists_iter(Some(l1), l2.next);
                 Some(l2)
            }
        }
    }
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
        // 小顶堆
        let mut priority_queue = BinaryHeap::new();
        for list in lists {
            if let Some(node) = list {
                priority_queue.push(node);
            }
        }
        // dummy
        let mut ret = Box::new(ListNode::new(0));
        let mut ret_ref = &mut ret;
        while let Some(mut node) = priority_queue.pop() {
            if let Some(n) = node.next.take() {
                priority_queue.push(n);
            }
            ret_ref.next = Some(node);
            ret_ref = ret_ref.next.as_mut().unwrap();
        }

        ret.next
    }
}

/*
86. 分隔链表
给你一个链表的头节点 head 和一个特定值 x ，请你对链表进行分隔，使得所有 小于 x 的节点都出现在 大于或等于 x 的节点之前。
你应当 保留 两个分区中每个节点的初始相对位置。
*/
pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    // 将一个链表分割成为两个链表，一个大于等x, 一个小于 x
    let mut dummy1 = Box::new(ListNode::new(-1));
    let mut dummy2 = Box::new(ListNode::new(-1));
    let mut p1 = &mut dummy1;
    let mut p2 = &mut dummy2;
    let mut p = head;

    while let Some(node) = p.as_ref() {
        if node.val < x {
            p1.next = p;
            p1 = p1.next.as_mut().unwrap();
            p = p1.next.take();
        } else {
            p2.next = p;
            p2 = p2.next.as_mut().unwrap();
            p = p2.next.take();
        }
    }
    p1.next = dummy2.next;
    dummy1.next
}

/*
876. 链表的中间结点
给定一个头结点为 head 的非空单链表，返回链表的中间结点。
如果有两个中间结点，则返回第二个中间结点。
*/
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head
}

fn main() {
    let nums = vec![3,2,4];
    let target = 6;
    let result = two_sum(nums, target);
    println!("two_sum: {:?}", result);
}
