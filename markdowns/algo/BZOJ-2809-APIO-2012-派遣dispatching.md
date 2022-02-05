---
title: '[APIO 2012] 派遣'
date: 2017-04-17 22:06:26
tags: [DFS序, 主席树]
categories: 题解（OI/XCPC）
---

## 题目大意

有 $n$ 名忍者，每名忍者有一个上级（当然，最上级的那位没有），同时每名忍者有薪水 $s_i$、领导力 $l_i$，现选定一批忍者，这些忍者均是某一个忍者（作为管理者）的下级（管理者可以不选），将获得满意度：管理者领导力$\times$所选忍者个数。求在薪水预算 $m$ 范围内的最大满意度。

$1 \leqslant n \leqslant 100,000$

$1 \leqslant m \leqslant 1,000,000,000$

$1 \leqslant l_i \leqslant 1,000,000,000$

## 题目链接

[【APIO 2012】派遣 - Luogu 1552](https://www.luogu.com.cn/problem/P1552)

<!-- more -->

## 题解

dfs 序 $+$ 主席树。

一个节点为根的子树在 dfs 序上是连续的。对于每一个节点，考虑其子树的区间，要使满意度更大，应该从薪水少的开始选，直到薪水和会超过预算，该操作可以用主席树做到 $O(\log n)$。

对于数据的处理要特别提一下：离散化时，相同的数应化为不同的数。（一开始没有意识到，各种 WA，于是在 COGS 上提了第一道题，找到了一份小的数据才发现问题的。。。不过要在「题目链接」中添加 COGS 的话，就要改所有的博文了。。。）

本题标算好像是左偏树，然而我不会。。。可是主席树常数好大。。。

VW：复杂度能和标算同阶就好啦

## 代码

```c++
#include <cstdio>
#include <algorithm>
// #define DBG
const int MAXN = 100005;
struct Edge;
struct Node {
    Edge *e;
    Node *fa;
    int dfn, rDfn, w;
} N[MAXN];
struct Edge {
    Node *u, *v;
    Edge *next;
    Edge(Node *u, Node *v) : u(u), v(v), next(u->e) {}
};
void addEdge(int u, int v) {
    N[u].e = new Edge(&N[u], &N[v]);
    N[v].e = new Edge(&N[v], &N[u]);
}
struct Ninja {
    int s, l, id;
    bool operator<(const Ninja &another) const {
        return s < another.s;
    }
} a[MAXN];
bool cmp(const Ninja &a, const Ninja &b) {
    return a.id < b.id;
}
int map[MAXN], n;
void discretization() {
    std::sort(a + 1, a + n + 1);
    for (int i = 1; i <= n; i++) map[i] = a[i].s, a[i].s = i;
    std::sort(a + 1, a + n + 1, cmp);
}
struct PSegT {
    struct Node {
        Node *lc, *rc;
        int l, r, cnt;
        long long sum;
        Node(int l, int r, Node *lc = NULL, Node *rc = NULL) : l(l), r(r), lc(lc), rc(rc), cnt((lc ? lc->cnt : 0) + (rc ? rc->cnt : 0)), sum((lc ? lc->sum : 0) + (rc ? rc->sum : 0)) {}
        Node(int l, int r, int cnt) : l(l), r(r), cnt(cnt), sum(cnt * (long long) map[l]), lc(NULL), rc(NULL) {}
        void pushDown() {
            if (lc && rc) return;
            int mid = l + (r - l) / 2;
            if (!lc) lc = new Node(l, mid);
            if (!rc) rc = new Node(mid + 1, r);
        }
        Node *insert(int num) {
            if (num < l || num > r) return this;
            if (num == l && num == r) return new Node(l, r, this->cnt + 1);
            int mid = l + (r - l) / 2;
            pushDown();
            if (num <= mid) return new Node(l, r, lc->insert(num), rc);
            else return new Node(l, r, lc, rc->insert(num));
        }
        int rank() {
            return lc ? lc->cnt : 0;
        }
    } *roots[MAXN];
    int n;
    void build(int a[], int n) {
        this->n = n;
        roots[0] = new Node(1, n);
        for (int i = 1; i <= n; i++) roots[i] = roots[i - 1]->insert(a[i]);
    }
    int query(int l, int r, int m) {
        Node *L = roots[l - 1], *R = roots[r];
        int res = 0;
        while (R->sum - L->sum > m) {
#ifdef DBG
            printf("query in [%d, %d]\n", l, r);
            printf(" L-([%d, %d], cnt = %d, sum = %lld)\n", L->l, L->r, L->cnt, L->sum);
            printf(" R-([%d, %d], cnt = %d, sum = %lld)\n", R->l, R->r, R->cnt, R->sum);
#endif
            L->pushDown();
            R->pushDown();
            long long temp = R->lc->sum - L->lc->sum;
#ifdef DBG
            printf(" temp = %lld, res += %d\n", temp, R->lc->cnt - L->lc->cnt);
#endif
            if (temp <= m) {
                m -= temp;
                res += R->lc->cnt - L->lc->cnt;
                L = L->rc;
                R = R->rc;
            } else {
                L = L->lc;
                R = R->lc;
            }
        }
#ifdef DBG
        printf("query in [%d, %d], return %d\n", l, r, res + R->cnt - L->cnt);
#endif
        return res + R->cnt - L->cnt;
    }
} pst;
int s[MAXN];
void dfs(Node *u, Node *fa = NULL) {
    static int dfsClock = 0;
    u->dfn = ++dfsClock;
    s[dfsClock] = u->w;
    for (Edge *e = u->e; e; e = e->next) {
        if (e->v != fa) dfs(e->v, u);
    }
    u->rDfn = dfsClock;
}
int main() {
    int m;
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= n; i++) {
        int f;
        scanf("%d %d %d", &f, &a[i].s, &a[i].l);
        if (f != 0) addEdge(i, f);
        a[i].id = i;
    }
    discretization();
    for (int i = 1; i <= n; i++) N[i].w = a[i].s;
    dfs(&N[1]);
#ifdef DBG
    for (int i = 1; i <= n; i++) printf("N[%d].dfn = %d\n", i, N[i].dfn);
#endif
    pst.build(s, n);
    long long ans = 0;
    for (int i = 1; i <= n; i++) ans = std::max(ans, 
                                 (long long) pst.query(N[i].dfn, N[i].rDfn, m) * a[i].l);
    printf("%lld\n", ans);
    return 0;
}
```