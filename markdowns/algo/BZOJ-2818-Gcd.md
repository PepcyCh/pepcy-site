---
title: 'GCD'
date: 2017-04-18 20:48:12
tags: [线性筛]
categories: 题解（OI/XCPC）
---

## 题目大意

给定整数 $n$，求 $1 \leqslant x, \; y \leqslant n$ 且 $gcd(x, \; y)$ 为质数的数对 $(x, \; y)$ 有多少对。

$1 \leqslant n \leqslant 10,000,000$

## 题目链接

[GCD - Luogu 2568](https://www.luogu.com.cn/problem/P2568)

<!-- more -->

## 题解

这题应该叫做「为什么我讨厌 gcd」。

在之前的两道题（[【HAOI 2011】problem b](http://pepcy.cf/BZOJ-2301-HAOI-2011-Problem-b/) 和 [【NOI 2010】能量采集](http://pepcy.cf/BZOJ-2005-NOI-2010-能量采集/)）中，我们推出了两个式子。然而，这里我们一个都不用。。。

考虑枚举每个质数，贡献即为
$$
\sum_{1 \leqslant i, \; j \leqslant \lfloor \frac{n}p \rfloor} [gcd(i, \; j) = 1]
$$
考虑范围内每个数，小于它且与它互质的数的个数正好是 $\varphi$ 函数的定义，由于数对有序，且两数相等并互质的数对只有一个 $(1, \; 1)$，故上式等于
$$
2 \times \sum_{1 \leqslant i \leqslant \lfloor \frac{n}p \rfloor} \varphi(i) - 1
$$
（这个式子本身应该是只当两数范围均为 $[1, \; n]$ 时才成立；以及这应该不算莫比乌斯反演了吧。。。）

预处理 $\varphi$ 及其前缀和，枚举每个质数即可。

## 代码

```c++
#include <cstdio>
const int MAXN = 10000005;
int prime[MAXN], primeCnt;
long long phi[MAXN];
bool notPrime[MAXN];
void linearShaker() {
    phi[1] = 1;
    notPrime[0] = notPrime[1] = true;
    for (int i = 2; i < MAXN; i++) {
        if (!notPrime[i]) {
            prime[++primeCnt] = i;
            phi[i] = i - 1;
        }
        for (int j = 1; j <= primeCnt && i * prime[j] < MAXN; j++) {
            notPrime[i * prime[j]] = true;
            if (i % prime[j] == 0) {
                phi[i * prime[j]] = phi[i] * prime[j];
                break;
            } else phi[i * prime[j]] = phi[i] * (prime[j] - 1);
        }
    }
}
int main() {
    int n;
    scanf("%d", &n);
    linearShaker();
    for (int i = 2; i <= n; i++) phi[i] += phi[i - 1];
    long long ans = 0;
    for (int i = 1; i <= primeCnt && prime[i] <= n; i++) ans += 2 * phi[n / prime[i]] - 1;
    printf("%lld\n", ans);
    return 0;
}
```