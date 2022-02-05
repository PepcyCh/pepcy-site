---
title: Minkowski sum（闵可夫斯基和）学习笔记
date: 2017-06-11 15:58:31
tags: [学习笔记, 模版, 计算几何]
categories: 学习笔记（OI/XCPC）
---

## 引入问题

逆时针给出平面上两个凸 $n$ 边形，询问 $q$ 个点是否可以成为一条两个端点分别在两个多边形内的线段的中点。

$n, q \leqslant 100,000$

## Minkowski sum 介绍

给定向量集合 $A$ 、$B$，定义它们的 Minkowski sum 为：
$$
A + B = \{\mathbf{a} + \mathbf{b} | \mathbf{a} \in A, \mathbf{b} \in B\}
$$
<!-- more -->

举个栗子（$A$ 、$B$ 、$A + B$ 均为多边形）：
$$
A = \{(0, 1), (0, -1), (1, 0)\} \\ 
B = \{(1, 1), (1, -1), (0, 0)\} \\
A + B = \{(1, 0), (2, 1), (2, −1), (0, 1), (1, 2), (1, 0), (0, −1), (1, 0), (1, −2)\}
$$

## 凸多边形的 Minkowski sum 的求法

对于两个 $n$ 边形，暴力复杂度 $O(n^2)$ 。

用 $n$ 个逆时针的向量表示每个凸多边形，把这些向量按极角排序，顺次相接，再平移一下，即得到它们的 Minkowski sum，复杂度 $O(n \log n)$ 。

我们可以先找到一个起点，然后对向量做以上操作，这样就不用平移了。一般 $y$ 值最大的点（有多个则为它们中 $x$ 值最小的点）为起点，至于为什么取它为起点，看一看 `atan2()` 的取值就知道了。

对于多个凸多边形，可以一起做。

## 回到问题

记两个凸多边形为 $A$、$B$，发现合法中点的集合为 $\frac{A + B}{2}$，让询问点都 $\times 2$ ，判断是否在 Minkowski sum 内即可。

两个凸 $n$ 边形的 Minkowski sum 也是凸多边形，点数在 $[n, 2n]$ 内。复杂度 $O(n \log n + nq)$ 。（但似乎实际比较快？）

## 问题代码

```c++
#include <cstdio>
#include <cmath>
#include <vector>
#include <algorithm>
const int MAXN = 100005;
const double EPS = 1e-9;
int dcmp(double x) {
    if (fabs(x) <= EPS) return 0;
    return x > EPS ? 1 : -1;
}
struct Point {
    double x, y;
    Point(double x = 0, double y = 0) : x(x), y(y) {}
    bool operator<(const Point &another) const {
        return atan2(y, x) < atan2(another.y, another.x);
    }
    friend Point operator+(const Point &a, const Point &b) {
        return Point(a.x + b.x, a.y + b.y);
    }
    friend Point operator-(const Point &a, const Point &b) {
        return Point(a.x - b.x, a.y - b.y);
    }
    friend double cross(const Point &a, const Point &b) {
        return a.x * b.y - a.y * b.x;
    }
    friend double dot(const Point &a, const Point &b) {
        return a.x * b.x + a.y * b.y;
    }
};
bool isPointOnSegment(const Point &p, const Point &s, const Point &t) {
    return dcmp(cross(s - p, t - p)) == 0 && dcmp(dot(s - p, t - p)) <= 0;
}
struct Poly {
    std::vector<Point> P;
    int size() const {
        return P.size();
    }
    Point operator[](int i) const {
        return P[i];
    }
	bool doseContain(const Point &p) {
        int windingNum = 0;
        for (int i = 0; i < size(); i++) {
            if (isPointOnSegment(p, P[i], P[(i + 1) % size()])) return true;
            int k = dcmp(cross(P[(i + 1) % size()] - P[i], p - P[i]));
            int d1 = dcmp(P[i].y - p.y);
            int d2 = dcmp(P[(i + 1) % size()].y - p.y);
            if (k > 0 && d1 <= 0 && d2 > 0) windingNum++;
            if (k < 0 && d2 <= 0 && d1 > 0) windingNum--;
        }
        return windingNum;
    }
};
int main() {
    Point top[2];
    static std::vector<Point> vec;
    for (int i = 0; i < 2; i++) {
        int n;
        scanf("%d", &n);
        Point first, last, curr;
        scanf("%lf %lf", &last.x, &last.y);
        top[i] = first = last;
        for (int j = 1; j < n; j++) {
            scanf("%lf %lf", &curr.x, &curr.y);
            vec.push_back(curr - last);
            last = curr;
            if (dcmp(curr.y - top[i].y) > 0 || (dcmp(curr.y - top[i].y) == 0 && dcmp(curr.x - top[i].x) < 0))
                top[i] = curr;
        }
        vec.push_back(first - last);
    }
    std::sort(vec.begin(), vec.end());
    Point start = top[0] + top[1];
    Poly poly;
    for (int i = 0; i < vec.size(); i++) {
        poly.P.push_back(temp + vec[i]);
        temp = temp + vec[i];
    }
    int q;
    scanf("%d", &q);
    while (q--) {
        int x, y;
        scanf("%d %d", &x, &y);
        puts(poly.doseContain(Point(x << 1, y << 1)) ? "YES" : "NO");
    }
    return 0;
}
```

## 说点别的（应该 OI 无关）

与 Minkowski sum 对应的，有一个 Minkowski difference（闵可夫斯基差）：
$$
A - B = \{\mathbf{v} | \mathbf{v} + B \subseteq A\}
$$
并不存在 $A - B = A + (-B)$。

考虑给出一个房间和一个类似于扫地机器人之类的东西的形状，求机器人能走到的位置集合，用机器人上一点 $P$ 的范围表示，答案就是它们的 Minkowski difference ，其中要以 $P$ 作为坐标原点。