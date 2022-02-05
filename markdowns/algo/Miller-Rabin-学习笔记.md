---
title: Miller-Rabin 学习笔记
date: 2017-05-07 14:58:15
tags: [学习笔记, 数论, 模版, Miller-Rabin]
categories: 学习笔记（OI/XCPC）
---

Miller-Rabin 算法用于判断一个数是否为质数，所谓「素数测试」是也。

## 算法介绍及过程

> **费马小定理**：若 $p$ 为质数，则有 $x^p \equiv x \; (\bmod p)$。

由费马小定理得：

* 若 $n$ 为质数，则 $x^{n - 1} \equiv 1 \; (\bmod n)$ 
* 若 $x^{n - 1} \not \equiv 1 \; (\bmod n)$ ，则 $n$ 不是质数

如果我们多次随机 $x$ 进行判断，大概率地会得出正确的结果。

<!-- more -->

但是！有一种东西叫**卡迈克尔（Carmichael）数**。

> **卡迈克尔数**：一个数 $n$ 为卡迈克尔数，当且仅当对于所有与 $n$ 互质的数 $x$ 都有 $x^{n - 1} \equiv 1 \; (\bmod n)$，且 $n$ 为和数。
>
> 最小的卡迈克尔数是 $561$。
>
> [[wiki]](https://zh.wikipedia.org/wiki/卡邁克爾數) [[OEIS:A002997]](https://oeis.org/A002997)

就是这个 $561$，把随机的方法卡了。。。（于是这篇博文是重写的，感谢 VW 学长）

于是我们需要一个准确的算法。

如果对于一定范围内的 $a$ 都有 $a^{n - 1} \equiv 1 \; (\bmod n)$ 时可以保证 $n$ 是质数，那就很不错了。从 [wiki](https://en.wikipedia.org/wiki/Miller–Rabin_primality_test) 上我们知道，这是对的（证明就不要管了。。。），而且对于 `unsigned long long` 范围内的数，只要让 $a$ 等于前 $12$ 个质数即可（来自 wiki，已验证）。

## 对 $a^{n - 1} \not\equiv 1 \; (\bmod n)$ 判断的一般写法

一般不是直接计算的。。。

把 $n - 1$ 分解为 $2^s d$ （ $d$ 为奇数），用平方差公式分解为：
$$
a^{n - 1} \equiv a^{2^s d} \equiv (a^d - 1) \prod_{r = 0}^{s - 1} (a^{2^r d} + 1) \; (\bmod n)
$$
即 $n$ 是合数 $\Leftrightarrow$ 存在 $a$ 使得：

* $a^d \not\equiv 1\; (\bmod n)$
* $a^{2^r d} \not\equiv -1\; (\bmod n) \qquad r \in [0, s - 1]$

## 算法代码

```c++
long long mul(long long a, long long b, long long mod) {
    long long res = 0;
    for (; b; b >>= 1, a = (a + a) % mod) if (b & 1) res = (res + a) % mod;
    return res;
}
long long pow(long long a, long long n, long long mod) {
    long long res = 1;
    for (; n; n >>= 1, a = mul(a, a, mod)) if (n & 1) res = mul(res, a, mod);
    return res;
}

bool isPrime(long long n) {
    static int primes[12] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37};
    long long s = 0, d = n - 1;
    while (d % 2 == 0) d /= 2, s++;
    if (s == 0) return n == 2;
    for (int i = 0; i < 12 && primes[i] < n; i++) {
        long long a = primes[i];
        if (pow(a, d, n) != 1) {
            bool flag = true;
            for (int r = 0; r < s; r++) 
                if (flag && pow(a, d * (1 << r), n) == n - 1) flag =  false;
            if (flag) return false;
        }
    }
    return true;
}
```