---
title: '[BZOJ 3944] Sum'
date: 2017-05-03 07:48:49
tags: [数论, 线性筛, 杜教筛, 莫比乌斯反演, 模版]
categories: 题解（OI/XCPC）
---

## 题目大意

给定 $n$ ，求 $\sum_{i = 1}^{n} \varphi(i)$ 和 $\sum_{i = 1}^{n} \mu(i)$，多组询问。

$1 \leqslant T \leqslant 10$

$0 \leqslant n < 2^{31}$ （为什么会有 $0$。。。）

## 题目链接

[BZOJ 3944](http://www.lydsy.com/JudgeOnline/problem.php?id=3944)

<!-- more -->

## 题解

杜教筛模版题。

狄利克雷卷积有 $\varphi \times 1 = id$ 、$mu \times 1 = e$ ，则有：
$$
\begin{align}
\Phi(n) &= \frac{n (n + 1)}{2} - \sum_{i = 2}^{n} \Phi(\lfloor \frac{n}{i} \rfloor) \\
M(n) &= 1 - \sum_{i = 2}^{n} M(\lfloor \frac{n}{i} \rfloor)
\end{align}
$$
这道题 RE、WA、TEL 了一共大概 $20$ 次，其中四五次是和 [BZOJ 4805](http://pepcy.cf/BZOJ-4805-欧拉函数求和/) 一样的原因，一些是重写了一遍少些了一句的 TLE，一些是蜜汁 RE，一些是蜜汁运算中过了 `int` （好像）而 WA 。。。因为太惨不忍睹了，所以以上有一半是开了小号交的。。。

## 代码

```c++
#include <cstdio>
#include <map>
const int MAXNN = 1700000;
long long phi[MAXNN], mu[MAXNN];
int prime[MAXNN], primeCnt;
bool notPrime[MAXNN];
void linearShaker() {
    notPrime[0] = notPrime[1] = true;
    phi[1] = mu[1] = 1;
    for (int i = 2; i < MAXNN; i++) {
        if (!notPrime[i]) {
            prime[++primeCnt] = i;
            phi[i] = i - 1;
            mu[i] = -1;
        }
        for (int j = 1; j <= primeCnt && i * prime[j] < MAXNN; j++) {
            notPrime[i * prime[j]] = true;
            if (i % prime[j] == 0) {
                phi[i * prime[j]] = phi[i] * prime[j];
                mu[i * prime[j]] = 0;
                break;
            } else {
                phi[i * prime[j]] = phi[i] * (prime[j] - 1);
                mu[i * prime[j]] = -mu[i];
            }
        }
    }
    for (int i = 2; i < MAXNN; i++) phi[i] += phi[i - 1], mu[i] += mu[i - 1];
}
long long pSumPhi(long long n) {
    if (n < MAXNN) return ::phi[n];
    static std::map<int, long long> phi;
    std::map<int, long long>::iterator it;
    if ((it = phi.find(n)) != phi.end()) return it->second;
    long long res = (long long) n * (n + 1) / 2, last;
    for (long long i = 2; i <= n; i = last + 1) {
        last = n / (n / i);
        res -= (last - i + 1) * pSumPhi(n / i);
    }
    return phi[n] = res;
}
long long pSumMu(long long n) {
    if (n < MAXNN) return ::mu[n];
    static std::map<int, long long> mu;
    std::map<int, long long>::iterator it;
    if ((it = mu.find(n)) != mu.end()) return it->second;
    long long res = 1, last;
    for (long long i = 2; i <= n; i = last + 1) {
        last = n / (n / i);
        res -= (last - i + 1) * pSumMu(n / i);
    }
    return mu[n] = res;
}
int main() {
    linearShaker();
    int T;
    scanf("%d", &T);
    while (T--) {
        long long n;
        scanf("%lld", &n);
        printf("%lld %lld\n", pSumPhi(n), pSumMu(n));
    }
    return 0;
}
```