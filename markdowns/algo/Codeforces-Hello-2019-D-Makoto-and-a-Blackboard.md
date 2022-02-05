---
title: '[Codeforces Hello 2019] D Makoto and a Blackboard'
date: 2019-01-05 10:59:11
tags: [积性函数]
categories: 题解（OI/XCPC）
---

## 题目大意

给出一个数 $n$，每次它会等概率地变为它的一个约数，求 $k$ 次操作后该数的期望，对 $1,000,000,007$ 取模。

$1 \leq n \leq 10^{15}$

$1 \leq k \leq 10^4$

## 题目链接

[Codeforces Hello 2019 - D](https://codeforces.com/contest/1097/problem/D)

<!-- more -->

## 题解

容易发现答案是关于 $n$ 的积性函数，则只需考虑 $n = p^q ~ (p \text{ is prime})$ 时的答案，记为 $f(p, q, k)$，则有：
$$
f(p, q, k) = \frac{1}{q + 1} \sum_{i = 0}^{q} f(p, i, k - 1)
$$
通过预处理逆元和前缀和可以做到 $O(qk)$ （不预处理逆元也可以过）。

## 代码

```c++
#include <bits/stdc++.h>

const int MOD = 1000000007;

long long qpow(long long a, long long n) {
    long long res = 1;
    for (; n; n >>= 1, a = a * a % MOD) if (n & 1) res = res * a % MOD;
    return res;
}

long long inv(long long x) {
    return qpow(x, MOD - 2);
}

const int MAXK = 10005;

std::queue<std::pair<long long, int> > q;

long long calc(long long p, long long n, long long k) {
    static long long f[2][MAXK];
    for (int i = 0; i <= n; i++) f[0][i] = qpow(p, i);

    int curr = 0, last = 1;
    for (int i = 0; i < k; i++) {
        for (int j = 1; j <= n; j++) {
            f[curr][j] += f[curr][j - 1];
            f[curr][j] >= MOD ? f[curr][j] -= MOD : 0;
        }
        curr ^= 1, last ^= 1;
        for (int j = 0; j <= n; j++)
            f[curr][j] = f[last][j] * inv(j + 1) % MOD;
    }

    return f[curr][n];
}

int main() {
    long long n;
    int k;
    scanf("%lld %d", &n, &k);

    long long temp = n;
    for (long long i = 2; i * i <= temp; i++) if (temp % i == 0) {
        int k = 0;
        while (temp % i == 0) {
            ++k;
            temp /= i;
        }
        q.emplace(i, k);
    }
    if (temp > 1) q.emplace(temp, 1);

    long long ans = 1;
    while (!q.empty()) {
        auto u = q.front();
        q.pop();

        long long t = calc(u.first, u.second, k);
        ans = ans * t % MOD;
    }

    printf("%lld\n", ans);
    
    return 0;
}
```