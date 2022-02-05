---
title: '[HDU 5909] Tree Cutting'
date: 2018-10-04 21:12:06
tags: [DP, 点分治]
categories: 学习笔记（OI/XCPC）
---

## 题目大意

给定一棵 $n$ 个节点的树，每个点有一个权值 $v_i$。对于该树的任意一个非空子树，定义其权值为 $v_1 \oplus v_2 \oplus \cdots \oplus v_m$ （$v_i$ 为子树上的点权）。现给出 $m = 2^p$，对于 $[0, m)$ 内的每一个数 $k$，求有多少个子树的权值为 $k$。

$T$ 组数据。

$1 \leq T \leq 10$

$1 \leq n \leq 1,000$

$1 \leq m \leq 2^{10}$

$0 \leq v_i < m$

## 题目链接

[HDU 5909](http://acm.hdu.edu.cn/showproblem.php?pid=5909)

<!--more-->

## 题解

点分治 + DP。

对于一条链上的情况，可以很快想到一个简单粗暴的 DP：定义 $f(u, i)$ 表示计算到点 $u$ 时数 $i$ 的答案。但放在树上合并子树时存在困难。

考虑点分治，每次考虑经过重心的链的答案。在 dfs 计算完一颗子树后，把当前的 DP 信息直接与该子树的兄弟合并，并用合并后的信息向下更新，这样每次都是合并多个数与一个数的情况，与链状时相同。之后再由低向上更新即可。

类似题目：[2017-CCPC-杭州-E-Master of Subgraph](http://acm.hdu.edu.cn/showproblem.php?pid=6268)

## 代码

```c++
#include <bits/stdc++.h>

const int MAXN = 1005;
const int MAXM = 1050;
const int MOD = 1000000007;

inline int add(int &x, int d) {
    x += d;
    x >= MOD ? x -= MOD : 0;
}

struct Edge;
struct Node {
    Edge *e;
    int val, f[MAXM], size, max;
    bool solved;
} N[MAXN];

struct Edge {
    Node *u, *v;
    Edge *next;

    Edge() {}
    Edge(Node *u, Node *v) : u(u), v(v), next(u->e) {}
} _pool[MAXN << 1], *_curr;

void addEdge(int u, int v) {
    N[u].e = new (_curr++) Edge(&N[u], &N[v]);
    N[v].e = new (_curr++) Edge(&N[v], &N[u]);
}

void dfs(Node *u, Node *fa, Node **a, int &p) {
    u->size = 1;
    u->max = 0;
    a[p++] = u;
    for (Edge *e = u->e; e; e = e->next) if (!e->v->solved && e->v != fa) {
        dfs(e->v, u, a, p);
        u->size += e->v->size;
        u->max = std::max(u->max, e->v->size);
    }
}

Node *center(Node *s) {
    static Node *a[MAXN];
    int p = 0;
    dfs(s, NULL, a, p);

    Node *res = NULL;
    for (int i = 0; i < a[0]->size; i++) {
        a[i]->max = std::max(a[i]->max, s->size - a[i]->size);
        if (!res || res->max > a[i]->max) res = a[i];
    }
    return res;
}

void dfs(Node *u, Node *fa, int m) {
    for (Edge *e = u->e; e; e = e->next) if (!e->v->solved && e->v != fa) {
        std::fill(e->v->f, e->v->f + m, 0);
        for (int i = 0; i < m; i++) if (u->f[i]) add(e->v->f[i ^ e->v->val], u->f[i]);
        dfs(e->v, u, m);
        for (int i = 0; i < m; i++) if (e->v->f[i]) add(u->f[i], e->v->f[i]);
    }
}

void calc(Node *u, int *ans, int m) {
    std::fill(u->f, u->f + m, 0);
    u->f[u->val] = 1;
    dfs(u, NULL, m);
    for (int i = 0; i < m; i++) if (u->f[i]) add(ans[i], u->f[i]);
}

void solve(int *ans, int m) {
    static std::stack<Node *> s;
    s.push(&N[1]);

    std::fill(ans, ans + m, 0);

    while (!s.empty()) {
        Node *u = s.top();
        s.pop();

        Node *root = center(u);
        root->solved = true;

        calc(root, ans, m);

        for (Edge *e = root->e; e; e = e->next) if (!e->v->solved) s.push(e->v);
    }
}

void init(int n) {
    _curr = _pool;
    for (int i = 1; i <= n; i++) {
        N[i].e = NULL;
        N[i].solved = false;
    }
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n, m;
        scanf("%d %d", &n, &m);
        init(n);

        for (int i = 1; i <= n; i++) scanf("%d", &N[i].val);
        for (int i = 1, u, v; i < n; i++) {
            scanf("%d %d", &u, &v);
            addEdge(u, v);
        }

        static int ans[MAXM];
        solve(ans, m);
        for (int i = 0; i < m; i++) printf("%d%c", ans[i], " \n"[i == m - 1]);
    }

    return 0;
}
```