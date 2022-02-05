---
title: '[Codeforces Good Bye 2018] E New Year and the Acquaintance Estimation'
date: 2018-12-31 11:28:17
tags: [线段树, 二分, 线段树上二分, Havel-Hakimi]
categories: 题解（OI/XCPC）
---

## 题目大意

已知一张 $n + 1$ 个点的无向图中 $n$ 个点的度数，求另一个点可能的度数。

$1 \leq n \leq 500,000$

## 题目链接

[Codeforces Good Bye 2018 - E](https://codeforces.com/contest/1091/problem/E)

<!-- more -->

## 题解

如果已知所有点的度数，可以通过以下方法构造出一张图，或判断出无解：

1. 按降序排序度数
2. 取出第一个点，记其度数为 $deg$ ，并向之后的 $deg$ 个点连边，它们的度数减一
3. 反复以上步骤直至剩余的度数全为 $0$，或发现做不到

该算法称为 [Havel–Hakimi algorithm](https://en.wikipedia.org/wiki/Havel%E2%80%93Hakimi_algorithm) ，并且原题已经友好地给了我们 wiki 的链接。（比赛时猜到了这种构造，点开链接后才确认了正确性）

如果未知点（记作 $u_0$ ）的度数为 $deg$ 时有解，考虑图中的一条边 $(u, v)$ ，且不存在边 $(u_0, u)$、$(u_0, v)$，那么可以找到 $u_0$ 度数为 $deg + 2$ 时的一个解。感受一下，可能的度数一定是一段连续的奇/偶数，那么我们只要找到上界与下界即可。

仔细思考/手动模拟之后发现，如果一个点的度数为某个值时有解，让该度数加上或减去 $1$，Havel-Hakimi 最终得到的序列会有一个 $\pm 1$ （其位置可能会在 $n$ 以后）。可以考虑以下算法：

先让 $u_0$ 与其他点不连边，对已知的 $n$ 个点跑 Havel-Hakimi，最后的序列的绝对值之和为下界；让 $u_0$ 与其他点都连边（相当于其他点的度数均减 $1$ ），对这 $n$ 个点跑 Havel-Hakimi，最后的序列的绝对值之和为上界。

现在考虑如何高效地跑 Havel-Hakimi。操作可以描述为区间减 $1$ 和整体排序。由于区间修改操作前序列有序，而每次操作只会让数减 $1$，整体排序操作可以高效完成。具体地说，记操作区间右端点为 $r$、序列为 $\{d_i\}$，则在修改操作前 $d_r = d_{r + 1}$ 时才会需要重新调整顺序。需要调整时，可以找到连续的一段值为 $d_r$ 的区间 $[L, R] ~ (r, r + 1\in [L, R])$ ，在修改与排序操作后，$[L, R]$ 的前 $R - r$ 个数不变，后 $r - L + 1$ 个数减 $1$。

具体实现上，可以用线段树维护序列，查找相等的区间时直接二分，时间复杂度是 $O(n \log^2n)$ ，依靠线段树的二分结构可以做到 $O(n \log n)$。

## 代码

```c++
#include <bits/stdc++.h>

const int MAXN = 500005;

struct SegT {
    struct Node {
        Node *lc, *rc;
        int l, r;
        long long min, tag;

        Node() {}
        Node(int pos, int val) : l(pos), r(pos), min(val), tag(0), lc(NULL), rc(NULL) {}
        Node(Node *lc, Node *rc) : l(lc->l), r(rc->r), lc(lc), rc(rc), tag(0) {
            maintain();
        }

        void add(long long d) {
            min += d;
            tag += d;
        }

        void pushDown() {
            if (tag) {
                lc->add(tag);
                rc->add(tag);
                tag = 0;
            }
        }

        void maintain() {
            min = std::min(lc->min, rc->min);
        }

        void update(int l, int r, int d) {
            if (r < this->l || this->r < l) return;
            if (l <= this->l && this->r <= r) {
                add(d);
                return;
            }
            pushDown();
            lc->update(l, r, d);
            rc->update(l, r, d);
            maintain();
        }

        long long query(int l, int r) {
            if (r < this->l || this->r < l) return LLONG_MAX;
            if (l <= this->l && this->r <= r) return min;
            pushDown();
            return std::min(lc->query(l, r), rc->query(l, r));
        }

        int bisearch(int l, int r, int val) {
            if (this->l == this->r) return this->l;
            int mid = this->l + ((this->r - this->l) >> 1);
            pushDown();
            if (r <= mid) return lc->bisearch(l, r, val);
            if (l > mid) return rc->bisearch(l, r, val);
            if (lc->min > val) return rc->bisearch(l, r, val);
            else return lc->bisearch(l, r, val);
        }
    } *root, _pool[MAXN << 1], *_curr;

    SegT() : root(NULL), _curr(_pool) {}

    Node *_build(int l, int r, int *a) {
        if (l == r) return new (_curr++) Node(l, a[l]);
        int mid = l + ((r - l) >> 1);
        return new (_curr++) Node(_build(l, mid, a), _build(mid + 1, r, a));
    }
    void build(int l, int r, int *a) {
        _curr = _pool;
        root = _build(l, r, a);
    }

    void update(int l, int r, int d) {
        root->update(l, r, d);
    }

    long long query(int l, int r) {
        return root->query(l, r);
    }

    int bisearch(int l, int r, int val) {
        if (root->query(l, r) > val) return r + 1;
        return root->bisearch(l, r, val);
    }
} segT;

int d[MAXN];

int main() {
    int n;
    scanf("%d", &n);
    for (int i = 1; i <= n; i++) scanf("%d", &d[i]);

    std::sort(d + 1, d + n + 1, std::greater<int>());

    segT.build(1, n, d);
    int L = 0, i;
    for (i = 1; i <= n; i++) {
        int t = segT.query(i, i);
        if (t <= 0) break;

        L += std::max(i + t - n, 0);

        if (i + t + 1 <= n) {
            int a = segT.query(i + t, i + t);
            int b = segT.query(i + t + 1, i + t + 1);
            if (a == b) {
                int pos1 = segT.bisearch(i + 1, i + t, a);
                int pos2 = segT.bisearch(i + t + 1, n, b - 1) - 1;

                int cnt = i + t + 1 - pos1;
                segT.update(pos1, pos1 + cnt - 1, 1);
                segT.update(pos2 - cnt + 1, pos2, -1);
            }
        }
        segT.update(i + 1, i + t, -1);
    }
    for (; i <= n; i++) L += std::abs(segT.query(i, i));

    for (int i = 1; i <= n; i++) --d[i];
    segT.build(1, n, d);
    int R = n;
    for (i = 1; i <= n; i++) {
        int t = segT.query(i, i);
        if (t <= 0) break;

        R -= std::max(i + t - n, 0);

        if (i + t + 1 <= n) {
            int a = segT.query(i + t, i + t);
            int b = segT.query(i + t + 1, i + t + 1);
            if (a == b) {
                int pos1 = segT.bisearch(i + 1, i + t, a);
                int pos2 = segT.bisearch(i + t + 1, n, b - 1) - 1;

                int cnt = i + t + 1 - pos1;
                segT.update(pos1, pos1 + cnt - 1, 1);
                segT.update(pos2 - cnt + 1, pos2, -1);
            }
        }
        segT.update(i + 1, i + t, -1);
    }
    for (; i <= n; i++) R -= std::abs(segT.query(i, i));

    if (L > R) printf("-1");
    for (int i = L; i <= R; i += 2) printf("%d ", i);
    puts("");
    
    return 0;
}
```