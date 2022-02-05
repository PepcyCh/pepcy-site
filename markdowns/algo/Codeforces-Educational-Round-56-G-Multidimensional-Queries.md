---
title: '[Codeforces Educational Round 56] G Multidimensional Queries'
date: 2018-12-24 11:57:40
tags: [线段树]
categories: 题解（OI/XCPC）
---

## 题目大意

$\{a_i\}$ 是 $k$ 维空间内的 $n$ 个点，有 $q$ 次操作，每次操作为修改一个点的坐标，或查询下标区间在 $[l, r]$ 内的点的最大两点间曼哈顿距离。

$1 \leq n, q \leq 200,000$

$1 \leq k \leq 5$

$-10^6 \leq a_{i, j} \leq 10^6$

## 题目链接

[Codeforces Educational Round 56 - G](https://codeforces.com/contest/1093/problem/G)

<!--more-->

## 题解

$j$、$k$ 两点间的曼哈顿距离可表示为：
$$
\sum_{i = 1}^{k} c_i a_{j, i} - \sum_{i = 1}^{k} c_i a_{k, i}
$$
其中 $c_i$ 为一个 $\{1, -1\}$ 的系数，且可以直接证明，对任何 $c_i$，曼哈顿距离为其中最大的一个。由于 $k$ 只有 $5$，可以考虑每举所有 $2^k$ 种 $c_i$，询问即区间内求 $\sum_{i = 1}^{k} c_i a_{j, i}$ 的最值，修改即单点修改，用 $2^k$ 棵线段树可以做到 $O((n + q) 2^k \log n)$。由于每次操作的点或区间对所有线段树均相同，故可用一棵线段树维护，<del>复杂度降为 O((n + q)2^k + q \log n)​ </del>，复杂度不会改变，但常数会变小很多，因为这样对内存访问更加友好；同时，由于 $c_i = c$ 时的最大值就是 $c_i = 2^k - 1 - c$ 时的最小值，故只用维护一种最值。

吐槽：virtual 的时候感觉大家都会 G，而自己却一直在想什么可持久化 5-d Tree。。。QAQ

## 代码

```c++
#include <cstdio>
#include <climits>
#include <algorithm>

const int MAXN = 200005;
const int MAXK = 5;

int K;

struct Point {
    int x[MAXK];
} P[MAXN], b;

int calc(const Point &p, int c) {
    int res = 0;
    for (int i = 0; i < K; i++) (c & (1 << i)) ? res += p.x[i] : res -= p.x[i];
    return res;
}

struct SegT {
    struct Node {
        Node *lc, *rc;
        int l, r, max[1 << MAXK];

        Node() {}
        Node(int pos) : l(pos), r(pos), lc(NULL), rc(NULL) {}
        Node(Node *lc, Node *rc) : l(lc->l), r(rc->r), lc(lc), rc(rc) {
            maintain();
        }

        void maintain() {
            for (int c = 0; c < 1 << K; c++) max[c] = std::max(lc->max[c], rc->max[c]);
        }

        void update(int pos, const Point &d) {
            if (l == r) {
                for (int c = 0; c < 1 << K; c++) max[c] = calc(d, c);
                return;
            }

            int mid = l + ((r - l) >> 1);
            if (pos <= mid) lc->update(pos, d);
            else rc->update(pos, d);
            maintain();
        }

        void query(int l, int r, int *ret) {
            if (r < this->l || this->r < l) return;
            if (l <= this->l && this->r <= r) {
                for (int c = 0; c < 1 << K; c++)
                    ret[c] = std::max(ret[c], max[c]);
                return;
            }
            lc->query(l, r, ret);
            rc->query(l, r, ret);
        }
    } *root, _pool[MAXN << 1], *_curr;

    SegT() : root(NULL), _curr(_pool) {}

    Node *build(int l, int r, Point *a) {
        if (l == r) {
            Node *u = new (_curr++) Node(l);
            for (int c = 0; c < 1 << K; c++) u->max[c] = calc(a[l], c);
            return u;
        }
        int mid = l + ((r - l) >> 1);
        return new (_curr++) Node(build(l, mid, a), build(mid + 1, r, a));
    }
    void build(Point *a, int n) {
        root = build(1, n, a);
    }

    void update(int pos, const Point &d) {
        root->update(pos, d);
    }

    void query(int l, int r, int *ret) {
        return root->query(l, r, ret);
    }
} segT;

int main() {
    int n;
    scanf("%d %d", &n, &K);

    for (int i = 1; i <= n; i++) for (int j = 0; j < K; j++) scanf("%d", &P[i].x[j]);
    segT.build(P, n);

    int q;
    scanf("%d", &q);
    while (q--) {
        int op;
        scanf("%d", &op);

        if (op == 1) {
            int pos;
            scanf("%d", &pos);
            for (int i = 0; i < K; i++) scanf("%d", &b.x[i]);
            segT.update(pos, b);
        } else {
            int l, r;
            scanf("%d %d", &l, &r);

            static int ret[1 << MAXK];
            std::fill(ret, ret + (1 << K), INT_MIN);
            segT.query(l, r, ret);
            int ans = 0;
            for (int c = 0; c < 1 << (K - 1); c++)
                ans = std::max(ans, std::abs(ret[c] + ret[(1 << K) - 1 - c]));
            printf("%d\n", ans);
        }
    }
    
    return 0;
}
```