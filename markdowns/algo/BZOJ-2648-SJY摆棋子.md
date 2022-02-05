---
title: 'SJY摆棋子'
date: 2017-04-29 21:04:05
tags: [k-d树, 模版]
categories: 题解（OI/XCPC）
---

## 题目大意

棋盘上有 $n$ 个黑色棋子，有 $m$ 次操作，放下一个黑色棋子或白色棋子，在放下白色棋子时，查询曼哈顿距离与其最近的黑色棋子，输出对应的曼哈顿距离。一个点上可能有多个棋子。

$1 \leqslant n, \; m \leqslant 500,000$

## 题目链接

[【Violet】天使玩偶/SJY摆棋子 - Luogu 4169](https://www.luogu.com.cn/problem/P4169)

<!-- more -->

## 题解

k-d 树裸题。

TLE 了四次。。。

## 代码

```c++
#include <cstdio>
#include <cstdlib>
#include <climits>
#include <cctype>
#include <algorithm>
#include <new>
// #define DBG
const int MAXN = 500005;
struct Point {
    int x, y;
    Point(int x = 0, int y = 0) : x(x), y(y) {}
#ifdef DBG
    void print() const {
        printf("Point: [x = %d, y = %d]\n", x, y);
    }
#endif
} P[MAXN];
int dist(const Point &a, const Point &b) {
    return abs(a.x - b.x) + abs(a.y - b.y);
}
struct KDTree {
    static bool cmp1(const Point &a, const Point &b) {
        return a.y < b.y ? a.y < b.y : (a.y == b.y && a.x < b.x);
    }
    static bool cmp2(const Point &a, const Point &b) {
        return a.x < b.x ? a.x < b.x : (a.x == b.x && a.y < b.y);
    }
    int res;
    struct Node {
        Node *c[2];
        Point p, r1, r2;
        Node() {}
        Node (Point p) : p(p), r1(p), r2(p) {
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
        int dist(const Point &p) {
            int res = 0;
            if (p.x < r1.x || r2.x < p.x) res += p.x < r1.x ? r1.x - p.x : p.x - r2.x;
            if (p.y < r1.y || r2.y < p.y) res += p.y < r1.y ? r1.y - p.y : p.y - r2.y;
            return res;
        }
        void query(const Point &p, int &res) {
#ifdef DBG
            printf("query():\n");
            this->p.print();
            printf("c[0]: %d, c[1]: %d\n", c[0] ? 1 : 0, c[1] ? 1 : 0);
#endif
            res = std::min(res, ::dist(this->p, p));
            if (!(c[0] || c[1])) return;
            int k = c[0] && c[1] ? c[0]->dist(p) > c[1]->dist(p) : (c[0] ? 0 : 1);
            c[k]->query(p, res);
            if (c[k ^ 1] && c[k ^ 1]->dist(p) < res) c[k ^ 1]->query(p, res);
        }
    } *root, _pool[MAXN << 1], *_cur;
    KDTree() : root(NULL), res(0) {
        _cur = _pool;
    }
    Node *build(int l, int r, Point P[], int d = 0) {
        if (l > r) return NULL;
        if (l == r) return new (_cur++) Node(P[l]);
        int mid = l + (r - l) / 2;
        d ? std::nth_element(P + l, P + mid, P + r + 1, cmp1) : std::nth_element(P + l, P + mid, P + r + 1, cmp2);
        Node *u = new (_cur++) Node(P[mid]);
        u->c[0] = build(l, mid - 1, P, d ^ 1);
        u->c[1] = build(mid + 1, r, P, d ^ 1);
        u->maintain();
        return u;
    }
    void insert(const Point &p) {
        Node **u = &root;
        int d = 0;
        while (*u) {
            int k = (d ? cmp1(p, (*u)->p) : cmp2(p, (*u)->p)) ^ 1;
            d ^= 1;
            (*u)->r1.x = std::min(p.x, (*u)->r1.x);
            (*u)->r1.y = std::min(p.y, (*u)->r1.y);
            (*u)->r2.x = std::max(p.x, (*u)->r2.x);
            (*u)->r2.y = std::max(p.y, (*u)->r2.y);
            u = &(*u)->c[k];
        }
        *u = new (_cur++) Node(p);
    }
    int query(const Point &p) {
        res = INT_MAX;
        root->query(p, res);
        return res;
    }
} kdT;
int main() {
    int n, m;
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= n; i++) scanf("%d %d", &P[i].x, &P[i].y);
    kdT.root = kdT.build(1, n, P);
    while (m--) {
        int op;
        Point p;
        scanf("%d %d %d", &op, &p.x, &p.y);
#ifdef DBG
        printf("op = %d, p: (%d, %d)\n", op, p.x, p.y);
#endif
        if (op == 1) kdT.insert(p);
        else printf("%d\n", kdT.query(p));
    }
    return 0;
}
```