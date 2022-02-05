---
title: Dinic 算法整理笔记
date: 2017-03-31 21:37:54
tags: [学习笔记, 模版, 网络流]
categories: 学习笔记（OI/XCPC）
---

## 算法介绍

Dinic 算法是一种用于解决网络流问题的算法，算法复杂度（上界）为 $O(n^2m)$ ，实际要比这个式子好得多（比如用 Dinic 算二分图时的复杂度是 $O(m\sqrt{n})$ ）。

Dinic 算法通过对残量网络建立层次图，并在层次图上不断寻找增广路来算出最大流。

**残量网络**：原图及其反向边构成的图。

**反向边**：与原边反向的边，原图上的边的反向边的容量为 $0$ ，每一条边在残量网络上均有其反向边。

**层次图**：只保留相邻层次之间的边，且不考虑已达到满流的边的图。

**层次**：可以视作到源点的距离分类。

**增广路**：残量网络上一条从源点到汇点的边，其所有边中的最小容量为其增广容量。

**当前边优化**：建立层次图以后，若某个点有一条边已经增广过了，则这条边在当前层次图之后的增广中不会再用到。

<!-- more -->

## 实现的部分

对于边与点，我们这么储存（链式前向星式）：

```c++
struct Edge;
struct Node {
    Edge *firstEdge, *currentEdge;
    int level;
} N[MAXN];
struct Edge {
    Node *from, *to;
    Edge *next, *reversedEdge;
    int capacity, flow;
    Edge(Node *from, Node *to, int capacity) : from(from), to(to), capacity(capacity), flow(0), next(from->firstEdge) {}
};
```

当图比较稠密时，可以考虑用 `std::vector` 存图，说法来自知乎。。。亲测差不多的样子。。。

## 模板题及其代码

模板题：[最大流 - LibreOJ 101](https://loj.ac/problem/101)

```c++
#include <cstdio>
#include <climits>
#include <queue>
#include <vector>
#include <algorithm>

const int MAXN = 105;
const int MAXM = 5005;

struct Edge;
struct Node;

struct Node {
    std::vector<Edge> e;
    Edge *curr;
    int level;
} N[MAXN];

struct Edge {
    Node *u,*v;
    int cap, flow, rev;

    Edge(Node *u, Node *v, int cap, int rev) : u(u), v(v), cap(cap), flow(0), rev(rev) {}
};

void addEdge(int u, int v, int cap) {
    N[u].e.emplace_back(&N[u], &N[v], cap, N[v].e.size());
    N[v].e.emplace_back(&N[v], &N[u], 0, N[u].e.size() - 1);
}

namespace Dinic {
    bool level(Node *s, Node *t, int n) {
        for (int i = 1; i <= n; i++) N[i].level = 0;
        static std::queue<Node *> q;
        q.push(s);
        s->level = 1;
        while (!q.empty()) {
            Node *u = q.front();
            q.pop();

            for (Edge *e = &u->e.front(); e <= &u->e.back(); e++) {
                if (e->cap > e->flow && e->v->level == 0) {
                    e->v->level = u->level + 1;
                    q.push(e->v);
                }
            }
        }
        return t->level;
    }

    int findPath(Node *u, Node *t, int limit = INT_MAX) {
        if (u == t) return limit;
        int res = 0;
        for (Edge *&e = u->curr; e <= &u->e.back(); e++) {
            if (e->cap > e->flow && e->v->level == u->level + 1) {
                int flow = findPath(e->v, t, std::min(limit, e->cap - e->flow));
                if (flow > 0) {
                    e->flow += flow;
                    e->v->e[e->rev].flow -= flow;
                    limit -= flow;
                    res += flow;
                    if (limit <= 0) return res;
                } else e->v->level = -1;
            }
        }
        return res;
    }

    long long solve(int s, int t, int n) {
        long long res = 0;
        while (level(&N[s], &N[t], n)) {
            for (int i = 1; i <= n; i++) N[i].curr = &N[i].e.front();
            long long flow;
            while ((flow = findPath(&N[s], &N[t])) > 0) res += flow;
        }
        return res;
    }
}

int main() {
    int n, m, s, t;
    scanf("%d %d %d %d", &n, &m, &s, &t);

    for (int i = 0, u, v, w; i < m; i++) {
        scanf("%d %d %d", &u, &v, &w);
        addEdge(u, v, w);
    }

    printf("%lld\n", Dinic::solve(s, t, n));

    return 0;
}
```