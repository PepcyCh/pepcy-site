---
title: '[HAOI 2006] 旅行'
date: 2017-04-17 22:17:18
tags: [并查集, 枚举]
categories: 题解（OI/XCPC）
---

## 题目大意

给定一个 $n$ 个节点、$m$ 条边的无向图，每条边有一个权值 $w_i$。给你两个顶点 $s$ 和 $t$ ，求一条路径，使得路径上最大边和最小边的比值最小。如果 $s$ 和 $t$ 之间没有路径，输出 `IMPOSSIBLE`；否则输出这个比值，如果答案不为整数，表示成一个约分到最简的分数。

$1 \leqslant n \leqslant 500$

$1 \leqslant m \leqslant 5,000$

$1 \leqslant w_i < 30,000$

## 题目链接

[【HAOI 2006】旅行 - Luogu 2502](https://www.luogu.com.cn/problem/P2502)

<!-- more -->

## 题解

对边进行排序，用类似 Kruscal 的方法，依次加入每条边，直至 $s$ 与 $t$ 连通，更新答案，枚举从哪条边开始添边即可。

## 代码

```c++
#include <cstdio>
#include <cfloat>
#include <algorithm>
const int MAXN = 505;
const int MAXM = 5005;
struct Edge {
    int u, v, w;
    bool operator<(const Edge &another) const {
        return w < another.w;
    }
} E[MAXM];
struct UnionFindSet {
    int fa[MAXN], n;
    int find(int x) {
        return x == fa[x] ? x : fa[x] = find(fa[x]);
    }
    void merge(int x, int y) {
        int p = find(x), q = find(y);
        if (p == q) return;
        fa[q] = p;
    }
    void init(int n) {
        this->n = n;
        for (int i = 1; i <= n; i++) fa[i] = i;
    }
} ufs;
int gcd(int a, int b) {
    return b == 0 ? a : gcd(b, a % b);
}
int main() {
    int n, m;
    scanf("%d %d", &n, &m);
    for (int i = 0; i < m; i++) scanf("%d %d %d", &E[i].u, &E[i].v, &E[i].w);
    std::sort(E, E + m);
    int s, t;
    scanf("%d %d", &s, &t);
    double ans = DBL_MAX;
    int ansMax, ansMin;
    for (int i = 0; i < m; i++) {
        ufs.init(n);
        int min = E[i].w, max;
        for (int j = i; j < m; j++) {
            max = E[j].w;
            ufs.merge(E[j].u, E[j].v);
            if (ufs.find(s) == ufs.find(t)) {
                if ((double) max / min < ans) {
                    ans = (double) max / min;
                    ansMax = max;
                    ansMin = min;
                }
                break;
            }
        }
    }
    if (ans == DBL_MAX) puts("IMPOSSIBLE");
    else if ((ansMax / ansMin) * ansMin == ansMax) printf("%.0lf\n", ans);
    else {
        int g = gcd(ansMax, ansMin);
        printf("%d/%d\n", ansMax / g, ansMin / g);
    }
    return 0;
}
```