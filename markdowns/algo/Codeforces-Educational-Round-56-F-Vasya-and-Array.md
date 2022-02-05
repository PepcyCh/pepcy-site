---
title: '[Codeforces Educational Round 56] F Vasya and Array'
date: 2018-12-25 12:40:06
tags: [DP]
categories: 题解（OI/XCPC）
---

## 题目大意

定义一个数列是好的，当且仅当它的任何长为 $len$ 的连续子序列包含至少两种数字。现给出一个长为 $n$ 的数列，每个位置是 $1 \sim k$ 中的数或 $-1$，要求替换所有的 $-1$ 为 $1 \sim k$ 中的数，使得该数列为好数列，求替换的方案数。

$1 \leq len \leq n \leq 100,000$

$1 \leq k \leq 100$

## 题目链接

[Codeforces Educational Round 56 - F](https://codeforces.com/contest/1093/problem/F)

<!--more-->

## 题解

考虑 DP，$f(i, j)$ 表示已经考虑了前 $i$ 个数，最后一个是 $j$ 的答案，则在不考虑要求的情况下，有：
$$
f(i, j) = \sum_{j' = 1}^{k} f(i - 1, j')
$$
然后考虑减去不合法的情况，由于更早的不合法情况已经考虑过了，只需考虑 $[i - len + 1, i]$ 这个区间（如果 $i - len + 1 > 0$），它们只有都是 $j$ 的情况下才不合法，这需要原数列中该区间内只有 $j$ 和 $-1$，此时减去不合法的情况数，也就是前 $i - len$ 个数的答案，此时如果第 $i - len$ 个数也为 $j$，那么它在计算 $f(i - 1, j)$ 时就被考虑了，故减去 $\sum_{j' = 1}^{k} f(i - len, j') - f(i - len, j)$ 。

## 代码

```c++
#include <cstdio>

const int MAXN = 100005;
const int MAXK = 105;
const int MOD = 998244353;

int f[MAXN][MAXK], a[MAXN], cnt[MAXK][MAXN];

int main() {
    int n, k, len;
    scanf("%d %d %d", &n, &k, &len);
    for (int i = 1; i <= n; i++) scanf("%d", &a[i]);

    for (int i = 1; i <= k; i++) for (int j = 1; j <= n; j++)
        cnt[i][j] = cnt[i][j - 1] + (a[j] == -1 || a[j] == i);

    f[0][0] = 1;
    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= k; j++) if (a[i] == -1 || a[i] == j) {
            f[i][j] = f[i - 1][0];

            if (i >= len && cnt[j][i] - cnt[j][i - len] == len) {
                f[i][j] -= f[i - len][0];
                f[i][j] < 0 ? f[i][j] += MOD : 0;
                f[i][j] += f[i - len][j];
                f[i][j] >= MOD ? f[i][j] -= MOD : 0;
            }

            f[i][0] += f[i][j];
            f[i][0] >= MOD ? f[i][0] -= MOD : 0;
        }
    }

    int ans = a[n] == -1 ? f[n][0] : f[n][a[n]];
    printf("%d\n", ans);
    
    return 0;
}
```