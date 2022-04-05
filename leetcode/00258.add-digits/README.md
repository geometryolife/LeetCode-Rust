# [258. Add Digits](https://leetcode.com/problems/add-digits/)

## 题目 - 各位相加

Given an integer `num`, repeatedly add all its digits until the result has only one digit, and return it.

**Example 1:**

    Input: num = 38
    Output: 2
    Explanation: The process is
    38 --> 3 + 8 --> 11
    11 --> 1 + 1 --> 2 
    Since 2 has only one digit, return it.

**Example 2:**

    Input: num = 0
    Output: 0

**Constraints:**

- 0 <= num <= 2³¹ - 1

**Follow up:**

Could you do it without any loop/recursion in `O(1)` runtime?

## 题目大意

给定一个非负整数 num，反复将各个位上的数字相加，直到结果为一位数。返回这个结果。

## 解题思路

最直观的方法是模拟各位相加的过程，直到剩下的数字是一位数。
