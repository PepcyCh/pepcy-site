---
title: '[CQOI 2016] K远点对'
date: 2017-04-30 08:55:50
tags: [k-d树, 模版]
categories: 题解（OI/XCPC）
---

## 题目大意

给定平面内 $n$ 个点的坐标，求欧几里得距离下的第 $k$ 远点对，输出其欧几里得距离的平方。

$1 \leqslant n \leqslant 100,000$

$1 \leqslant k \leqslant 100$

$0 \leqslant x,\; y < 2^{31}$

## 题目链接

[【CQOI 2016】K 远点对](https://loj.ac/problem/2043)

<!-- more -->

## 题解

k-d 树裸题。

枚举每一个点为询问点，求 $2 k$ 远点（每一对点都被算了两次）。

TLE 了四次。。。把某一个 `long long` 写成了 `int`，其实是 WA 的说。。。

## 代码

```c++
#include <cstdio>
#include <climits>
#include <vector>
#include <queue>
#include <algorithm>
const int MAXN = 100005;
struct Point {
    int x, y;
    Point(int x = 0, int y = 0) : x(x), y(y) {}
} P[MAXN];
long long dist(const Point &a, const Point &b) {
    return (long long) (a.x - b.x) * (a.x - b.x) + (long long) (a.y - b.y) * (a.y - b.y);
}
struct KDTree {
    static bool cmp1(const Point &a, const Point &b) {
        return a.y < b.y || (a.y == b.y && a.x < b.x);
    }
    static bool cmp2(const Point &a, const Point &b) {
        return a.x < b.x || (a.x == b.x && a.y < b.y);
    }
    std::priority_queue<long long, std::vector<long long>, std::greater<long long> > q;
    struct Node {
        Node *c[2];
        Point p, r1, r2;
        Node() {}
        Node(Point p) : p(p), r1(p), r2(p) {
            c[0] = c[1] = NULL;
        }
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
        long long dist(const Point &p) {
            return std::max(std::max(::dist(p, r1), ::dist(p, r2)), 
                            std::max(::dist(p, Point(r1.x, r2.y)), ::dist(p, Point(r2.x, r1.y))));
        }
        void query(const Point &p, std::priority_queue<long long, std::vector<long long>, std::greater<long long> > &q) {
            long long d = ::dist(p, this->p);
            if (d > q.top()) q.pop(), q.push(d);
            if (!(c[0] || c[1])) return;
            long long dis[2] = {c[0] ? c[0]->dist(p) : INT_MIN, 
                                c[1] ? c[1]->dist(p) : INT_MIN};
            int k = dis[0] < dis[1];
            c[k]->query(p, q);
            if (c[k ^ 1] && dis[k ^ 1] > q.top()) c[k ^ 1]->query(p, q);
        }
    } *root, _pool[MAXN], *_cur;
    KDTree() : root(NULL) {
        _cur = _pool;
    }
    Node *build(Point *l, Point *r, int d = 0) {
        if (l > r) return NULL;
        if (l == r) return new (_cur++) Node(*l);
        Point *mid = l + (r - l) / 2;
        std::nth_element(l, mid, r + 1, d ? cmp1 : cmp2);
        Node *u = new (_cur++) Node(*mid);
        u->c[0] = build(l, mid - 1, d ^ 1);
        u->c[1] = build(mid + 1, r, d ^ 1);
        u->maintain();
        return u;
    }
    long long query(Point P[], int n, int k) {
        while (!q.empty()) q.pop();
        for (int i = 0; i < k << 1; i++) q.push(-1);
        for (int i = 1; i <= n; i++) root->query(P[i], q);
        return q.top();
    }
} kdT;
int main() {
    int n, k;
    scanf("%d %d", &n, &k);
    for (int i = 1; i <= n; i++) scanf("%d %d", &P[i].x, &P[i].y);
    kdT.root = kdT.build(P + 1, P + n);
    printf("%lld\n", kdT.query(P, n, k));
    return 0;
}
```