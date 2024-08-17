// 1512. Number of Good Pairs
/*
[1512\. Number of Good Pairs](https://leetcode.com/problems/number-of-good-pairs/)

Given an array of integers `nums`, return _the number of **good pairs**_.
A pair `(i, j)` is called _good_ if `nums[i] == nums[j]` and `i` < `j`.

**Example 1:**
**Input:** nums = \[1,2,3,1,1,3\]
**Output:** 4
**Explanation:** There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.

**Example 2:**
**Input:** nums = \[1,1,1,1\]
**Output:** 6
**Explanation:** Each pair in the array are _good_.

**Example 3:**
**Input:** nums = \[1,2,3\]
**Output:** 0
*/
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut val: i32 = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] == nums[j] && i < j {
                val += 1;
            }
        }
    }
    val
}
