---
title: '[SCOI 2003] 严格n元树'
date: 2017-05-04 21:43:26
tags: [DP, 高精度]
categories: 题解（OI/XCPC）
---

## 题目大意

如果一棵树的所有非叶节点都恰好有 $n$ 个儿子，那么我们称它为严格 $n$ 元树。如果该树中最底层的节点深度为 $d$（根的深度为 $0$），那么我们称它为一棵深度为 $d$ 的严格 $n$ 元树。给定 $n$、$d$，求不同的深度为 $d$ 的严格 $n$ 元树有多少个。

$1 \leqslant n \leqslant 32$

$1 \leqslant d \leqslant 16$

## 题目链接

[【SCOI 2003】严格 N 元树 - Luogu 4295](https://www.luogu.com.cn/problem/P4295)

<!-- more -->

## 题解

DP + 高精度。

定义 $f(d)$ 为答案，$S(d)$ 为 $f(d)$ 的前缀和，表示深度不大于 $d$ 的严格 $n$ 元树的种数。考虑其 $n$ 个子节点，每个都有 $S(d - 1)$ 种可能，再加上只有一个根节点，那么有：
$$
\begin{align}
S(d) &= S(d - 1)^n + 1 \\
ans = f(d) &= S(d) - S(d - 1)
\end{align}
$$

## 代码

```c++
#include <cstdio>
#include <vector>
#include <algorithm>
const int MAXD = 20;
struct BigInt {
    std::vector<char> v;
    BigInt(int x = 0) {
        *this = x;
    }
    BigInt &operator=(int x) {
        v.clear();
        do v.push_back(x % 10); while (x /= 10);
        return *this;
    }
    BigInt &operator=(const BigInt &x) {
        v.resize(x.v.size());
        for (int i = 0; i < x.v.size(); i++) v[i] = x.v[i];
        return *this;
    }
    void print() const {
        for (int i = v.size() - 1; ~i; i--) putchar(v[i] + '0');
    }
} S[MAXD];
BigInt operator+(const BigInt &a, const BigInt &b) {
    BigInt res;
    res.v.clear();
    bool flag = false;
    for (int i = 0; i < std::max(a.v.size(), b.v.size()); i++) {
        int temp = 0;
        if (i < a.v.size()) temp += a.v[i];
        if (i < b.v.size()) temp += b.v[i];
        if (flag) temp++, flag = false;
        if (temp >= 10) flag = true, temp -= 10;
        res.v.push_back(temp);
    }
    if (flag) res.v.push_back(1);
    return res;
}
BigInt &operator+=(BigInt &a, const BigInt &b) {
    return a = a + b;
}
BigInt &operator++(BigInt &a) {
        return a += 1;
}
BigInt operator-(const BigInt &a, const BigInt &b) {
    BigInt res;
    res.v.clear();
    bool flag = false;
    for (int i = 0; i < std::max(a.v.size(), b.v.size()); i++) {
        int temp = a.v[i];
        if (i < b.v.size()) temp -= b.v[i];
        if (flag) temp--, flag = false;
        if (temp < 0) flag = true, temp += 10;
        res.v.push_back(temp);
    }
    int size = res.v.size();
    while (size > 1 && res.v[size - 1] == 0) size--;
    res.v.resize(size);
    return res;
}
BigInt operator*(const BigInt &a, const BigInt &b) {
    BigInt res;
    res.v.resize(a.v.size() + b.v.size());
    for (int i = 0; i < a.v.size(); i++) for (int j = 0; j < b.v.size(); j++) {
        res.v[i + j] += a.v[i] * b.v[j];
        res.v[i + j + 1] += res.v[i + j] / 10;
        res.v[i + j] %= 10;
    }
    int size = res.v.size();
    while (size > 1 && res.v[size - 1] == 0) size--;
    res.v.resize(size);
    return res;
}
BigInt &operator*=(BigInt &a, const BigInt &b) {
    return a = a * b;
}
BigInt pow(const BigInt &a, int n) {
    BigInt res(1);
    for (BigInt x = a; n; n >>= 1, x *= x) if (n & 1) res *= x;
    return res;
}
int main() {
    int n, d;
    scanf("%d %d", &n, &d);
    if (d == 0) {
        puts("1");
        return 0;
    }
    S[0] = 1;
    for (int i = 1; i <= d; i++) {
        S[i] = pow(S[i - 1], n);
        ++S[i];
    }
    (S[d] - S[d - 1]).print();
    puts("");
    return 0;
}
```