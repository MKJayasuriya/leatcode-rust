// 1. Two Sum
/*
[1\. Two Sum](https://leetcode.com/problems/two-sum/)

Hint

Given an array of integers `nums` and an integer `target`, return _indices of the two numbers such that they add up to `target`_.
You may assume that each input would have **_exactly_ one solution**, and you may not use the _same_ element twice.
You can return the answer in any order.

**Example 1:**
**Input:** nums = \[2,7,11,15\], target = 9
**Output:** \[0,1\]
**Explanation:** Because nums\[0\] + nums\[1\] == 9, we return \[0, 1\].

**Example 2:**
**Input:** nums = \[3,2,4\], target = 6
**Output:** \[1,2\]

**Example 3:**
**Input:** nums = \[3,3\], target = 6
**Output:** \[0,1\]
*/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n {
        for j in i + 1..n {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

// 2. Add Two Numbers
/*
[2\. Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)

You are given two **non-empty** linked lists representing two non-negative integers. The digits are stored in **reverse order**, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
You may assume the two numbers do not contain any leading zero, except the number 0 itself.

**Example 1:**
**Input:** l1 = \[2,4,3\], l2 = \[5,6,4\]
**Output:** \[7,0,8\]
**Explanation:** 342 + 465 = 807.

**Example 2:**
**Input:** l1 = \[0\], l2 = \[0\]
**Output:** \[0\]

**Example 3:**
**Input:** l1 = \[9,9,9,9,9,9,9\], l2 = \[9,9,9,9\]
**Output:** \[8,9,9,9,0,0,0,1\]
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: add_two_numbers(n1.next, n2.next),
                }))
            } else {
                let carry = Some(Box::new(ListNode::new(1)));
                Some(Box::new(ListNode {
                    val: sum - 10,
                    next: add_two_numbers(add_two_numbers(carry, n1.next), n2.next),
                }))
            }
        }
    }
}

// 4. Median of Two Sorted Arrays
/*
[4\. Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)

Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return **the median** of the two sorted arrays.
The overall run time complexity should be `O(log (m+n))`.

**Example 1:**
**Input:** nums1 = \[1,3\], nums2 = \[2\]
**Output:** 2.00000
**Explanation:** merged array = \[1,2,3\] and median is 2.

**Example 2:**
**Input:** nums1 = \[1,2\], nums2 = \[3,4\]
**Output:** 2.50000
**Explanation:** merged array = \[1,2,3,4\] and median is (2 + 3) / 2 = 2.5.
 */

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut array = nums1;
    let mut arr2 = nums2;
    array.append(&mut arr2);
    array.sort();
    if array.len() % 2 != 0 {
        array[array.len() / 2] as f64
    } else {
        let val = array.len() / 2;
        (array[val - 1] + array[val]) as f64 / 2.0
    }
}

// 7. Reverse Integer
/*
[7\. Reverse Integer](https://leetcode.com/problems/reverse-integer/)

Given a signed 32-bit integer `x`, return `x` _with its digits reversed_. If reversing `x` causes the value to go outside the signed 32-bit integer range `[-231, 231 - 1]`, then return `0`.
**Assume the environment does not allow you to store 64-bit integers (signed or unsigned).**

**Example 1:**
**Input:** x = 123
**Output:** 321

**Example 2:**
**Input:** x = -123
**Output:** -321

**Example 3:**
**Input:** x = 120
**Output:** 21
*/

pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut y: i64 = 0;
    while x != 0 {
        let m = x % 10;
        x /= 10;
        y = (y * 10) + m as i64;
    }
    if y > i32::MAX as i64 || y < i32::MIN as i64 {
        return 0;
    }
    y as i32
}

// 8. String to Integer
/*
[8\. String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/)

Implement the `myAtoi(string s)` function, which converts a string to a 32-bit signed integer.
The algorithm for `myAtoi(string s)` is as follows:

1.  **Whitespace**: Ignore any leading whitespace (`" "`).
2.  **Signedness**: Determine the sign by checking if the next character is `'-'` or `'+'`, assuming positivity is neither present.
3.  **Conversion**: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
4.  **Rounding**: If the integer is out of the 32-bit signed integer range `[-231, 231 - 1]`, then round the integer to remain in the range. Specifically, integers less than `-231` should be rounded to `-231`, and integers greater than `231 - 1` should be rounded to `231 - 1`.

Return the integer as the final result.

**Example 1:**
**Input:** s = "42"
**Output:** 42

**Explanation:**
The underlined characters are what is read in and the caret is the current reader position.
Step 1: "42" (no characters read because there is no leading whitespace)
         ^
Step 2: "42" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "42" ("42" is read in)
           ^

**Example 2:**
**Input:** s = " -042"
**Output:** \-42

**Explanation:**
Step 1: "   \-042" (leading whitespace is read and ignored)
            ^
Step 2: "   \-042" ('-' is read, so the result should be negative)
             ^
Step 3: "   -042" ("042" is read in, leading zeros ignored in the result)
               ^

**Example 3:**
**Input:** s = "1337c0d3"
**Output:** 1337

**Explanation:**
Step 1: "1337c0d3" (no characters read because there is no leading whitespace)
         ^
Step 2: "1337c0d3" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "1337c0d3" ("1337" is read in; reading stops because the next character is a non-digit)
             ^

**Example 4:**
**Input:** s = "0-1"
**Output:** 0

**Explanation:**
Step 1: "0-1" (no characters read because there is no leading whitespace)
         ^
Step 2: "0-1" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "0\-1" ("0" is read in; reading stops because the next character is a non-digit)
          ^

**Example 5:**
**Input:** s = "words and 987"
**Output:** 0

**Explanation:**
Reading stops at the first non-digit character 'w'
*/

pub fn my_atoi(s: String) -> i32 {
    let mut y = String::new();
    let mut sign = true;
    let mut is_stat = true;
    for char in s.trim().chars() {
        if is_stat && char == '-' {
            is_stat = false;
            sign = false;
        } else if is_stat && char == '+' {
            is_stat = false;
            sign = true;
        } else if char.is_ascii_digit() {
            is_stat = false;
            y.push(char);
        } else {
            break;
        }
    }

    if y.is_empty() {
        return 0;
    }

    match y.parse::<i32>() {
        Ok(num) => {
            if sign {
                num
            } else {
                -num
            }
        }
        Err(_) => {
            if sign {
                i32::MAX
            } else {
                i32::MIN
            }
        }
    }
}

// 9. Palindrome Number
/*
[9\. Palindrome Number](https://leetcode.com/problems/palindrome-number/)

Given an integer `x`, return `true` _if_ `x` _is a_
_**palindrome**_
_, and_ `false` _otherwise_.

**Example 1:**
**Input:** x = 121
**Output:** true
**Explanation:** 121 reads as 121 from left to right and from right to left.

**Example 2:**
**Input:** x = -121
**Output:** false
**Explanation:** From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

**Example 3:**
**Input:** x = 10
**Output:** false
**Explanation:** Reads 01 from right to left. Therefore it is not a palindrome.
*/

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }
    if x % 10 == 0 {
        return false;
    }
    let mut num = 0;
    let mut y = x;
    while y > 0 {
        num = (num * 10) + (y % 10);
        y /= 10;
    }
    x == num
}
