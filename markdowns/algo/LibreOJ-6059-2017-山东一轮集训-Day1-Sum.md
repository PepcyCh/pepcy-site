---
title: '[LibreOJ 6059][2017 山东一轮集训 Day1] Sum'
date: 2017-06-30 21:21:09
tags: [DP, FFT]
categories: 题解（OI/XCPC）
---

## 题目大意

求有多少 $n$ 位十进制数是 $p$ 的倍数且每位之和小于等于 $m_i (m_i = 0, 1, 2, \ldots, m - 1, m)$，允许前导 $0$，答案对 $998244353$ 取模。

$1 \leqslant n \leqslant 1,000,000,000$

$1 \leqslant q \leqslant 50$

$1 \leqslant m \leqslant 1,000$

## 题目链接

[LibreOJ 6059](https://loj.ac/problem/6059)

<!-- more -->

## 题解

考虑 DP，记 $f(i, j, k)$ 表示已经考虑了 $i$ 位、已经考虑的位数组成的数模 $p$ 的余数为 $j$、已经考虑的位数的数字和为 $k$ 时的答案，则转移：
$$
f(i, j, k) \rightarrow f(i + 1, (j \times 10 + c) \bmod p, k + c) \quad c = 0, 1, 2 \dots 9
$$
这样显然会 TLE。

转移可写为：
$$
f(i, (j \times 10 + c) \bmod p, k) = \sum_{t = 0}^{k} f(i - 1, j, t) \times f(1, c, k - t)
$$
发现是卷积的形式。同时发现 DP 可以倍增。于是 FFT／NTT + 倍增 + DP 的解就出来了。

## 代码

```c++
#include <cstdio>
#include <algorithm>
const int MAXP = 50;
const int MAXM = 1000;
const int MAXM_EXTEND = 2048;
const int MOD = 998244353;
const int G = 3;
void exgcd(long long a, long long b, long long &x, long long &y) {
    if (!b) x = 1, y = 0;
    else exgcd(b, a % b, y, x), y -= x * (a / b);
}
long long inv(long long x) {
    long long res, temp;
    exgcd(x, MOD, res, temp);
    return (res + MOD) % MOD;
}
long long pow(long long a, long long n) {
    long long res = 1;
    for (; n; n >>= 1, a = a * a % MOD) if (n & 1) res = res * a % MOD;
    return res;
}
namespace NumberTheoreticTransform {
    static const int N = 2048;
    long long omega[N], omegaInv[N];
    void init() {
        long long g = pow(G, (MOD - 1) / N), ig = inv(g);
        omega[0] = omegaInv[0] = 1;
        for (int i = 1; i < N; i++) {
            omega[i] = omega[i - 1] * g % MOD;
            omegaInv[i] = omegaInv[i - 1] * ig % MOD;
        }
    }
    int extend(int n) {
        int res = 1;
        while (res < n) res <<= 1;
        return res;
    }
    void reverse(long long *a, int n) {
        int k = 0;
        while ((1 << k) < n) k++;
        for (int i = 0, j = 0; i < n; i++) {
            if (i < j) std::swap(a[i], a[j]);
            for (int l = n >> 1; (j ^= l) < l; l >>= 1);
        }
    }
    void transform(long long *a, int n, long long *omega) {
        reverse(a, n);
        for (int l = 2; l <= n; l <<= 1) {
            int hl = l >> 1;
            for (long long *x = a; x != a + n; x += l) {
                for (int i = 0; i < hl; i++) {
                    long long t = omega[N / l * i] * x[i + hl] % MOD;
                    x[hl + i] = (x[i] - t + MOD) % MOD;
                    (x[i] += t) %= MOD;
                }
            }
        }
    }
    void dft(long long *a, int n) {
        transform(a, n, omega);
    }
    void idft(long long *a, int n) {
        transform(a, n, omegaInv);
        long long t = inv(n);
        for (int i = 0; i < n; i++) (a[i] *= t) %= MOD;
    }
}
int p, m;
struct Data {
    long long pow, a[MAXP][MAXM_EXTEND];
    Data() : pow(1), a() {}
};
Data operator*(Data &a, Data &b) {
    Data res;
    res.pow = a.pow * b.pow % p;
    int s = NumberTheoreticTransform::extend(m + 1) * 2;
    for (int i = 0; i < p; i++) {
        NumberTheoreticTransform::dft(a.a[i], s);
        if (&a != &b) NumberTheoreticTransform::dft(b.a[i], s);
    }
    for (int i1 = 0; i1 < p; i1++) for (int i2 = 0; i2 < p; i2++) {
        static long long temp[MAXM_EXTEND];
        for (int i = 0; i < s; i++) temp[i] = a.a[i1][i] * b.a[i2][i];
        NumberTheoreticTransform::idft(temp, s);
        for (int i = 0; i <= m; i++) (res.a[(i1 * b.pow + i2) % p][i] += temp[i]) %= MOD;
    }
    for (int i = 0; i < p; i++) {
        NumberTheoreticTransform::idft(a.a[i], s);
        if (&a != &b) NumberTheoreticTransform::idft(b.a[i], s);
    }
    return res;
}
Data pow(Data a, int n) {
    Data res;
    res.a[0][0] = 1;
    for (; n; n >>= 1, a = a * a) if (n & 1) res = res * a;
    return res;
}
int main() {
    NumberTheoreticTransform::init();
    int n;
    scanf("%d %d %d", &n, &p, &m);
    Data init;
    init.pow = 10;
    for (int i = 0; i <= std::min(9, m); i++) init.a[i % p][i]++;
    Data res = pow(init, n);
    long long ans = 0;
    for (int i = 0; i <= m; i++) {
        (ans += res.a[0][i]) %= MOD;
        printf("%lld%c", ans, " \n"[i == m]);
    }
    return 0;
}
```