---
title: '[Codeforces Hello 2019] F Alex and a TV Show'
date: 2019-01-05 10:49:40
tags: [莫比乌斯反演, 数学, 容斥原理]
categories: 题解（OI/XCPC）
---

## 题目大意

维护 $n$ 个可重集，有 $q$ 次操作，操作有四种：

* `1 x v` ：让第 $x$ 个集合变为 $\{v\}$
* `2 x y z` ：让第 $x$ 个集合变为第 $y$ 个集合与第 $z$ 个集合的并（因为是可重集，所以指的是直接合在一起）
* `3 x y z` ：让第 $x$ 个集合变为第 $y$ 个集合与第 $z$ 个集合的积，定义 $A \times B = \{\gcd(a, b) \mid a \in A, b \in B\}$
* `4 x v` ：询问 $v$ 在第 $x$ 个集合中出现的次数，对 $2$ 取模

$1 \leq n \leq 10^5$

$1 \leq q \leq 10^6$

$1 \leq v \leq 7,000$

## 题目链接

[Codeforces Hello 2019 - F](https://codeforces.com/contest/1097/problem/F)

<!-- more -->

## 题解

考虑到询问只对 $2$ 取模，再观察数据范围，意识到复杂度应该是 $O(qv / 32)$  ，考虑用 bitset 表示每一个集合，此时发现无法快速地完成操作三。

考虑存储约数而非数本身，则操作三就是两个 bitset 的按位与。通过预处理，前三个操作都可以做到 $O(v / 32)$ 了，考虑如何快速回答询问。

记 $f(x)$ 表示在原集合中 $x$ 的出现次数，$g(x)$ 表示 $x$ 作为因子出现的次数（即转化后的集合中出现的次数），有：
$$
g(d) = \sum_{d \mid n} f(n)
$$
则：
$$
f(d) = \sum_{d | n} g(n) \mu(\frac{n}{d})
$$
通过预处理，所有操作均可做到 $O(v / 32)$。

## 代码

```c++
#include <bits/stdc++.h>

const int MAXN = 100005;
const int MAXV = 7005;

std::bitset<MAXV> bs[MAXN], num[MAXV], dv[MAXV];

void init() {
    for (int i = 1; i < MAXV; i++) for (int j = i; j < MAXV; j += i) num[j][i] = 1;

    static bool mu[MAXV];
    for (int i = 1; i < MAXV; i++) {
        mu[i] = true;
        for (int j = 2; j <= i; j++) if (i % j == 0) {
            mu[i] = (i / j % j == 0) ? false : mu[i / j];
            break;
        }
    }

    for (int i = 1; i < MAXV; i++) for (int j = 1; i * j < MAXV; j++) dv[i][i * j] = mu[j];
}

int main() {
    init();

    int n, q;
    scanf("%d %d", &n, &q);

    while (q--) {
        int op, x;
        scanf("%d %d", &op, &x);

        if (op == 1) {
            int v;
            scanf("%d", &v);
            bs[x] = num[v];
        } else if (op == 2) {
            int y, z;
            scanf("%d %d", &y, &z);
            bs[x] = bs[y] ^ bs[z];
        } else if (op == 3) {
            int y, z;
            scanf("%d %d", &y, &z);
            bs[x] = bs[y] & bs[z];
        } else {
            int v;
            scanf("%d", &v);
            putchar(((bs[x] & dv[v]).count() & 1) + '0');
        }
    }
    puts("");
    
    return 0;
}
```