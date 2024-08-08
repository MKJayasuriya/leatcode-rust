// 1470. Shuffle the Array
/*
[1470 Shuffle the Array](https://leetcode.com/problems/shuffle-the-array/)

Hint
Given the array `nums` consisting of `2n` elements in the form `[x1,x2,...,xn,y1,y2,...,yn]`.
_Return the array in the form_ `[x1,y1,x2,y2,...,xn,yn]`.

**Example 1:**

**Input:** nums = \[2,5,1,3,4,7\], n = 3
**Output:** \[2,3,5,4,1,7\]
**Explanation:** Since x1\=2, x2\=5, x3\=1, y1\=3, y2\=4, y3\=7 then the answer is \[2,3,5,4,1,7\].

**Example 2:**

**Input:** nums = \[1,2,3,4,4,3,2,1\], n = 4
**Output:** \[1,4,2,3,3,2,4,1\]

**Example 3:**

**Input:** nums = \[1,1,2,2\], n = 2
**Output:** \[1,2,1,2\]

**Constraints:**

-   `1 <= n <= 500`
-   `nums.length == 2n`
-   `1 <= nums[i] <= 10^3`
*/

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![];
    let n = n as usize;
    for i in 0..n {
        new_vec.push(nums[i]);
        new_vec.push(nums[i + n]);
    }
    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        let nums = vec![2, 5, 1, 3, 4, 7];
        let n = 3;
        let result = shuffle(nums, n);
        let expected = vec![2, 3, 5, 4, 1, 7];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_shuffle_empty() {
        let nums: Vec<i32> = vec![];
        let n = 0;
        let result = shuffle(nums, n);
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_shuffle_single_pair() {
        let nums = vec![1, 2];
        let n = 1;
        let result = shuffle(nums, n);
        let expected = vec![1, 2];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_shuffle_large() {
        let nums = vec![1, 1, 2, 2, 3, 3];
        let n = 3;
        let result = shuffle(nums, n);
        let expected = vec![1, 2, 1, 3, 2, 3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_shuffle_with_negatives() {
        let nums = vec![1, -1, 2, -2, 3, -3];
        let n = 3;
        let result = shuffle(nums, n);
        let expected = vec![1, -2, -1, 3, 2, -3];
        assert_eq!(result, expected);
    }
}
