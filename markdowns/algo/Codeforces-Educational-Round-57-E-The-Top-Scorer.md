---
title: '[Codeforces Educational Round 57] E The Top Scorer'
date: 2018-12-30 15:24:28
tags: [计数, 概率与期望, 容斥原理]
categories: 题解（OI/XCPC）
---

## 题目大意

$p$ 个人玩一个游戏，胜者会从分数最高的人中等概率地选出。已知 $p$ 个人的分数总和 $s$ 和一号玩家的分数下限 $r$，求一号玩家获胜的概率。答案对 $998244353$ 取模。

$1 \leq p \leq 100$

$0 \leq r \leq s \leq 5,000$

## 题目链接

[Codeforces Educational Round 57 - E The Top Scorer](https://codeforces.com/contest/1096/problem/E)

<!-- more -->

## 题解

让一号玩家取胜，我们可以枚举一号玩家的分数 $i$，以及达到该分数的玩家数 $cnt$，其余的人分数任意但均小于 $i$，且总和为 $s - i \times cnt$。即统计 $n$ 个不超过 $l$ 的数和为 $sum$ 的方案数，这是一个经典的容斥问题，其算式为 $\sum_{i = 0}^{n} (-1)^i \binom{n}{i} \binom{sum + n - 1 - il}{n - 1}$ 。由此可在确定 $i$、$cnt$ 的情况下的算得方案数（对答案的贡献每一项要乘上 $cnt^{-1}$ ），最后除以总方案数即可。

## 代码

```c++
#include <cstdio>

const int MAXN = 10005;
const int MOD = 998244353;

long long qpow(long long a, long long n) {
    long long res = 1;
    for (; n; n >>= 1, a = a * a % MOD) if (n & 1) res = res * a % MOD;
    return res;
}

long long inv(long long x) {
    return qpow(x, MOD - 2);
}

long long fact[MAXN], invFact[MAXN];
void init() {
    fact[0] = 1;
    for (int i = 1; i < MAXN; i++) fact[i] = fact[i - 1] * i % MOD;
    invFact[MAXN - 1] = inv(fact[MAXN - 1]);
    for (int i = MAXN - 2; ~i; i--) invFact[i] = invFact[i + 1] * (i + 1) % MOD;
}

long long combi(int n, int m) {
    if (n < 0 || n < m) return 0;
    return fact[n] * invFact[m] % MOD * invFact[n - m] % MOD;
}

long long calc(int n, int l, int s) {
    if (!n) return !s;

    long long res = 0;
    for (int i = 0; i <= n; i++) {
        long long temp = combi(n, i) * combi(s + n - 1 - i * l, n - 1) % MOD;
        i % 2 ? res -= temp : res += temp;
        res < 0 ? res += MOD : 0;
        res >= MOD ? res -= MOD : 0;
    }
    return res;
}

int main() {
    init();

    int p, s, r;
    scanf("%d %d %d", &p, &s, &r);

    long long ans = 0;
    for (int i = r; i <= s; i++) for (int j = 1; j <= p && j * i <= s; j++) {
        long long temp = combi(p - 1, j - 1) * calc(p - j, i, s - j * i) % MOD * inv(j) % MOD;
        ans += temp;
        ans >= MOD ? ans -= MOD : 0;
    }

    ans = ans * inv(calc(p, s + 1, s - r)) % MOD;
    printf("%lld\n", ans);
    
    return 0;
}
```