---
title: '[SPOJ 10628] Count on a tree'
date: 2017-04-21 20:56:55
tags: [主席树, LCA]
categories: 题解（OI/XCPC）
---

## 题目大意

给定一棵 $n$ 个节点的树，每个点有一个权值，对于 $m$ 个询问 $(u, \; v, \; k)$，你需要回答 $u \; xor \; lastans$ 和 $v$ 这两个节点间第 $k$ 小的点权。其中 $lastans$ 是上一个询问的答案，初始为 $0$。

$1\leqslant n, \; m \leqslant 100,000$

## 题目链接

[Count on a tree - Luogu 2633](https://www.luogu.com.cn/problem/P2633)

[SPOJ 10628 - COT](http://www.spoj.com/problems/COT/)

<!-- more -->

## 题解

主席树。对于每一个询问，求出 $p = lca(u, \; v)$，然后在线段树 $u + v - p - p.fa$ 上二分即可。

求 lca 用倍增。

## 代码

有烦人的 PE。。。

```c++
#include <cstdio>
#include <climits>
#include <vector>
#include <queue>
#include <algorithm>
const int MAXN = 100005;
const int MAXLOGN = 17;
struct PSegT *null;
struct PSegT {
    PSegT *lc, *rc;
    int cnt;
    PSegT(PSegT *lc, PSegT *rc) : lc(lc), rc(rc), cnt(lc->cnt + rc->cnt) {}
    PSegT(PSegT *lc, PSegT *rc, int cnt) : lc(lc), rc(rc), cnt(cnt) {}
    PSegT *insert(int l, int r, int x) {
        if (l == r) return new PSegT(null, null, cnt + 1);
        else {
            int mid = l + (r - l) / 2;
            if (x <= mid) return new PSegT(lc->insert(l, mid, x), rc);
            else return new PSegT(lc, rc->insert(mid + 1, r, x));
        }
    }
};
struct Node {
    std::vector<Node *> adj;
    Node *fa;
    int dep, w;
    bool vis;
    PSegT *seg;
} N[MAXN];
void addEdge(int u, int v) {
    N[u].adj.push_back(&N[v]);
    N[v].adj.push_back(&N[u]);
}
void init() {
    null = new PSegT(NULL, NULL, 0);
    null->lc = null->rc = null;
}
int n, f[MAXN][MAXLOGN], logn;
void build() {
    N[0].vis = true;
    N[0].seg = null;
    std::queue<Node *> q;
    q.push(&N[1]);
    N[1].vis = true;
    N[1].dep = 1;
    N[1].fa = &N[0];
    while (!q.empty()) {
        Node *u = q.front();
        q.pop();
        u->seg = u->fa->seg->insert(0, INT_MAX, u->w);
        for (Node **p = &u->adj.front(), *v = *p; p <= &u->adj.back(); v = *++p) {
            if (!v->vis) {
                v->vis = true;
                v->dep = u->dep + 1;
                v->fa = u;
                q.push(v);
            }
        }
    }
    while ((1 << (logn + 1)) <= n) logn++;
    f[1][0] = 1;
    for (int i = 2; i <= n; i++) f[i][0] = N[i].fa - N;
    for (int j = 1; j <= logn; j++) {
        for (int i = 1; i <= n; i++) {
            f[i][j] = f[f[i][j - 1]][j - 1];
        }
    }
}
int lca(int u, int v) {
    if (N[u].dep < N[v].dep) std::swap(u, v);
    if (N[u].dep > N[v].dep) {
        for (int i = logn; i >= 0; i--) {
            if (N[f[u][i]].dep >= N[v].dep) u = f[u][i];
        }
    }
    if (u != v) {
        for (int i = logn; i >= 0; i--) {
            if (f[u][i] != f[v][i]) {
                u = f[u][i];
                v = f[v][i];
            }
        }
        return f[u][0];
    }
    return u;
}
int query(int u, int v, int k) {
    int p = lca(u, v);
    PSegT *su = N[u].seg, *sv = N[v].seg, *sp = N[p].seg, *sf = N[p].fa->seg;
    int l = 0, r = INT_MAX;
    while (l < r) {
        int mid = l + (r - l) / 2;
        int s = su->lc->cnt + sv->lc->cnt - sp->lc->cnt - sf->lc->cnt;
        if (k > s) {
            k -= s;
            l = mid + 1;
            su = su->rc;
            sv = sv->rc;
            sp = sp->rc;
            sf = sf->rc;
        } else {
            r = mid;
            su = su->lc;
            sv = sv->lc;
            sp = sp->lc;
            sf = sf->lc;
        }
    }
    return l;
}
int main() {
    int m;
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= n; i++) scanf("%d", &N[i].w);
    for (int i = 1; i < n; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        addEdge(u, v);
    }
    init();
    build();
    int lastAns = 0;
    while (m--) {
        int u, v, k;
        scanf("%d %d %d", &u, &v, &k);
        u ^= lastAns;
        printf(m ? "%d\n" : "%d", lastAns = query(u, v, k));
    }
    return 0;
}
```