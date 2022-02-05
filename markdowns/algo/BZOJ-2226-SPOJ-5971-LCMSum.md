---
title: '[SPOJ 5971] LCMSum'
date: 2017-05-03 21:49:04
tags: [数论, 线性筛, 莫比乌斯反演, 积性函数]
categories: 题解（OI/XCPC）
---

## 题目大意

给定 $n$ ，求
$$
\sum_{i = 1}^{n} lcm(i, \; n)
$$
多组询问。

$1 \leqslant T \leqslant 300,000$

$1 \leqslant n \leqslant 1,000,000$

## 题目链接

[LCMSUM - Luogu SP5971](https://www.luogu.com.cn/problem/SP5971)

[SPOJ 5971 - LCMSUM](http://www.spoj.com/problems/LCMSUM/)

<!-- more -->

## 题解

$$
\begin{align}
&\sum_{i = 1}^{n} lcm(i, ;\ n) \\
= &\sum_{i = 1}^{n} \frac{n i}{gcd(i, \; n)} \\
= &n \sum_{i = 1}^{n} \frac{i}{gcd(i, \; n)} \\
= &n \sum_{d | n} \frac{1}{d} \sum_{i = 1}^{n} [gcd(i, \; n) = d] i  \\
= &n \sum_{d | n} \frac{1}{d} \sum_{i = 1}^{\lfloor \frac{n}{d} \rfloor} [gcd(i, \; \lfloor \frac{n}{d} \rfloor) = 1] i d \\
= &n \sum_{d | n} \sum_{i = 1}^{\lfloor \frac{n}{d} \rfloor} [gcd(i, \; \lfloor \frac{n}{d} \rfloor) = 1] i \\
\end{align}
$$

记后面的和式为 $f(i)$，由于当 $n > 1$ 时，如果有 $gcd(n, \; x) = 1$，则有 $gcd(n, \; n - x) = 1$，所以与 $n$ 互质的数是成对出现的，所以有：
$$
f(i) = 
\begin{cases}
1 \qquad n = 1 \\
\frac{n \varphi(n)}{2} \qquad n > 1
\end{cases}
$$
此时答案为 $n \sum_{d | n} f(d)$，枚举约数，每次询问复杂度大概是 $O(\sqrt{n})$，考虑进一步优化。

把 $f(i)$ 放回去，$d = 1$ 时补上一个 $1$，则答案为：
$$
n (\sum_{d | n} \lfloor \frac{d \varphi(d)}{2} \rfloor + 1) = \frac{n}{2} (\sum_{d | n}d\varphi(d) + 1)
$$
发现 $g(n) = \sum_{d | n} d \varphi(d)$ 具有积性，但我不会推式子。。。

虽然做不到 $O(n)$ 预处理，但 $O(n \log n)$ 的预处理也够过了。

明明都是小学开始接触的东西，为什么一到了反演，lcm 就比 gcd 复杂了这么多。。。

## 代码

```c++
#include <cstdio>
const int MAXN = 1000005;
long long phi[MAXN], f[MAXN];
int prime[MAXN], primeCnt;
bool notPrime[MAXN];
void shaker() {
    notPrime[0] = notPrime[1] = true;
    phi[1] = 1;
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
    for (int i = 1; i < MAXN; i++) for (int j = i; j < MAXN; j += i) f[j] += i * phi[i];
}
long long calc(int n) {
    return n * (f[n] + 1) / 2;
}
int main() {
    shaker();
    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);
        printf("%lld\n", calc(n));
    }
    return 0;
}
```