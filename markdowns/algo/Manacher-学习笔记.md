---
title: Manacher 学习笔记
date: 2017-04-24 21:48:13
tags: [Manacher, 字符串, 学习笔记]
categories: 学习笔记（OI/XCPC）
---

## 算法介绍

Manacher 算法是能在 $O(n)$ 求解以每个位置为中心的最长回文子串长度的算法。

## 算法过程

在原字符串 $s$ 的两头分别加入不在字符集内的字符，比如 `@`、`\0`，同时，在每相邻两个字符间加入不在字符集内的字符，比如 `#`，这样对于偶数长的回文子串也能像奇数长的回文子串一样处理。以此得到新字符串 $s'$ ，之后的操作均在 $s'$ 上。

定义 $right$ 为已经算出来的所有回文子串中，右侧最远的一个的右侧端点，$pos$ 为其回文中心。

假设目前在考虑位置 $i$ ，记 $j = 2 pos - i$ ，即 $i$ 关于 $pos$ 的对称点。分三种情况：

<!-- more -->

* $right < i$

  无特殊性质，$r_i \geqslant 1$

* $right \geqslant i, \; j - r_j \geqslant 2 pos - right$

  $r_i \geqslant r_j$

* $right \geqslant i, \; j - r_j < 2 pos - right$

  $r_i \geqslant right - i$

在由以上三种情况对应的最小值的基础上进行比较。

## 模版题

[POJ 3974](http://poj.org/problem?id=3974) （[题解](http://pepcy.cf/POJ-3974-Palindrome/)）