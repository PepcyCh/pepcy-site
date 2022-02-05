---
title: '[BZOJ 2654] tree'
date: 2017-05-06 21:50:36
tags: [二分, 最小生成树, 图]
categories: 题解（OI/XCPC）
---

## 题目大意

给定一个无向带权连通图，每条边是黑色或白色。求一棵边权和最小的恰好有 $need$ 条白色边的生成树。题目保证有解。

$1 \leqslant n \leqslant 50,000$

$1 \leqslant m \leqslant 100,000$

$1 \leqslant w_i \leqslant 100$

## 题目链接

[BZOJ 2654](http://www.lydsy.com/JudgeOnline/problem.php?id=2654)

<!-- more -->

## 题解

二分答案 + Kruskal 判定。

一开始以为是 [【HNOI 2006】公路修建问题](http://pepcy.cf/BZOJ-1196-HNOI-2006-公路修建问题/) ，后来发现那个是「至少有」，本题是「恰好有」。。。

跑两次 MST ，一次边权相同的让白色在前，一次让黑色在前，两次分别求出白色边数的最大值、最小值，如果 $need$ 在范围内，则输出答案，否则考虑如何改变 MST 中白边的数量。

我们二分出一个值（一开始为 $0$），让白边都加上这个值，如果该值为正，则 MST 中白边数可能会减少，为负则可能增加。以此方法直到 $need$ 在范围内，输出答案。

## 代码

```c++
#include <cstdio>
#include <algorithm>
const int MAXN = 50005;
const int MAXM = 100005;
const int MAXW = 105;
struct Edge {
    int u, v, w, color;
} E[MAXM];
bool cmp1(const Edge &a, const Edge &b) {
    return a.w < b.w || (a.w == b.w && a.color < b.color);
}
bool cmp2(const Edge &a, const Edge &b) {
    return a.w < b.w || (a.w == b.w && a.color > b.color);
}
struct UnionFindSet {
    int fa[MAXN];
    int find(int x) {
        return x == fa[x] ? x : fa[x] = find(fa[x]);
    }
    void merge(int x, int y) {
        int p = find(x), q = find(y);
        fa[q] = p;
    }
    void init(int n) {
        for (int i = 1; i <= n; i++) fa[i] = i;
    }
} ufs;
int n, m, need;
int check(int x) { // 0: more, -1: less, others: answer
    for (int i = 0; i < m; i++) if (!E[i].color) E[i].w += x;
    std::sort(E, E + m, cmp1);
    int max = 0, res = 0;
    ufs.init(n);
    for (int i = 0; i < m; i++) {
        int p = ufs.find(E[i].u), q = ufs.find(E[i].v);
        if (p == q) continue;
        ufs.merge(p, q);
        if (!E[i].color) max++;
        res += E[i].w;
    }
    std::sort(E, E + m, cmp2);
    int min = 0;
    ufs.init(n);
    for (int i = 0; i < m; i++) {
        int p = ufs.find(E[i].u), q = ufs.find(E[i].v);
        if (p == q) continue;
        ufs.merge(p, q);
        if (!E[i].color) min++;
    }
    for (int i = 0; i < m; i++) if (!E[i].color) E[i].w -= x;
    if (need > max) return -1;
    if (need < min) return 0;
    return res - x * need;
}
int dichotomy() {
    int l = -MAXW, r = MAXW;
    while (l <= r) {
        int mid = l + (r - l) / 2;
        int temp = check(mid);
        if (temp == 0) l = mid + 1;
        else if (temp == -1) r = mid - 1;
        else return temp;
    }
    return -1;
}
int main() {
    scanf("%d %d %d", &n, &m, &need);
    for (int i = 0; i < m; i++)
        scanf("%d %d %d %d", &E[i].u, &E[i].v, &E[i].w, &E[i].color);
    printf("%d\n", dichotomy());
    return 0;
}
```