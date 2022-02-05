---
title: '[Codeforces Educational Round 56] E Intersection of Permutations'
date: 2018-12-25 11:14:19
tags: [树状数组, 二维树状数组]
categories: 题解（OI/XCPC）
---

## 题目大意

两个长为 $n$ 的 $1 \sim n$ 的排列 $\{a_i\}$、$\{b_i\}$。有 $m$ 次操作，每次操作询问在 $\{a_i\}$ 的 $[l_a, r_a]$ 区间内和 $\{b_i\}$ 的 $[l_b, r_b]$ 区间内有多少个相同的数，或交换 $\{b_i\}$ 中的两个数。

$1 \leq n, m \leq 200,000$

## 题目链接

[Codeforces Educational Round 56 - E](https://codeforces.com/contest/1093/problem/E)

<!--more-->

## 题解

从两个排列中，我们可以得到一个数组 $\{pos_i\}$ （代码中为 `b[]`）表示 $\{b_i\}$ 中第 $i$ 个数在 $\{a_i\}$ 中的位置，则可以视该数组为一组平面上的点，每次询问转换为 $i \in [l_a, r_a], pos_i \in [l_b, r_b]$ 这一矩阵内点的个数。解决这个问题，我们可以使用二维树状数组，但直接写会 MLE，而使用 `std::map` 会 TLE。考虑离散化，我们第一遍先得到参与运算的坐标，之后就可以离散了。

## 代码

```c++
#include <cstdio>
#include <vector>
#include <algorithm>

const int MAXN = 200005;

int a[MAXN], b[MAXN], temp[MAXN], n;

constexpr int lowbit(int x) {
    return x & -x;
}

struct BIT_PRE {
    std::vector<int> a[MAXN];

    void update(int x, int y) {
        for (int i = x; i <= n; i += lowbit(i)) a[i].push_back(y);
    }

    void query(int x, int y) {
        for (int i = x; i; i -= lowbit(i)) a[i].push_back(y);
    }

    void query(int x0, int y0, int x1, int y1) {
        query(x1, y1);
        query(x0 - 1, y0 - 1);
        query(x1, y0 - 1);
        query(x0 - 1, y1);
    }

    void calc() {
        for (int i = 1; i <= n; i++) {
            std::sort(a[i].begin(), a[i].end());
            a[i].resize(std::unique(a[i].begin(), a[i].end()) - a[i].begin());
        }
    }
};

int getIndex(const std::vector<int> &vec, int a) {
    return std::lower_bound(vec.begin(), vec.end(), a) - vec.begin() + 1;
}

struct BIT {
    std::vector<int> a[MAXN];
    BIT_PRE pre;

    void init() {
        pre.calc();
        for (int i = 1; i <= n; i++) a[i].resize(pre.a[i].size() + 1, 0);
    }

    void update(int x, int y, int d) {
        for (int i = x; i <= n; i += lowbit(i))
            for (int j = getIndex(pre.a[i], y); j < (int) a[i].size(); j += lowbit(j)) a[i][j] += d;
    }

    int query(int x, int y) {
        int res = 0;
        for (int i = x; i; i -= lowbit(i))
            for (int j = getIndex(pre.a[i], y); j; j -= lowbit(j)) res += a[i][j];
        return res;
    }

    int query(int x0, int y0, int x1, int y1) {
        return query(x1, y1) + query(x0 - 1, y0 - 1) - query(x1, y0 - 1) - query(x0 - 1, y1);
    }
} bit;

struct Operation {
    int op, a, b, c, d;

    Operation() {}
    Operation(int op, int a, int b, int c, int d) : op(op), a(a), b(b), c(c), d(d) {}
    Operation(int op, int a, int b) : op(op), a(a), b(b) {}
} O[MAXN];

int main() {
    int m;
    scanf("%d %d", &n, &m);

    for (int i = 1, x; i <= n; i++) {
        scanf("%d", &x);
        a[x] = i;
    }

    for (int i = 1, x; i <= n; i++) {
        scanf("%d", &x);
        b[i] = a[x];
        bit.pre.update(i, b[i]);
    }

    std::copy(b + 1, b + n + 1, temp);
    for (int i = 0; i < m; i++) {
        int op;
        scanf("%d", &op);

        if (op == 1) {
            int la, ra, lb, rb;
            scanf("%d %d %d %d", &la, &ra, &lb, &rb);

            bit.pre.query(lb, la, rb, ra);

            O[i] = Operation(op, lb, la, rb, ra);
        } else {
            int x, y;
            scanf("%d %d", &x, &y);

            bit.pre.update(x, b[x]);
            bit.pre.update(y, b[y]);
            std::swap(b[x], b[y]);
            bit.pre.update(x, b[x]);
            bit.pre.update(y, b[y]);

            O[i] = Operation(op, x, y);
        }
    }

    std::copy(temp, temp + n, b + 1);
    bit.init();

    for (int i = 1; i <= n; i++) bit.update(i, b[i], 1);
    for (int i = 0; i < m; i++) {
        int op = O[i].op;

        if (op == 1) {
            int lb = O[i].a, la = O[i].b, rb = O[i].c, ra = O[i].d;
            int ans = bit.query(lb, la, rb, ra);
            printf("%d\n", ans);
        } else {
            int x = O[i].a, y = O[i].b;
            bit.update(x, b[x], -1);
            bit.update(y, b[y], -1);
            std::swap(b[x], b[y]);
            bit.update(x, b[x], 1);
            bit.update(y, b[y], 1);
        }
    }
    
    return 0;
}
```