---
title: '[2016 Multi-University Training Contest 8] I Rikka with Subset'
date: 2018-12-31 15:41:36
tags: [多校, FFT, 数学, 卷积]
categories: 题解（OI/XCPC）
---

## 题目大意

给出一个长为 $n$ 的整数列 $\{a_i\}$ 。对 $1 \sim n$ 中的每一个数 $k$ ，定义其一个子集 $S$ 关于 $k$ 的值为 $S$ 中前 $\min\{|S|, k\}$ 大的数之和；定义一个关于该数列和 $k$ 的函数，其值为该数列所有非空子集关于 $k$ 的值之和。对每一个 $k$，求出其函数值，答案对 $998244353$ 取模。$T$ 组数据。

$1 \leq T \leq 10$

$1 \leq n \leq 100,000$

$0 \leq a_i \leq 10^9$

## 题目链接

[HDU 5829](http://acm.hdu.edu.cn/showproblem.php?pid=5829)

<!-- more -->

## 题解

对数列按增序排序。发现计算函数值的逐差更容易，记为 $s_k$ ，表示数列所有大小至少为 $k$ 的非空子集中第 $k$ 大的数之和。考虑每个数在多少个子集中可以作为第 $k$ 大的数，有：（数列下标从 $0$ 开始）
$$
\begin{align}
s_k &= \sum_{i = 0}^{n - k} a_i 2^i \binom{n - i - 1}{k - 1} \\
&= \frac{1}{(k - 1)!} \sum_{i = 0}^{n - k} \frac{a_i 2^i (n - i - 1)!}{(n - i - k)!}
\end{align}
$$
发现计算 $s_k$ 时和式有 $n - k + 1$ 项，考虑将 $s$ 倒序得到 $\{s'_k\}$ ，并试图推出卷积形式：
$$
s'_k = \frac{1}{n - k - 1} \sum_{i = 0}^{k} a_i 2^i (n - i - 1)! \times \frac{1}{(k - i)!}
$$
取序列 $f$、$g$：
$$
f_i = a_i 2^i (n - i - 1)! \\
g_i = \frac{1}{i!}
$$
它们做卷积可得到 $\{(n - k - 1)s'_k\}$ ，继而得到答案。

## 代码

行末必须有空格，否则会 PE。

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 100005;
const int MAXN_EXTEND = 262144;
const int MOD = 998244353;
const int G = 3;

long long qpow(long long a, long long n) {
    long long res = 1;
    for (; n; n >>= 1, a = a * a % MOD) if (n & 1) res = res * a % MOD;
    return res;
}

long long inv(long long x) {
    return qpow(x, MOD - 2);
}

long long fact[MAXN], invFact[MAXN], pow2[MAXN];

void init() {
    fact[0] = 1;
    for (int i = 1; i < MAXN; i++) fact[i] = fact[i - 1] * i % MOD;
    invFact[MAXN - 1] = inv(fact[MAXN - 1]);
    for (int i = MAXN - 2; ~i; i--) invFact[i] = invFact[i + 1] * (i + 1) % MOD;

    pow2[0] = 1;
    for (int i = 1; i < MAXN; i++) {
        pow2[i] = pow2[i - 1] << 1ll;
        pow2[i] >= MOD ? pow2[i] -= MOD : 0;
    }
}

namespace NTT {
    static const int N = ::MAXN_EXTEND;

    long long omega[N], omegaInv[N];

    void init() {
        long long g = qpow(G, (MOD - 1) / N), ig = inv(g);
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
        for (int i = 0, j = 0; i < n; i++) {
            if (i < j) std::swap(a[i], a[j]);
            for (int l = n >> 1; (j ^= l) < l; l >>= 1) {}
        }
    }

    void transform(long long *a, int n, long long *omega) {
        reverse(a, n);

        for (int l = 2; l <= n; l <<= 1) {
            int hl = l >> 1;
            for (long long *x = a; x != a + n; x += l) {
                for (int i = 0; i < hl; i++) {
                    long long t = omega[N / l * i] * x[i + hl] % MOD;
                    x[i + hl] = (x[i] - t + MOD) % MOD;
                    x[i] += t;
                    x[i] >= MOD ? x[i] -= MOD : 0;
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
        for (int i = 0; i < n; i++) a[i] = a[i] * t % MOD;
    }
}

int a[MAXN];
long long f[MAXN_EXTEND], g[MAXN_EXTEND], s[MAXN];

int main() {
    init();
    NTT::init();

    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);
        for (int i = 0; i < n; i++) scanf("%d", &a[i]);
        std::sort(a, a + n);

        for (int i = 0; i < n; i++) {
            f[i] = a[i] * fact[n - i - 1] % MOD * pow2[i] % MOD;
            g[i] = invFact[i];
        }

        int N = NTT::extend(n << 1);
        std::fill(f + n, f + N, 0);
        std::fill(g + n, g + N, 0);

        NTT::dft(f, N);
        NTT::dft(g, N);
        for (int i = 0; i < N; i++) f[i] = f[i] * g[i] % MOD;
        NTT::idft(f, N);

        for (int i = 0; i < n; i++) s[n - i - 1] = f[i] * invFact[n - i - 1] % MOD;
        for (int i = 1; i < n; i++) {
            s[i] += s[i - 1];
            s[i] >= MOD ? s[i] -= MOD : 0;
        }
        for (int i = 0; i < n; i++) printf("%lld ", s[i]);
        puts("");
    }
    
    return 0;
}
```