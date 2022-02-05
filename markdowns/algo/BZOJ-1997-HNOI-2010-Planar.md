---
title: '[HNOI 2010] 平面图判定'
date: 2017-04-11 21:55:28
tags: [2SAT]
categories: 题解（OI/XCPC）
---

## 题目大意

给定包含一个哈密顿回路的图，求其是否为可平面图。多组询问。

$1 \leqslant T \leqslant 100$

$3 \leqslant n \leqslant 200$

$m \leqslant 10,000$

## 题目链接

[【HNOI 2010】平面图判定](https://www.luogu.com.cn/problem/P3209)

<!-- more -->

## 题解

首先给出图论上的一个定理：

> 一个图为可平面图，当且仅当  $m \leqslant 3n - 6$。

这样我们可以只处理 $m \leqslant 594$ 的情况。（学校为了数竞党们请来的图论老师对 OI 还是有用的嘛）

剩下的为 2-SAT。

不是构成哈密顿回路的边为布尔变量，边画在回路内部还是外部对应真或假，对于可能会交叉的边必须一内一外。

## 代码

本题是在满汉全席之前写的，所以代码比满汉全席丑。。。（满汉全席是当板子写的，自然要好看一些啊）

另外，一开始没有写 `inq` 数组及相关的内容，居然就 AC 了。。。

```c++
#include <cstdio>
#include <stack>
#include <algorithm>
// #define DBG
const int MAXN = 205;
struct Edge;
struct Node {
    Edge *e;
    int dfn, low, belong;
    bool inq;
} N[MAXN * 3 * 2];
struct Edge {
    Node *u, *v;
    Edge *next;
    Edge(Node *u, Node *v) : u(u), v(v), next(u->e) {}
};
void addEdge(int u, int v) {
#ifdef DBG
    printf("edge : %d -- %d\n", u, v);
#endif
    N[u].e = new Edge(&N[u], &N[v]);
    N[v].e = new Edge(&N[v], &N[u]);
}
int vTrue(int x) {
    return (x << 1) - 1;
}
int vFalse(int x) {
    return x << 1;
}
int sccCnt, dfsClock;
std::stack<Node *> s;
void tarjan(Node *u) {
    s.push(u);
    u->inq = true;
    u->dfn = u->low = ++dfsClock;
    for (Edge *e = u->e; e; e = e->next) {
        if (e->v->dfn == 0) {
            tarjan(e->v);
            u->low = std::min(u->low, e->v->low);
        } else u->low = std::min(u->low, e->v->dfn);
    }
    if (u->dfn == u->low) {
        Node *c;
        sccCnt++;
        while (true) {
            c = s.top();
            s.pop();
            c->belong = sccCnt;
            c->inq = false;
            if (c == u) break;
        }
    }
}
struct Pair {
    int u, v;
    Pair() {}
    Pair(int u, int v) : u(u), v(v) {}
} E[MAXN * 3];
bool cross(const Pair &a, const Pair &b) {
    return (a.u < b.u && b.u < a.v && a.v < b.v) || 
           (b.u < a.u && a.u < b.v && b.v < a.v);
}
int c[MAXN], pos[MAXN];
bool check(int m) {
    for (int i = 1; i <= m; i++) 
        if (N[vTrue(i)].belong == N[vFalse(i)].belong) return false;
    return true;
}
void clearGraph(int n) {
    for (int i = 1; i <= n; i++) {
        for (Edge *&e = N[i].e, *next; e; next = e->next, delete e, e = next);
        N[i].low = N[i].dfn = N[i].belong = 0;
        N[i].inq = false;
    }
}
int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n, m;
        scanf("%d %d", &n, &m);
        if (m > 3 * n - 6) {
            for (int i = 0; i < m; i++) scanf("%*d %*d");
            for (int i = 0; i < n; i++) scanf("%*d");
            puts("NO");
            continue;
        }
        for (int i = 1; i <= m; i++) scanf("%d %d", &E[i].u, &E[i].v);
        for (int i = 1; i <= n; i++) scanf("%d", &c[i]), pos[c[i]] = i;
        int top = 0;
        for (int i = 1; i <= m; i++) {
            E[i].u = pos[E[i].u];
            E[i].v = pos[E[i].v];
            if (E[i].u > E[i].v) std::swap(E[i].u, E[i].v);
            if (E[i].v - E[i].u <= 1 || (E[i].u == 1 && E[i].v == n)) continue;
            E[++top] = Pair(E[i].u, E[i].v);
        }
        m = top;
#ifdef DBG
        printf("top = %d\n", top);
#endif
        clearGraph(m << 1);
        for (int i = 1; i <= m; i++) for (int j = i + 1; j <= m; j++) {
            if (cross(E[i], E[j])) {
                addEdge(vTrue(i), vFalse(j));
                addEdge(vFalse(i), vTrue(j));
            }
        }
        sccCnt = dfsClock = 0;
        while (!s.empty()) s.pop();
        for (int i = 1; i <= m << 1; i++) if (N[i].dfn == 0) tarjan(&N[i]);
#ifdef DBG
        for (int i = 1; i <= m; i++) 
            printf("i = %d, true : %d, false : %d\n", 
                   i, N[vTrue(i)].belong, N[vFalse(i)].belong);
#endif
        puts(check(m) ? "YES" : "NO");
    }
    return 0;
}
```