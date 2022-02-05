---
title: '[Codeforces Good Bye 2018] F New Year and the Mallard Expedition'
date: 2019-01-01 11:28:30
tags: [贪心]
categories: 题解（OI/XCPC）
---

## 题目大意

走一段由 $n$ 段路程组成的路，每段路的长度为 $l_i$，地形为草地、水、岩浆中的一种。有三种运动方式，走路、游泳和飞行。走路一单位长度耗时 $5$，可获得 $1$ 点能量，只能在草地上；游泳一单位长度耗时 $3$，可获得 $1$ 点能量，只能在水中；飞行一单位长度耗时 $1$，消耗 $1$ 点能量，可以在任何地形上使用。每时每刻能量不可为负值，可以在任意时刻调整运动方式和运动方向，求最短用时。

$1 \leq n \leq 100,000$

$1 \leq l_i \leq 10^{12}$

## 题目链接

[Codeforces Good Bye 2018 - F](https://codeforces.com/contest/1091/problem/F)

<!-- more -->

## 题解

先在所有草地上走、在所有水中游泳、在所有岩浆上飞行，考虑贪心地调整到最优解。

首先，以上描述的运动策略不一定合法，即可能会出现能量为负的情况。此时可以考虑选取之前的一段水或草地，在其上往返游泳或走一单位长度可获得 $1$ 点能量，由于游泳耗时更短，所以只要之前有水，就在水上往返运动，且可获得的能量与路径长度无关（即再短的路也可以为我们带来所需的能量）。

其次，可以发现最优解时最后一定不会有剩余的能量，如果有，我们考虑将之前的一段走或游泳转为飞行，$S$ 点剩余能量可以转化之前 $S / 2$ 单位长度的路。由于走路耗时更大，故应尽可能多地转化步行。考虑可以被转化的步行的长度的上界：首先，记走过了长为 $G$ 的草地，上界一定不大于 $G$；其次，途中不能出现负值的能量，所以对每一段路结束时剩余的能量 $S$ ，用 $S / 2$ 更新上界。

## 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 100005;

long long l[MAXN];
char str[MAXN];

int main() {
    int n;
    scanf("%d", &n);
    for (int i = 0; i < n; i++) scanf("%lld", &l[i]);
    scanf("%s", str);

    long long ans = 0, stamina = 0, convert = 0;
    bool hasWater = false;

    for (int i = 0; i < n; i++) {
        if (str[i] == 'G') {
            ans += 5 * l[i];
            stamina += l[i];
            convert += 2 * l[i];
        } else if (str[i] == 'W') {
            ans += 3 * l[i];
            stamina += l[i];
            hasWater = true;
        } else  {
            ans += l[i];
            stamina -= l[i];
            if (stamina < 0) {
                ans -= stamina * (hasWater ? 3 : 5);
                stamina = 0;
            }
        }

        convert = std::min(convert, stamina);
    }

    if (stamina > 0) {
        ans -= 2 * convert;
        ans -= stamina - convert;
    }

    printf("%lld\n", ans);
    
    return 0;
}
```