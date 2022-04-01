# [344. Reverse String](https://leetcode.com/problems/reverse-string/)

## 题目

Write a function that reverses a string. The input string is given as an array of characters `s`.

You must do this by modifying the input array `in-place` with `O(1)` extra memory.

**Example 1:**

    Input: s = ["h","e","l","l","o"]
    Output: ["o","l","l","e","h"]

**Example 2:**

    Input: s = ["H","a","n","n","a","h"]
    Output: ["h","a","n","n","a","H"]

**Constraints:**

- 1 <= s.length <= 10⁵
- s[i] is a printable ascii character.

## 题目大意

反转一个字符串，输入一个字符数组，输出一个反转后的字符数组。

## 解题思路

用 2 个指针，通过指针对撞思路，不断交换首尾元素。
