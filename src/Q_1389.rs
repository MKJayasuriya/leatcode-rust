// 1389. Create Target Array in the Given Order
/*
[1389\. Create Target Array in the Given Order](https://leetcode.com/problems/create-target-array-in-the-given-order/)

Given two arrays of integers `nums` and `index`. Your task is to create _target_ array under the following rules:
-   Initially _target_ array is empty.
-   From left to right read nums\[i\] and index\[i\], insert at index `index[i]` the value `nums[i]` in _target_ array.
-   Repeat the previous step until there are no elements to read in `nums` and `index.`
Return the _target_ array.
It is guaranteed that the insertion operations will be valid.

**Example 1:**
**Input:** nums = \[0,1,2,3,4\], index = \[0,1,2,2,1\]
**Output:** \[0,4,1,3,2\]
**Explanation:**
nums       index     target
0            0        \[0\]
1            1        \[0,1\]
2            2        \[0,1,2\]
3            2        \[0,1,3,2\]
4            1        \[0,4,1,3,2\]

**Example 2:**
**Input:** nums = \[1,2,3,4,0\], index = \[0,1,2,3,0\]
**Output:** \[0,1,2,3,4\]
**Explanation:**
nums       index     target
1            0        \[1\]
2            1        \[1,2\]
3            2        \[1,2,3\]
4            3        \[1,2,3,4\]
0            0        \[0,1,2,3,4\]

**Example 3:**
**Input:** nums = \[1\], index = \[0\]
**Output:** \[1\]
*/

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut tar_vec = vec![];
    for idx in 0..nums.len() {
        tar_vec.insert(index[idx] as usize, nums[idx]);
    }
    tar_vec
}
