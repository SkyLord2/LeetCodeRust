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

fn main() {
    let nums = vec![3,2,4];
    let target = 6;
    let result = two_sum(nums, target);
    println!("two_sum: {:?}", result);
}
