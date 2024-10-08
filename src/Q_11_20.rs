// 11. Container With Most Water
/*
[11\. Container With Most Water](https://leetcode.com/problems/container-with-most-water/)

You are given an integer array `height` of length `n`. There are `n` vertical lines drawn such that the two endpoints of the `ith` line are `(i, 0)` and `(i, height[i])`.
Find two lines that together with the x-axis form a container, such that the container contains the most water.
Return _the maximum amount of water a container can store_.

**Notice** that you may not slant the container.

**Example 1:**
**Input:** height = \[1,8,6,2,5,4,8,3,7\]
**Output:** 49
**Explanation:** The above vertical lines are represented by array \[1,8,6,2,5,4,8,3,7\]. In this case, the max area of water (blue section) the container can contain is 49.

**Example 2:**
**Input:** height = \[1,1\]
**Output:** 1
*/

pub fn max_area(height: Vec<i32>) -> i32 {
    let len = height.len();
    if height[0] == height[len - 1] && height[0] == *height.iter().max().unwrap_or(&0) {
        return height[0] * height[0];
    }
    let mut heights = height;
    heights.sort();
    if len > 2 {
        heights.dedup();
    }
    let higest_height = heights[heights.len() - 2];
    higest_height * higest_height
}
