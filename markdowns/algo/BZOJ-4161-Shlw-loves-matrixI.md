---
title: '[BZOJ 4161] Shlw loves matrixI'
date: 2018-01-27 21:39:35
tags: [线性齐次递推, FFT, 模版]
categories: 题解（OI/XCPC）
---

## 题目大意

给定数列 $\{h_n\}$ 的前 $k$ 项（从 $0$ 开始）及 $a_1, a_2 \dots a_k$，数列的每一项满足：
$$
h_n = \sum_{i = 1}^{k} a_ih_{k - i}
$$
求 $h_n$ 对 $1,000,000,007$ 取模的结果。

$|h_i|, |a_i|, n \leq 10^9$

$k \leq 2000$

## 题目链接

[BZOJ 4161](http://www.lydsy.com/JudgeOnline/problem.php?id=4161)

<!--more-->

## 题解

多项式取模求线性齐次递推的模版题（但模数不太好啊）。

矩阵快速幂的解法复杂度为 $O(k^3\log n)$，显然无法解决这个问题，所以我们考虑其他方法。

以下方法会用到多项式取模，关于用 FFT 优化这个东西（本题暴力取模也是可以过的），可以看 Miskcoo 的这两篇博文：

[多项式求逆 - Miskcoo' Space](http://blog.miskcoo.com/2015/05/polynomial-inverse)

[多项式除法及求模 - MIskcoo's Space](http://blog.miskcoo.com/2015/05/polynomial-division)

定义转移矩阵：
$$
A = \begin{vmatrix}
a_1 & a_2 & a_3 & \cdots & a_k \\
1 & 0 & 0 & \cdots & 0 \\
0 & 1 & 0 & \cdots & 0 \\
\vdots & \vdots & \vdots & \ddots & \vdots \\
0 & 0 & 0 & \cdots & 0
\end{vmatrix}
$$
顺便定义这样一个矩阵，以便后续表示方便：
$$
H^{(i)} = \begin{vmatrix}
h_i \\
h_{i - 1} \\
h_{i - 2} \\
\vdots \\
h_{i - k + 1}
\end{vmatrix}
$$
考虑 $A$ 的特征多项式：
$$
f(x) = |xI - A| = x^k - \sum_{i = 1}^{k} a_ix^{k - i}
$$
由 Hamilton-Cayley 定理知：$f(A) = 0$。

因为要求 $h_n$，相当于求 $H^{(n)} = A^{n}H^{(0)}$。

考虑下式：
$$
x^n = Q(x) f(x) + M(x) \Leftrightarrow x^n \equiv M(x) (\bmod f(x))
$$
带入 $x = A$，由 $f(A) = 0$ 有：
$$
A^n = M(A) = \sum_{i = 0}^{k - 1}m_iA^i
$$
两侧同时乘 $H^{(0)}$：
$$
H^{(n)} = A^nH^{(0)} = \sum_{i = 0}^{k - 1}m_iA^iH^{(0)} = \sum_{i = 0}^{k - 1}m_iH^{(i)}
$$
考虑第一行：
$$
h_n = \sum_{i = 0}^{k - 1}m_ih_i
$$
故我们可以用多项式取模求出 $M(x)$，然后带值即可求出 $h_n$。

计算 $x^n$ 时用快速幂，取模可以 FFT，也可以暴力，用 FFT 的时间复杂度是 $O(k \log k \log n)$，用暴力的时间复杂度是 $O(k^2 \log n)$。

## 代码

只写了暴力取模。。。太弱了不会在模 $1,000,000,007 = 500,000,003 \times 2 + 1$ 下用 NTT。

```c++
#include <cstdio>
#include <cstring>
#include <algorithm>
 
const int MOD = 1000000007;
const int MAXK = 2005;
 
long long a[MAXK];
void mul(long long *A, long long *B, long long *r, int k) {
    static long long res[MAXK << 1];
    // std::fill(res, res + (k << 1), 0);
    memset(res, 0, sizeof (res));
 
    for (int i = 0; i < k; i++) for (int j = 0; j < k; j++) {
        res[i + j] += A[i] * B[j] % MOD;
        res[i + j] >= MOD ? res[i + j] -= MOD : 0;
    }
 
    for (int i = (k << 1) - 2; i >= k; i--) if (res[i]) for (int j = k - 1; ~j; j--) {
        res[i - k + j] += res[i] * a[j] % MOD;
        res[i - k + j] >= MOD ? res[i - k + j] -= MOD : 0;
    }
 
    for (int i = 0; i < k; i++) r[i] = res[i];
}
 
int main() {
    int n, k;
    scanf("%d %d", &n, &k);
     
    static long long h[MAXK];
    for (int i = k - 1; ~i; i--) { // 这里倒着输入的原因是，我想让 -a[i] 表示特征多项式 x^i 的系数
        scanf("%lld", &a[i]);
        a[i] < 0 ? a[i] += MOD : 0;
    }
 
    for (int i = 0; i < k; i++) {
        scanf("%lld", &h[i]);
        h[i] < 0 ? h[i] += MOD : 0;
    }
 
    if (n < k) return printf("%lld\n", h[n]), 0;
 
    static long long m[MAXK], t[MAXK];
    m[0] = t[1] = 1;
 
    for (int i = n; i; i >>= 1, mul(t, t, t, k)) {
        if (i & 1) mul(m, t, m, k);
    }
 
    long long hn = 0;
    for (int i = 0; i < k; i++) hn = (hn + m[i] * h[i] % MOD) % MOD;
 
    printf("%lld\n", hn);
 
    return 0;
}
```