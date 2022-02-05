---
title: Pollard's Rho 学习笔记
date: 2017-05-05 22:18:13
tags: [数论, 模版, Pollard's Rho, 学习笔记]
categories: 学习笔记（OI/XCPC）
---

Pollard's Rho 算法是用于分解质因数的算法。

## 算法介绍与过程

用 [Miller-Rabin 算法](http://pepcy.cf/Miller-Rabin-学习笔记/)判断当前要分解的数是否为质数，若是，在对应位上指数加一，直接返回。

若不是，找到一个数 $p$ 是它为 $n$ 的约数，然后对 $n / p$ 、$p$ 递归求解。

<!-- more -->

假设我们现在要找 $n$ 的一个约数 $p$，先让 $p = n$ ，之后进行计算。

先随机一个数 $x$ ，并令 $y = x$（[wiki](https://en.wikipedia.org/wiki/Pollard%27s_rho_algorithm) 上是令 $x = y = 2$ ），按以下方式进行判断：

让 $x = g(x)$ ，计算 $d = gcd(|x - y|, p)$ ，如果 $d$ 是 $1$ 则让 $y = g(g(y))$ （实现时并没有每次都改变 $y$，而是在计算了 $2$、$4$、$8$、$16$ …… 次后让 $y = x$）并继续重复该算法；若 $d = n$，则判断失败（一般返回 $p$ 让重新寻找）；否则返回 $d$ 作为约数。

其中 $g(x)$ 一般为 $g(x) = (x^2 + c) \bmod p$ ，$c$ 为常数，一般为 $1$ ，在重新计算时改变常数。

如果在寻找时出现了 $x = y$ ，则意味着出现了环，返回失败并让重新寻找。$x$、$y$ 形成的环一般为 $\rho$ 型，这就是算法名字的由来。另外，取 $x = g(x), \; y = g(g(y))$ 也是为了能够判圈（就像 Floyd 判圈法）。

吐槽：一开始完全按 wiki 上写的实现，结果卡住了，一直找不到约数 233333

## 算法代码

```c++
#include <cstdlib>

long long mul(long long a, long long b, long long mod) {
    long long res = 0;
    for (; b; b >>= 1, a = (a + a) % mod) if (b & 1) res = (res + a) % mod;
    return res;
}
long long gcd(long long a, long long b) {
    return b ? gcd(b, a % b) : a;
}

namespace PollardRho {
    long long g(long long x, long long n, long long c) {
        return (mul(x, x, n) + c) % n;
    }
    long long rho(long long n, long long c) {
        long long x = rand() % n, y = x, d = 1;
        for (long long i = 1, k = 2; d == 1; i++) {
            x = g(x, n, c);
            d = gcd(x > y ? x - y : y - x, n);
            if (x == y) return n;
            if (i == k) k <<= 1, y = x;
        }
        return d;
    }
    void find(long long n, long long c, std::map<long long, int> &res) {
        if (n == 1) return;
        if (isPrime(n)) {
            res[n]++;
            return;
        }
        long long p = n;
        while (p == n) p = rho(p, c++);
        find(p, c, res);
        find(n / p, c, res);
    }
    std::map<long long, int> divide(long long n) {
        std::map<long long, int> res;
        find(n, 1, res);
        return res;
    }
}
```