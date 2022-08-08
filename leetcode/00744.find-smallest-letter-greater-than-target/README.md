# [744. Find Smallest Letter Greater Than Target](https://leetcode.com/problems/find-smallest-letter-greater-than-target/)

## 题目 - 寻找比目标字母大的最小字母

Given a characters array `letters` that is sorted in **non-decreasing** order and a character `target`, return the smallest character in the array that is larger than `target`.

Note that the letters wrap around.

For example, if `target == 'z'` and `letters == ['a', 'b']`, the answer is `'a'`.

**Example 1:**

    Input: letters = ["c","f","j"], target = "a"
    Output: "c"

**Example 2:**

    Input: letters = ["c","f","j"], target = "c"
    Output: "f"

**Example 3:**

    Input: letters = ["c","f","j"], target = "d"
    Output: "f"

**Constraints:**

- 2 <= letters.length <= 10⁴
- letters[i] is a lowercase English letter.
- letters is sorted in non-decreasing order.
- letters contains at least two different characters.
- target is a lowercase English letter.

## 题目大意

给定一个只包含小写字母的有序（非递减）数组 letters 和一个目标字母 target。
寻找有序数组中比目标字母大的最小字母。

## 解题思路

从头到尾遍历字符数组的过程中，比较数组字母与目标字母，符合条件则返回当前字母；
若遍历结束还没找到符合条件的字母，则返回数组的第一个元素。
（借助 rust 的 iter 方法创建可遍历数组的迭代器。）
