---
title: '[SHOI 2007] 善意的投票'
date: 2017-04-18 21:39:09
tags: [网络流, 最小割]
categories: 题解（OI/XCPC）
---

## 题目大意

有 $n$ 个小朋友打算投票，同时有 $m$ 对好朋友，每个小朋友有一个一开始的投票意愿。定义冲突数为好朋友之间投票不同的对数加上投票与自己意愿不同的个数。求最小冲突数。

$2 \leqslant n \leqslant 300$

## 题目链接

[【SHOI2007】善意的投票 / 【JLOI2010】冠军调查 - Luogu 2057](https://www.luogu.com.cn/problem/P2057)

<!-- more -->

## 题解

建最小割跑最大流即为答案。

从源点向意愿赞同／反对的小朋友连一条容量为 $1$ 的边，从意愿反对／赞同的小朋友向汇点连一条容量为 $1$ 的边，以上两组边若被割断，则表示投票与意愿相反；每对朋友互连一条容量为 $1$ 的边，割断表示朋友间投票不同，但实际有用的边只有从赞同／反对的人连向反对／赞同的边。

像 `Dinic` 这样的类似工具类的东西，还是写成 `namespace` 比较合适。。。

## 代码

```c++
#include <cstdio>
#include <climits>
#include <queue>
#include <algorithm>
const int MAXN = 305;
struct Edge;
struct Node {
    Edge *e, *curr;
    int level;
} N[MAXN];
struct Edge {
    Node *u, *v;
    Edge *next, *rev;
    int cap, flow;
    Edge(Node *u, Node *v, int cap) : u(u), v(v), cap(cap), flow(0), next(u->e) {}
};
void addEdge(int u, int v, int cap = 1) {
    N[u].e = new Edge(&N[u], &N[v], cap);
    N[v].e = new Edge(&N[v], &N[u], 0);
    N[u].e->rev = N[v].e;
    N[v].e->rev = N[u].e;
}
namespace Dinic {
    bool makeLevelGraph(Node *s, Node *t, int n) {
        for (int i = 0; i < n; i++) N[i].level = 0;
        std::queue<Node *> q;
        q.push(s);
        s->level = 1;
        while (!q.empty()) {
            Node *u = q.front();
            q.pop();
            for (Edge *e = u->e; e; e = e->next) {
                if (e->cap > e->flow && e->v->level == 0) {
                    e->v->level = u->level + 1;
                    if (e->v == t) return true;
                    q.push(e->v);
                }
            }
        }
        return false;
    }
    int findPath(Node *s, Node *t, int limit = INT_MAX) {
        if (s == t) return limit;
        for (Edge *&e = s->curr; e; e = e->next) {
            if (e->cap > e->flow && e->v->level == s->level + 1) {
                int flow = findPath(e->v, t, std::min(limit, e->cap - e->flow));
                if (flow > 0) {
                    e->flow += flow;
                    e->rev->flow -= flow;
                    return flow;
                }
            }
        }
        return 0;
    }
    int solve(int s, int t, int n) {
        int res = 0;
        while (makeLevelGraph(&N[s], &N[t], n)) {
            for (int i = 0; i < n; i++) N[i].curr = N[i].e;
            int flow;
            while ((flow = findPath(&N[s], &N[t])) > 0) res += flow;
        }
        return res;
    }
}
int main() {
    int n, m;
    scanf("%d %d", &n, &m);
    static int vote[MAXN];
    const int s = 0, t = n + 1;
    for (int i = 1; i <= n; i++) {
        scanf("%d", &vote[i]);
        if (vote[i] == 1) addEdge(s, i);
        else addEdge(i, t);
    }
    for (int i = 0; i < m; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        if (vote[u] == 1 && vote[v] == 0) addEdge(u, v);
        if (vote[v] == 1 && vote[u] == 0) addEdge(v, u);
    }
    printf("%d\n", Dinic::solve(s, t, t + 1));
    return 0;
}
```