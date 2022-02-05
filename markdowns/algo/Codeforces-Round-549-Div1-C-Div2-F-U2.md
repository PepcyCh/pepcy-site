---
title: '[Codeforces Round 549] Div1-C/Div2-F U2'
date: 2019-04-05 08:12:51
tags: [凸包, 计算几何]
categories: 题解（OI/XCPC）
---

## 题目大意

平面上任意两个横坐标不同的点可以确定一条形如 $y = x^2 + bx + c$ 的抛物线。现给出平面上的 $n$ 个点，以它们建出尽可能多的此种抛物线，使得任意一个点不在某个抛物线的上方（在线上不计）。

$1 \leq n \leq 100,000$

$|x_i|, |y_i| \leq 1,000,000$

## 题目链接

[Codeforces Round 549 - Div1-C/Div2-F](https://codeforces.com/contest/1142/problem/C)

<!-- more -->

## 题解

对式子进行移项，得 $y - x^2 = bx + c$，即把所有点改成 $(x_i, y_i - x_i^2)$，求它们的一个上凸包。

记得对相同横坐标的点只取纵坐标最大的一个，因为要求横坐标不同的点才能构成抛物线。

## 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 100005;

struct Point {
    long long x, y;

    Point() {}
    Point(long long x, long long y) : x(x), y(y) {}

    Point operator-(const Point &rhs) const { return Point(x - rhs.x, y - rhs.y); }
    friend long long cross(const Point &a, const Point &b) { return a.x * b.y - a.y * b.x; }
} P[MAXN], ch[MAXN];

int getConvexHull(int n) {
    std::sort(P, P + n, [](const Point &a, const Point &b) {
                return a.x == b.x ? a.y < b.y : a.x < b.x;
            });
    int p = 0;
    for (int i = 0; i < n; i++) if (i == n - 1 || P[i].x != P[i + 1].x) P[p++] = P[i];
    int m = 0;
    for (int i = 0; i < p; i++) {
        while (m > 1 && cross(ch[m - 1] - ch[m - 2], P[i] - ch[m - 2]) >= 0) --m;
        ch[m++] = P[i];
    }
    return m;
}

int main() {
    int n;
    scanf("%d", &n);
    for (int i = 0; i < n; i++) {
        scanf("%lld %lld", &P[i].x, &P[i].y);
        P[i].y = P[i].y - P[i].x * P[i].x;
    }
    int ans = getConvexHull(n) - 1;
    printf("%d\n", ans);
    
    return 0;
}
```