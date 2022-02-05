---
title: '[POJ 3207] Ikki''s Story IV - Panda'' Trick'
date: 2017-04-12 21:10:37
tags: [2SAT]
categories: 题解（OI/XCPC）
---

## 题目大意

有一个环上有 $n$ 个点，有 $m$ 条边连接其上的点，要求边要么在环内，要么在环外，求所有的边是否可以做到不相交。可以输出 `panda is telling the truth...`，不可以输出 `the evil panda is lying again`。

点的编号从 $0$ 开始。

$1 \leqslant n \leqslant 1,000$

$1 \leqslant m \leqslant 500$

## 题目链接

[POJ 3207](http://poj.org/problem?id=3207)

<!-- more -->

## 题解

完全就是[Planar（平面图）](http://pepcy.cf/BZOJ-1997-HNOI-2010-Planar/)那道题。。。

## 代码

```c++
#include <cstdio>
#include <stack>
#include <algorithm>
const int MAXN = 1005;
const int MAXM = 505;
struct Edge;
struct Node {
    Edge *e;
    int dfn, low, belong;
    bool ins;
} N[MAXM << 1];
struct Edge {
    Node *u, *v;
    Edge *next;
    Edge(Node *u, Node *v) : u(u), v(v), next(u->e) {}
};
void addEdge(int u, int v) {
    N[u].e = new Edge(&N[u], &N[v]);
}
int getV(int x, int k){
    return (x << 1) - k;
}
struct TwoSat {
    std::stack<Node *> s;
    int dfsClock, sccCnt;
    void tarjan(Node *u) {
        s.push(u);
        u->dfn = u->low = ++dfsClock;
        u->ins = true;
        for (Edge *e = u->e; e; e = e->next) {
            if (e->v->dfn == 0) {
                tarjan(e->v);
                u->low = std::min(u->low, e->v->low);
            } else if (e->v->ins) u->low = std::min(u->low, e->v->dfn);
        }
        if (u->dfn == u->low) {
            Node *c;
            sccCnt++;
            while (true) {
                c = s.top();
                s.pop();
                c->ins = false;
                c->belong = sccCnt;
                if (c == u) break;
            }
        }
    }
    bool check(int n) {
        for (int i = 1; i <= n; i++)
            if (N[getV(i, 0)].belong == N[(getV(i, 1))].belong) return false;
        return true;
    }
    bool operator()(int n) {
        dfsClock = sccCnt = 0;
        for (int i = 1; i <= n << 1; i++) if (N[i].dfn == 0) tarjan(&N[i]);
        return check(n);
    }
} ts;
struct Pair {
    int u, v;
    Pair() {}
    Pair(int u, int v) : u(u), v(v) {}
} E[MAXM];
bool cross(const Pair &a, const Pair &b) {
    return (a.u < b.u && b.u < a.v && a.v < b.v) || 
           (b.u < a.u && a.u < b.v && b.v < a.v);
}
int main() {
    int n, m;
    scanf("%d %d", &n, &m);
    int top = 0;
    for (int i = 1; i <= m; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        if (u > v) std::swap(u, v);
        if (v - u <= 1 || (u == 0 && v == n - 1)) continue;
        E[++top] = Pair(u, v);
    }
    m = top;
    for (int i = 1; i <= m; i++) for (int j = i + 1; j <= m; j++) {
        if (cross(E[i], E[j])) {
            addEdge(getV(i, 0), getV(j, 1));
            addEdge(getV(i, 1), getV(j, 0));
            addEdge(getV(j, 0), getV(i, 1));
            addEdge(getV(j, 1), getV(i, 0));
        }
    }
    puts(ts(m) ? "panda is telling the truth..." : "the evil panda is lying again");
    return 0;
}
```