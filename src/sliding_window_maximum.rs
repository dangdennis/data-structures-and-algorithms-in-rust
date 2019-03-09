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
    let mut max_index = 0;

    for i in 0..k {
        // remove items that fall outside the window
        if deque.len() > 0 && deque[0] < i - k + 1 {
            dbg!(&deque);
            deque.pop_front();
        } 

        // clean the deque of any values less than the currently selected one
        while (deque.len() > 0) && nums[*deque.back().unwrap() as usize] < nums[i as usize] {
          deque.pop_back();
        }

        // add the current i
        deque.push_back(i);
        if nums[max_index] < nums[i as usize] {
            max_index = i as usize;
        }
    }

    res.push(nums[max_index as usize]);

    for i in k..nums.len() as i32 {
        // remove items that fall outside the window
        if deque.len() > 0 && deque[0] < i - k + 1 {
            dbg!(&deque);
            deque.pop_front();
        } 

        // clean the deque of any values less than the currently selected one
        while (deque.len() > 0) && nums[*deque.back().unwrap() as usize] < nums[i as usize] {
          deque.pop_back();
        }

        deque.push_back(i);

        res.push(nums[deque[0] as usize]);
    }
    res
}

fn main() {
    let nums: Vec<i32> = vec![1,3,-1,-3,5,3,6,7];
    let k: i32 = 3;
    let res = max_sliding_window(nums, k);
}