---
title: '[USACO 2008] 土地购买'
date: 2017-04-15 22:55:53
tags: [DP, 斜率优化DP, 单调队列]
categories: 题解（OI/XCPC）
---

## 题目大意

有 $n$ 块土地，每块的大小为 $w_i \times h_i$。对于一块土地，它的价格是它的面积；一次可以购买多块土地，价格为 $max(w) \times max(h)$。求购买所有土地的最小花费。

$1 \leqslant n \leqslant 50,000$

$1 \leqslant w_i, \; h_i \leqslant 1,000,000$

## 题目链接

[【USACO08MAR】Land Acquisition G - Luogu 2900](https://www.luogu.com.cn/problem/P2900)

<!-- more -->

## 题解

DP + 斜率优化。

对土地按宽排序，如果有一块土地的长宽均小于另一个，则该土地队答案无影响。删掉所有的无用土地后，得到的土地序列是一维降序，另一维升序，我们可以让宽降序、长升序。

记 $f[i]$ 为此时前 $i$ 块的答案，则转移为：
$$
f[i] = min(f[j] + w[j + 1] \times h[i])
$$
对于后面的一堆，考虑两个决策点 $a$、$b$（$a > b$），假设 $a$ 优于 $b$，有：
$$
\begin{align}
f[a] + w[a + 1] \times h[i] &< f[b] + w[b + 1] \times h[i] \\
\frac{f[a] - f[b]}{w[b + 1] - w[a + 1]} &< h[i]
\end{align}
$$
用斜率的单调队列维护决策点，最优决策点在一个下凸包上。

## 代码

交的代码数组们开成长宽的范围了。。。

```c++
#include <cstdio>
#include <vector>
#include <algorithm>
const int MAXN = 50005;
long long f[MAXN];
std::pair<int, int> a[MAXN];
std::vector<std::pair<int, int> *> vec;
double slope(int i, int j) {
    return (double) (f[i] - f[j]) / (vec[j]->first - vec[i]->first);
}
int n;
void dp() {
    static int q[MAXN];
    int *l = q, *r = q;
    for (int i = 1; i <= n; i++) {
        while (l < r && slope(*l, *(l + 1)) < vec[i - 1]->second) l++;
        int k = *l;
        f[i] = f[k] + (long long) vec[k]->first * vec[i - 1]->second;
        if (i < n) {
            while (l < r && slope(*(r - 1), *r) > slope(*r, i)) r--;
            *++r = i;
        }
    }
}
int main() {
    scanf("%d", &n);
    for (int i = 1; i <= n; i++) scanf("%d %d", &a[i].first, &a[i].second);
    std::pair<int, int> *last = NULL;
    std::sort(a + 1, a + n + 1);
    for (int i = n; i; i--) {
        if (!last || a[i].second > last->second) {
            last = &a[i];
            vec.push_back(last);
        }
    }
    n = vec.size();
    dp();
    printf("%lld\n", f[n]);
    return 0;
}
```