---
title: '[POJ 3974] Palindrome'
date: 2017-04-24 21:02:07
tags: [字符串, 模版, Manacher]
categories: 题解（OI/XCPC）
---

## 题目大意

给定一个字符串，求其最长的回文子串的长度，多组询问。

$1 \leqslant|s| \leqslant 1,000,000$

$1 \leqslant T \leqslant 30$

## 题目链接

[POJ 3974](http://poj.org/problem?id=3974)

<!-- more -->

## 题解

Manacher 模版题，计算出$r_i$数组后找最大值即可。

## 代码

```c++
#include <cstdio>
#include <cstring>
#include <algorithm>
const int MAXN = 1000005;
char s[MAXN];
namespace Manacher {
    int len, r[MAXN << 1];
    char s[MAXN << 1];
    void prepare() {
        len = 0;
        s[++len] = '@';
        s[++len] = '#';
        int n = strlen(::s);
        for (int i = 0; i < n; i++) s[++len] = ::s[i], s[++len] = '#';
        s[++len] = '\0';
    }
    void manacher() {
        int right = 0, pos = 0;
        for (int i = 1; i <= len; i++) {
            int x = right < i ? 1 : std::min(r[2 * pos - i], right - i);
            while (s[i + x] == s[i - x]) x++;
            if (x + i > right) {
                right = x + i;
                pos = i;
            }
            r[i] = x;
        }
    }
    void calc() {
        prepare();
        manacher();
    }
}
int main() {
    int T = 0;
    while (scanf("%s", s), s[0] != 'E') {
        Manacher::calc();
        int ans = 0;
        for (int i = 1; i <= Manacher::len; i++) ans = std::max(ans, Manacher::r[i] - 1);
        printf("Case %d: %d\n", ++T, ans);
    }
    return 0;
}
```