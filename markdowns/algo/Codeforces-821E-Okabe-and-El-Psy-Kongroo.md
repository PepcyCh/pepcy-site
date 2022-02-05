---
title: '[Codeforces 821E] Okabe and El Psy Kongroo'
date: 2017-06-30 22:05:08
tags: [DP, 矩阵乘法]
categories: 题解（OI/XCPC）
---

## 题目大意

在平面直角坐标系中，从 $(0, 0)$ 走到 $(k, 0)$ 。要求：

* 每时每刻满足 $x, y \geqslant 0$。
* 每次只能从 $(x, y)$ 走到 $(x + 1, y + 1)$、$(x + 1, y)$、$(x + 1, y - 1)$。
* 有 $n$ 条水平线段，用 $(a_i, b_i, c_i)$ 表示线段 $y = c_i, (a_i \leqslant x \leqslant b_i)$ ，满足 $b_{i} = a_{i + 1}$、$a_1 = 0$。要求，对于任意 $a_i \leqslant x \leqslant b_i$ 有 $y \leqslant c_i$。

求方案数，答案对 $1,000,000,007$ 取模。

$1 \leqslant n \leqslant 100$

$1 \leqslant k \leqslant 1 \times 10^{18}$

$0 \leqslant a_i < b_i \leqslant 1 \times 10^{18} \quad 0 \leqslant c_i \leqslant 15$

## 题目链接

[Codeforces 821E](http://codeforces.com/problemset/problem/821/E)

<!-- more -->

## 题解

我本来一直没有写 CF 的题解，但这道题，犯了一个小错误导致 $-9$ 。。。

DP + 矩乘。

记 $f(x, y)$ 表示走到 $(x, y)$ 的答案，转移为：
$$
f(x, y) = f(x - 1, y - 1) + f(x - 1, y) + f(x - 1, y + 1)
$$
注意到 $c$ 很小、横坐标们很大，考虑矩乘优化（这个矩乘还是很好想的，相比某 KMP + 矩乘的题），完毕。

然后，当时我干了个什么呢？把矩阵快速幂的指数设成了 `int`  ，一直没发现，于是就一直 TLE。。。 233333

## 代码

```c++
#include <cstdio>
#include <cstring>
#include <algorithm>
template <typename T>
void read(T &x) {
    char ch;
    while ((ch = getchar()) > '9' || ch < '0');
    x = ch - '0';
    while ((ch = getchar()) >= '0' && ch <= '9') x = (x << 1) + (x << 3) + ch - '0';
}
const int MAXC = 16;
const int MAXN = 100;
const int MOD = 1000000007;
struct Matrix {
    long long a[MAXC][MAXC];
    Matrix(bool init = false) {
        memset(a, 0, sizeof (a));
        if (init) for (int i = 0; i < MAXC; i++) a[i][i] = 1;
    }
    friend Matrix operator*(const Matrix &a, const Matrix &b) {
        Matrix res(false);
        for (int i = 0; i < MAXC; i++) for (int k = 0; k < MAXC; k++) for (int j = 0; j < MAXC; j++)
            (res(i, j) += a(i, k) * b(k, j) % MOD) %= MOD;
        return res;
    }
    long long &operator()(int i, int j) {
        return a[i][j];
    }
    long long operator()(int i, int j) const {
        return a[i][j];
    }
};
Matrix pow(Matrix a, long long n) {
    Matrix res(true);
    for (; n; n >>= 1, a = a * a) if (n & 1) res = res * a;
    return res;
}
int main() {
    int n;
    long long k;
    scanf("%d %lld", &n, &k);
    Matrix init(false);
    init(0, 0) = 1;
    for (int i = 0; i < n; i++) {
        long long a, b;
        int c;
        scanf("%lld %lld %d", &a, &b, &c);
        if (a >= k) break;
        for (int j = c + 1; j < MAXC; j++) init(j, 0) = 0;
        if (c == 0) continue;
        Matrix trans(false);
        for (int j = 0; j <= c; j++) {
            trans(j, j) = 1;
            if (j) trans(j, j - 1) = 1;
            if (j < c) trans(j, j + 1) = 1;
        }
        Matrix temp = pow(trans, std::min(b, k) - a);
        init = temp * init;
    }
    printf("%lld\n", init(0, 0));
    return 0;
}
```