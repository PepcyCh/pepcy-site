---
title: '[SDOI 2010] 捉迷藏'
date: 2017-05-26 22:50:20
tags: [k-d树]
categories: 题解（OI/XCPC）
---

## 题目大意

给定平面上 $n$ 个点，求一个点使得它到其余点的曼哈顿距离的最大最小值之差最小，输出这个差。数据保证没有重点。

$2 \leqslant n \leqslant 500,000$

$0 \leqslant x, y \leqslant 100,000,000$

## 题目链接

[【SDOI 2010】捉迷藏 - Luogu 2479](https://www.luogu.com.cn/problem/P2479)

<!-- more -->

## 题解

k-d 树 + 枚举。

建树后枚举每个点计算即可。

之前的模版上少了一点剪枝。。。

## 代码

```c++
#include <cstdio>
#include <cstdlib>
#include <climits>
#include <algorithm>
const int MAXN = 500005;
struct Point {
    int x, y;
    Point(int x = 0, int y = 0) : x(x), y(y) {}
    bool operator!=(const Point &another) const {
        return x != another.x || y != another.y;
    }
};
int dist(const Point &a, const Point &b) {
    return abs(a.x - b.x) + abs(a.y - b.y);
}
struct KDTree {
    static bool cmp1(const Point &a, const Point &b) {
        return a.y < b.y || (a.y == b.y && a.x < b.x);
    }
    static bool cmp2(const Point &a, const Point &b) {
        return a.x < b.x || (a.x == b.x && a.y < b.y);
    }
    struct Node {
        Node *c[2];
        Point p, r1, r2;
        Node() {}
        Node(const Point &p) : p(p), r1(p), r2(p) {}
        void maintain() {
            if (c[0]) {
                r1.x = std::min(r1.x, c[0]->r1.x);
                r1.y = std::min(r1.y, c[0]->r1.y);
                r2.x = std::max(r2.x, c[0]->r2.x);
                r2.y = std::max(r2.y, c[0]->r2.y);
            }
            if (c[1]) {
                r1.x = std::min(r1.x, c[1]->r1.x);
                r1.y = std::min(r1.y, c[1]->r1.y);
                r2.x = std::max(r2.x, c[1]->r2.x);
                r2.y = std::max(r2.y, c[1]->r2.y);
            }
        }
        int distMax(const Point &p) {
            int res = 0;
            res += std::max(p.x - r1.x, r2.x - p.x);
            res += std::max(p.y - r1.y, r2.y - p.y);
            return res;
        }
        void queryMax(const Point &p, int &res) {
            res = std::max(res, dist(p, this->p));
            if (!c[0] && !c[1]) return;
            int k = c[0] && c[1] ? c[0]->distMax(p) < c[1]->distMax(p) : (c[0] ? 0 : 1);
            if (c[k]->distMax(p) > res) c[k]->queryMax(p, res);
            if (c[k ^ 1] && c[k ^ 1]->distMax(p) > res) c[k ^ 1]->queryMax(p, res);
        }
        int distMin(const Point &p) {
            int res = 0;
            if (p.x < r1.x || p.x > r2.x) res += p.x < r1.x ? r1.x - p.x : p.x - r2.x;
            if (p.y < r1.y || p.y > r2.y) res += p.y < r1.y ? r1.y - p.y : p.y - r2.y;
            return res;
        }
        void queryMin(const Point &p, int &res) {
            if (p != this->p) res = std::min(res, dist(p, this->p));
            if (!c[0] && !c[1]) return;
            int k = c[0] && c[1] ? c[0]->distMin(p) > c[1]->distMin(p) : (c[0] ? 0 : 1);
            if (c[k]->distMin(p) < res) c[k]->queryMin(p, res);
            if (c[k ^ 1] && c[k ^ 1]->distMin(p) < res) c[k ^ 1]->queryMin(p, res);
        }
    } *root, _pool[MAXN], *_curr;
    KDTree() : root(NULL), _curr(_pool) {}
    Node *build(Point *l, Point *r, int d = 0) {
        if (l > r) return NULL;
        if (l == r) return new (_curr++) Node(*l);
        Point *mid = l + (r - l) / 2;
        std::nth_element(l, mid, r + 1, d ? cmp1 : cmp2);
        Node *u = new (_curr++) Node(*mid);
        u->c[0] = build(l, mid - 1, d ^ 1);
        u->c[1] = build(mid + 1, r, d ^ 1);
        u->maintain();
        return u;
    }
    int queryMax(const Point &p) {
        int res = 0;
        root->queryMax(p, res);
        return res;
    }
    int queryMin(const Point &p) {
        int res = INT_MAX;
        root->queryMin(p, res);
        return res;
    }
} kdT;
int main() {
    int n;
    scanf("%d", &n);
    static Point P[MAXN];
    for (int i = 0; i < n; i++) scanf("%d %d", &P[i].x, &P[i].y);
    kdT.root = kdT.build(P, P + n - 1);
    int ans = INT_MAX;
    for (int i = 0; i < n; i++) ans = std::min(ans, kdT.queryMax(P[i]) - kdT.queryMin(P[i]));
    printf("%d\n", ans);
    return 0;
}
```