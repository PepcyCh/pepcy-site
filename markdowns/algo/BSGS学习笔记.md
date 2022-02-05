---
title: BSGS学习笔记
date: 2017-04-07 21:09:52
tags: [学习笔记, 数论, BSGS]
categories: 学习笔记（OI/XCPC）
---

## 算法介绍

BSGS（Baby-Step Giant-Step）算法用于求如下方程的解：
$$
a^x \equiv b \; (mod \; p), \; p \ 为质数
$$
<!-- more -->

令 $m = \lceil \sqrt p \rceil$。根据费马小定理，有 $a^{p - 1} \equiv 1\; (mod\; p)$，故若方程有解，则必然存在一个解满足 $0 \leqslant x < p - 1$。设该解为 $x = im + j$，其中 $0 \leq i,\ j \leq m$。

方程可化为：
$$
\begin{align}
a^x &\equiv b\; (mod \; p) \\
a^{im + j} &\equiv b\; (mod\; p) \\
a^j &\equiv b a^{-im}\; (mod\; p) \\
a^j &\equiv b (a^{-m})^i (mod\; p) \\
\end{align}
$$
我们只需要找到一组 $i$、$j$ 使得最后一个式子成立即可。

枚举 $j$，求出左边$a^j \; mod \; p$ 的所有取值，并将以$(j, \; a^j)$的映射关系插入到一个表中。

之后，求出$a^m$在模$p$意义下的乘法逆元。枚举 $i$，求出所有的 $b(a^{-m})^i$，每得到一个值后，从表中查找该值，如果存在，取出其对应的$j$，$x = im + j$即为一个解。

时间复杂度为 $O(\sqrt p)$。

## 模板题

[BZOJ 2242](http://www.lydsy.com/JudgeOnline/problem.php?id=2242) （[题解](http://pepcy.cf/BZOJ-2242-SDOI-2011-计算器/)）