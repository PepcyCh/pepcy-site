---
title: '[JSOI 2008] 魔兽地图'
date: 2017-04-09 21:44:20
tags: [树形DP, DP]
categories: 题解（OI/XCPC）
---

## 题目大意

给定 $n$ 个装备和数量为 $m$ 的金钱，求可获得的最大力量。其中，每个装备有一个力量值，同时装备分为基本装备和高级装备，基本装备给定了价格和数量限制，高级装备需要一些其他装备来合成（需要哪些、需要几个已给定）。

$1 \leqslant n \leqslant 51$

$1 \leqslant m \leqslant 2,000$

$1 \leqslant limit \leqslant 100$（数量限制）

## 题目链接

[【JSOI 2008】魔兽地图 - Luogu 4037](https://www.luogu.com.cn/problem/P4037)

<!-- more -->

## 题解

树形 DP。

对于每一个节点 $x$，有一个 $f[i, \; j]$，表示花费 $j$ 的金钱、有 $i$ 个 $x$ 被用于合成高一级的装备时的最大力量。在转移时，定义一个辅助数组 $g[i, \; j]$，表示当前节点的子节点中，已经考虑了 $i$ 个、花费为 $j$ 的最大力量。记 $tot$ 为一个节点字节点数，则转移为（当前节点为 $x$）：

枚举 $x$ 的合成量 $l$：

$$g[i, \; j] = max(g[i - 1, \; j - k] + v.f[l \times num(v->x), \; k]), \; v \; is \; son \; of \; x$$

再枚举其中的 $i$ 个用于合成高一级装备：

$$x.f[i, \; j] = max(g[tot, \; j] + x.price \times (l - i))$$

## 代码

```c++
#include <cstdio>
#include <climits>
#include <cstring>
#include <algorithm>
// #define DBG
const int MAXN = 55;
const int MAXLIMIT = 105;
const int MAXM = 2005;
struct Edge;
struct Node {
    Edge *e;
    int strength, limit, price, deg;
    int f[MAXLIMIT][MAXM];
#ifdef DBG
    int id;
#endif
} N[MAXN];
struct Edge {
    Node *u, *v;
    Edge *next;
    int w;
    Edge(Node *u, Node *v, int w) : u(u), v(v), w(w), next(u->e) {}
};
void addEdge(int u, int v, int w) {
    N[u].e = new Edge(&N[u], &N[v], w);
    N[v].deg++;
}
int n, m;
int g[MAXN][MAXM];
void dp(Node *u) {
#ifdef DBG
    printf("dp(%d)\n", u->id);
#endif
    if (!u->e) {
        u->limit = std::min(u->limit, m / u->price);
        for (int i = 0; i <= u->limit; i++) {
            for (int j = i; j <= u->limit; j++) 
                u->f[i][j * u->price] = (j - i) * u->strength;
        }
#ifdef DBG
        printf("end dp(%d), limit = %d\n", u->id, u->limit);
#endif
        return;
    }
    u->limit = INT_MAX;
    for (Edge *e = u->e; e; e = e->next) {
        dp(e->v);
        u->limit = std::min(u->limit, e->v->limit / e->w);
        u->price += e->v->price * e->w;
    }
    u->limit = std::min(u->limit, m / u->price);
#ifdef DBG
    printf("end calcLimit(%d), limit = %d\n", u->id, u->limit);
#endif
    memset(g, -0x3f3f3f3f, sizeof (g));
    g[0][0] = 0;
    for (int l = u->limit; l >=0; l--) {
        int tot = 0;
        for (Edge *e = u->e; e; e = e->next) {
            tot++;
            for (int i = 0; i <= m; i++) {
                for (int j = 0; j <= i; j++) 
                    g[tot][i] = std::max(g[tot][i], 
                                         g[tot - 1][i - j] + e->v->f[e->w * l][j]);
            }
        }
        for (int i = 0; i <= l; i++) {
            for (int j = 0; j <= m; j++) 
                u->f[i][j] = std::max(u->f[i][j], g[tot][j] + (l - i) * u->strength);
        }
    }
}
int main() {
    scanf("%d %d", &n, &m);
#ifdef DBG
    for (int i = 1; i <= n; i++) N[i].id = i;
#endif
    for (int i = 1; i <= n; i++) {
        char type[2];
        scanf("%d %c", &N[i].strength, type);
        if (type[0] == 'A') {
            int c;
            scanf("%d", &c);
            for (int j = 0; j < c; j++) {
                int x, num;
                scanf("%d %d", &x, &num);
                addEdge(i, x, num);
            }
        } else scanf("%d %d", &N[i].price, &N[i].limit);
    }
    for (int i = 1; i <= n; i++) memset(N[i].f, -0x3f3f3f3f, sizeof (N[i].f));
    static int h[MAXN][MAXM];
    int tot = 0;
    for (int x = 1; x <= n; x++) {
#ifdef DBG
        printf("x = %d\n", x);
#endif
        if (N[x].deg == 0) {
            dp(&N[x]);
            tot++;
#ifdef DBG
            printf("tot = %d\n", tot);
#endif
            for (int i = 0; i <= m; i++) {
                for (int j = 0; j <= i; j++) {
                    for (int k = 0; k <= N[x].limit; k++) {
                        h[tot][i] = std::max(h[tot][i], 
                                             h[tot - 1][j] + N[x].f[k][i - j]);
                    }
                }
            }
        }
    }
    int ans = 0;
    for (int i = 0; i <= m; i++) ans = std::max(ans, h[tot][i]);
    printf("%d\n", ans);
    return 0;
}
```