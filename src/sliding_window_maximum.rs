use std::collections::VecDeque;

// Given an array nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position. Return the max sliding window.

// Input: nums = [1,3,-1,-3,5,3,6,7], and k = 3
// Output: [3,3,5,5,6,7]

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return nums
    }
    
    let mut res: Vec<i32> = Vec::new();
    
    // init the deque
    let mut deque: VecDeque<i32> = VecDeque::new();
    
    for i in 0..k {
        // clean the deque of any values less than the currently selected one
        if deque.len() > 0 && deque[0] < i - k {
            println!("{}", deque[0]);
            println!("{}", i);
        } 
        
        deque.push_back(i);
        res.push_back(nums[i]);
        
        
        // append the value to the deque
        // add the 
    }
    
    return nums
}

fn main() {
    let nums: Vec<i32> = vec![1,3,-1,-3,5,3,6,7];
    let k: i32 = 3;
    max_sliding_window(nums, k);
}