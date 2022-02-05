---
title: '[Codeforces Round 526] Div1-B/Div2-E The Fair Nut and Strings'
date: 2019-01-05 11:12:41
tags: [Trie, 字符串, 贪心]
categories: 题解（OI/XCPC）
---

## 题目大意

本题中所有字符串均只由 $a$、$b$ 两种字符组成。

给出两个长为 $n$ 的字符串 $s$ 和 $t$，保证有字典序 $s \leq t$ 。现要给出 $k$ 个长为 $n$ 的、字典序介于 $s$ 与 $t$ 的字符串，使得可以作为其中至少一个字符串的前缀的字符串数目最多，求这个数目。

$1 \leq n \leq 500,000$

$1 \leq k \leq 10^9$

## 题目链接

[Codeforces Round 526 - Div1-B/Div2-E](https://codeforces.com/contest/1083/problem/B)

<!-- more -->

## 题解

考虑一棵以 $s$ 和 $t$ 为「边界」的 Trie。假如已经找好了所有 $k$ 个字符串，放入后叶子结点的数目就是 $k$，除根节点外的总节点数就是所求数目，考虑如何最大化这个数目。

从头到尾考虑字符串，考虑 Trie 的形态，在更早尽可能多地分支会使答案更大，于是一个贪心策略就是：每次尽可能多地分支，同时维护当前层的节点数，直到该层节点数不小于 $k$。

感觉是一道很妙的题。

## 代码

```c++
#include <cstdio>

const int MAXN = 500005;

char s[MAXN], t[MAXN];

int main() {
    int n, k;
    scanf("%d %d %s %s", &n, &k, s, t);

    long long ans = 0, curr = 1;
    for (int i = 0; i < n; i++) {
        curr = curr * 2 - int(s[i] == 'b') - int(t[i] == 'a');
        if (curr >= k) {
            ans += 1ll * k * (n - i);
            break;
        } else ans += curr;
    }

    printf("%lld\n", ans);
    
    return 0;
}
```