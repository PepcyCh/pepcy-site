---
title: '[国家集训队] Crash的数字表格'
date: 2017-05-04 20:37:27
tags: [数论, 线性筛, 莫比乌斯反演, 积性函数]
categories: 题解（OI/XCPC）
---

## 题目大意

给定 $n$、$m$ ，求
$$
\sum_{i = 1}^{n} \sum_{j = 1}^{m} lcm(i, \; j) \bmod 20,101,009
$$
$1 \leqslant n, \; m \leqslant 10,000,000$

## 题目链接

[【国家集训队】Crash 的数字表格/ZJPTAB - Luogu 1829](https://www.luogu.com.cn/problem/P1829)

<!-- more -->

## 题解

$$
\begin{align}
&\sum_{i = 1}^{n} \sum_{j = 1}^{m} lcm(i, \; j) \\
= &\sum_{i = 1}^{n} \sum_{j = 1}^{m} \frac{i j}{gcd(i, \; j)} \\
= &\sum_{d = 1}^{n} \frac{1}{d} \sum_{i = 1}^{n} \sum_{j = 1}^{m} [gcd(i, \; j) = d]ij \\
= &\sum_{d = 1}^{n} d \sum_{i = 1}^{\lfloor \frac{n}{d} \rfloor} \sum_{j = 1}^{\lfloor \frac{m}{d} \rfloor} [gcd(i, \; j) = d]ij \\
\end{align}
$$

记后面的和式为 $f(n, \; m)$。
$$
\begin{align}
f(n, \; m) =&\sum_{i = 1}^{n} \sum_{j = 1}^{m} [gcd(i, \; j) = d]ij \\
= &\sum_{i = 1}^{n} \sum_{j = 1}^{m} i j \sum_{d | i, \; d | j} \mu(d) \\
= &\sum_{d = 1}^{n} \mu(d) \sum_{i = 1}^{n} \sum_{j = 1}^{m} [d | i, \; d | j] ij \\
= &\sum_{d = 1}^{n} \mu(d) \sum_{d | i} \sum_{d | j} ij \\
= &\sum_{d = 1}^{n} \mu(d) \sum_{d | i} i \frac{d \lfloor \frac{m}{d} \rfloor (\lfloor \frac{m}{d} + 1 \rfloor)}{2} \\
= &\frac{1}{4} \sum_{d = 1}^{n} \mu(d) d^2 \lfloor \frac{n}{d} \rfloor \lfloor \frac{m}{d} \rfloor (\lfloor \frac{n}{d} + 1\rfloor) (\lfloor \frac{m}{d} + 1 \rfloor) \\
\end{align}
$$
带回原式：
$$
\begin{align}
&\sum_{i = 1}^{n} \sum_{j = 1}^{m} lcm(i, \; j) \\
= &\frac{1}{4} \sum_{d = 1}^{n} d \sum_{d' = 1}^{\lfloor \frac{n}{d} \rfloor} \mu(d') d'^2 \lfloor \frac{n}{dd'} \rfloor \lfloor \frac{m}{dd'} \rfloor (\lfloor \frac{n}{dd'} + 1\rfloor) (\lfloor \frac{m}{dd'} + 1 \rfloor) \\
= &\frac{1}{4} \sum_{T = 1}^{n} T \lfloor \frac{n}{T} \rfloor \lfloor \frac{m}{T} \rfloor (\lfloor \frac{n}{T} + 1 \rfloor) (\lfloor \frac{m}{T} + 1\rfloor) \sum_{d | T} d \mu(d)
\end{align}
$$
记 $g(n) = n \sum_{d | n} d \mu(d)$，发现它有积性。考虑在线性筛中从 $i$ 转移到 $i \times p_j$，若有 $p_j | i$，发现多出来的那些项的 $\mu(d)$ 一项均为 $0$，最后造成的结果是，只有式子中的 $n$ 被乘上了 $p_j$。

其实以上是多组询问时的做法，单组询问的本题并不需要这么麻烦，但我已经推到这了。。。

多组询问版本：[BZOJ 2693](http://www.lydsy.com/JudgeOnline/problem.php?id=2693)（权限题）。

## 代码

```c++
#include <cstdio>
#include <algorithm>
const int MAXN = 10000005;
const int MOD = 20101009;
long long f[MAXN];
int prime[MAXN], primeCnt;
bool notPrime[MAXN];
void linearShaker() {
    notPrime[0] = notPrime[1] = true;
    f[1] = 1;
    for (int i = 2; i < MAXN; i++) {
        if (!notPrime[i]) {
            prime[++primeCnt] = i;
            f[i] = (long long) i * (1 - i) % MOD;
        }
        for (int j = 1; j <= primeCnt && i * prime[j] < MAXN; j++) {
            notPrime[i * prime[j]] = true;
            if (i % prime[j] == 0) {
                f[i * prime[j]] = f[i] * prime[j] % MOD;
                break;
            } else f[i * prime[j]] = f[i] * f[prime[j]] % MOD;
        }
    }
    for (int i = 2; i < MAXN; i++) (f[i] += f[i - 1]) %= MOD;
}
long long pow(long long a, long long n) {
    long long res = 1;
    for (; n; n >>= 1, a = a * a % MOD) if (n & 1) res = res * a % MOD;
    return res;
}
long long calc(int n, int m) {
    if (n > m) std::swap(n, m);
    long long res = 0;
    for (int i = 1, last; i <= n; i = last + 1) {
        last = std::min(n / (n / i), m / (m / i));
        res += ((((long long) n / i) * ((long long) n / i + 1) / 2) % MOD)
             * ((((long long) m / i) * ((long long) m / i + 1) / 2) % MOD) % MOD
             * (f[last] - f[i - 1]) % MOD;
    }
    return (res % MOD + MOD) % MOD;
}
int main() {
    linearShaker();
    int n, m;
    scanf("%d %d", &n, &m);
    printf("%lld\n", calc(n, m));
    return 0;
}
```