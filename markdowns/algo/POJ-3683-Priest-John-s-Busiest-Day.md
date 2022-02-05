---
title: '[POJ 3683] Priest John''s Busiest Day'
date: 2017-04-12 21:26:57
tags: [2SAT, 模版]
categories: 题解（OI/XCPC）
---

## 题目大意

给定 $n$ 对新人婚礼的时间范围（时间以 `hh:mm` 的形式给出）及婚礼时长，要求婚礼要么在时间范围的一开始举行，要么在最结尾时举行；同一时间只能举行一场婚礼。求是否可以举行所有的婚礼，可行的话输出合法方案。

$1 \leqslant n \leqslant 1,000$

## 题目链接

[POJ 3683](http://poj.org/problem?id=3683)

<!-- more -->

## 题解

带输出方案的 2SAT，做一个板子吧。。。

判断可行的部分就不多说了，当两个时间重叠时，变量关系是不同时选择。

输出方案时，对图 Tarjan 缩点，反向连边，拓扑排序（当一个点被标记为选时，对应的点标记为不选并传递标记），最后按标记输出。

## 代码

用静态分配内存 `new(p++)` 神技，把 TLE 的代码（时限 2s）拉到了 172ms。。。

```c++
#include <cstdio>
#include <stack>
#include <algorithm>
#include <new>
const int MAXN = 1005;
const int MAXM = 2000005;
struct Edge;
struct Node {
    Edge *e;
    int dfn, low, belong;
    bool ins;
} N[MAXN << 1];
struct Edge {
    Node *u, *v;
    Edge *next;
    Edge() {}
    Edge(Node *u, Node *v) : u(u), v(v), next(u->e) {}
} _pool1[MAXM], *_cur1 = _pool1;
void addEdge(int u, int v) {
    N[u].e = new (_cur1++) Edge(&N[u], &N[v]);
}
int getV(int x, int k) {
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
            if (N[getV(i, 0)].belong == N[getV(i, 1)].belong) return false;
        return true;
    }
    bool operator()(int n) {
        sccCnt = dfsClock = 0;
        for (int i = 1; i <= n << 1; i++) if (N[i].dfn == 0) tarjan(&N[i]);
        return check(n);
    }
} ts;
struct Pair {
    int s, t;
    Pair() {}
    Pair(int s, int t) : s(s), t(t) {}
    void print() {
        printf("%.2d:%.2d %.2d:%.2d\n", s / 60, s % 60, t / 60, t % 60);
    }
} T[MAXN << 1];
bool cross(const Pair &a, const Pair &b) {
    return !((a.s >= b.t) || (b.s >= a.t));
}
void build(int n) {
    for (int i = 1; i <= n; i++) for (int j = i + 1; j <= n; j++) {
        for (int I = 0; I < 2; I++) for (int J = 0; J < 2; J++) {
            if (cross(T[getV(i, I)], T[getV(j, J)])) {
                addEdge(getV(i, I), getV(j, J ^ 1));
                addEdge(getV(j, J), getV(i, I ^ 1));
            }
        }
    }
}
struct EdgeC;
struct NodeC {
    EdgeC *e;
    NodeC *opp;
    int deg, mark;
    NodeC() : mark(-1) {}
} NC[MAXN << 1];
struct EdgeC {
    NodeC *u, *v;
    EdgeC *next;
    EdgeC() {}
    EdgeC(NodeC *u, NodeC *v) : u(u), v(v), next(u->e) {}
} _pool2[MAXM], *_cur2 = _pool2;
void addEdgeC(int u, int v) {
    NC[u].e = new (_cur2++) EdgeC(&NC[u], &NC[v]);
    NC[v].deg++;
}
void rebuild(int n) {
    for (int i = 1; i <= n; i++) {
        for (Edge *e = N[i].e; e; e = e->next) {
            if (e->u->belong != e->v->belong) addEdgeC(e->v->belong, e->u->belong);
        }
    }
}
struct TopoSort {
    std::stack<NodeC *> s;
    void dfs(NodeC *u) {
        if (u->mark != -1) return;
        u->mark = 0;
        for (EdgeC *e; e; e = e->next) dfs(e->v);
    }
    void operator()(int n) {
        for (int i = 1; i <= n; i++) if (NC[i].deg == 0) s.push(&NC[i]);
        while (!s.empty()) {
            NodeC *u = s.top();
            s.pop();
            // printf(""); //加注释本机上会段错误，POJ上无影响。。。
            if (u->mark != -1) continue;
            u->mark = 1;
            dfs(u->opp);
            for (EdgeC *e = u->e; e; e = e->next) {
                e->v->deg--;
                if (e->v->deg == 0) s.push(e->v);
            }
        }
    }
} topoSort;
int main() {
    int n;
    scanf("%d", &n);
    for (int i = 1; i <= n; i++) {
        int sh, sm, th, tm, d;
        scanf("%d:%d %d:%d %d", &sh, &sm, &th, &tm, &d);
        sh = sh * 60 + sm;
        th = th * 60 + tm;
        T[getV(i, 0)] = Pair(sh, sh + d);
        T[getV(i, 1)] = Pair(th - d, th);
    }
    build(n);
    if (!ts(n)) {
        puts("NO");
        return 0;
    }
    puts("YES");
    rebuild(n << 1);
    for (int i = 1; i <= n; i++) {
        NC[N[getV(i, 0)].belong].opp = &NC[N[getV(i, 1)].belong];
        NC[N[getV(i, 1)].belong].opp = &NC[N[getV(i, 0)].belong];
    }
    topoSort(ts.sccCnt);
    for (int i = 1; i <= n; i++) {
        int x = NC[N[getV(i, 0)].belong].mark;
        T[getV(i, x ^ 1)].print();
    }
    return 0;
}
```