---
title: '[ZOJ 3964] Yet Another Game of Stones'
date: 2019-03-30 21:55:06
tags: [博弈, 不公平博弈, NIM博弈]
categories: 题解（OI/XCPC）
---

## 题目大意

Alice 与 Bob 取 $n$ 堆石子（Alice 为先手），但每堆除数目 $a_i$，还有一个针对 Alice 的限制 $b_i$，具体地说：

* $b_i = 0$，则该堆对 Alice 无限制
* $b_i  = 1$，则 Alice 只能在该堆取奇数个石子
* $b_i = 2$，则 Alice 只能在该堆取偶数个石子

问 Alice 是否必胜。

$1 \leq n \leq 100,000$

$1 \leq a_i \leq 1,000,000,000$

## 题目链接

[ZOJ 3964](http://acm.zju.edu.cn/onlinejudge/showProblem.do?problemId=5596)

<!-- more -->

## 题解

一道非公平博弈的题，则要考虑什么时候公平，以及不公平时怎么变的公平。

首先，如果有一堆石子满足 $a_i = 1$ 且 $b_i = 1$，则此时这个限制是无用的，之后我们讨论的限制都是有用的限制。

如果所有堆都无有用限制，那就是普通的 NIM 游戏。

如果只有一个限制堆：

* Alice 可以将其取光，剩余石子堆为普通的 NIM 游戏。
* Alice 可以将其取至只剩一个，此时限制无用，为普通 NIM 游戏。
* 否则 Alice 对这一堆无可奈何，Bob 必胜。

如果有多个限制堆，则必有一个 Alice 无可奈何，Bob 必胜。

## 代码

```c++
#include <cstdio>

const int MAXN = 100005;

int a[MAXN], b[MAXN];

void solve() {
    int n, nim = 0, cnt = 0;
    scanf("%d", &n);
    for (int i = 0; i < n; i++) scanf("%d", &a[i]);
    for (int i = 0; i < n; i++) {
        scanf("%d", &b[i]);
        if (b[i] == 1 && a[i] == 1) b[i] = 0;
        if (!b[i]) nim ^= a[i];
        else ++cnt;
    }
    if (cnt > 1) {
        puts("Bob");
        return;
    } else if (cnt == 0) {
        puts(nim ? "Alice" : "Bob");
        return;
    }
    bool ans = true;
    for (int i = 0; i < n; i++) if (b[i]) {
        if (b[i] % 2 == a[i] % 2) {
            ans = !nim;
        } else if (b[i] == 1) {
            ans = nim == 1;
        } else {
            ans = false;
        }
    }
    puts(ans ? "Alice" : "Bob");
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) solve();
    return 0;
}
```