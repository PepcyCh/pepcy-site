---
title: '[国家集训队] happiness'
date: 2017-04-10 21:12:35
tags: [网络流, 最小割, 最大权闭合图]
categories: 题解（OI/XCPC）
---

## 题目大意

班级可视为 $n \times m$ 的矩阵，现进行文理分科。

* 位置 $[i, \; j]$ 的人选文科会获得满意值 $art[i, \; j]$，选理科会获得 $science[i, \; j]$。
* 位置 $[i, \; j]$ 的人及其下面的人均选择文科会额外获得满意值 $artColumn[i, \; j]$，均选择理科会额外获得 $scienceColumn[i, \; j]$。
* 位置 $[i, \; j]$的人及其右面的人均选择文科会额外获得满意值 $artRow[i, \; j]$，均选择理科会额外获得 $scienceRow[i, \; j]$。

求最大满意值。

$1 \leqslant n, \; m \leqslant 100$

其他数据 $\leqslant 5,000$

（直接从文理分科复制过来改了改。。。）

## 题目链接

[【国家集训队】happiness - Luogu 1646](https://www.luogu.com.cn/problem/P1646)

<!-- more -->

## 题解

最大权闭合图。

先让所有人选择文科。

每个人拆为 $5$ 个节点，分别表示改选理科满意值的变化量、改选后文科列额外满意值的损失量、改选后理科列额外满意值的增加量、改选后文科行额外满意值的损失量、改选后理科行额外满意值的增加量，分别记为节点种类 $1 \sim 5$。对于每一个节点 $1$，向其自己与下面的人的节点 $2$ 连边，向其自己与右面的人的节点 $4$；从其自己与下面的人的节点 $3$、其自己与右面的人的节点 $5$ 向该节点的节点 $1$ 连边。

（继续复制。。。）

文理分科的姊妹篇吗（说来这个的题号在文理分科前啊。。。）。。。

## 代码

代码中的行 R(row) 和列 C(column) 写反了。。。写之前还专门查的。。。

```c++
#include <cstdio>
#include <climits>
#include <queue>
#include <algorithm>
const int MAXN = 105;
struct Edge;
struct Node {
    Edge *e, *curr;
    int level;
} N[MAXN * MAXN * 5];
struct Edge {
    Node *u, *v;
    Edge *next, *rev;
    int cap, flow;
    Edge(Node *u, Node *v, int cap) : u(u), v(v), cap(cap), flow(0), next(u->e) {}
};
void addEdge(int u, int v, int cap) {
    N[u].e = new Edge(&N[u], &N[v], cap);
    N[v].e = new Edge(&N[v], &N[u], 0);
    N[u].e->rev = N[v].e;
    N[v].e->rev = N[u].e;
}
struct Dinic {
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
    int operator()(int s, int t, int n) {
        int res = 0;
        while (makeLevelGraph(&N[s], &N[t], n)) {
            for (int i = 0; i < n; i++) N[i].curr = N[i].e;
            int flow;
            while ((flow = findPath(&N[s], &N[t])) > 0) res += flow;
        }
        return res;
    }
} dinic;
int main() {
    int n, m;
    scanf("%d %d", &n, &m);
    int sum = 0;
    static int a[MAXN][MAXN], b[MAXN][MAXN], aR[MAXN][MAXN], bR[MAXN][MAXN], aC[MAXN][MAXN], bC[MAXN][MAXN];
    for (int i = 1; i <= n; i++) for (int j = 1; j <= m; j++) 
        scanf("%d", &a[i][j]), sum += a[i][j];
    for (int i = 1; i <= n; i++) for (int j = 1; j <= m; j++) 
        scanf("%d", &b[i][j]);
    for (int i = 1; i < n; i++) for (int j = 1; j <= m; j++) 
        scanf("%d", &aR[i][j]), sum += aR[i][j];
    for (int i = 1; i < n; i++) for (int j = 1; j <= m; j++) 
        scanf("%d", &bR[i][j]);
    for (int i = 1; i <= n; i++) for (int j = 1; j < m; j++) 
        scanf("%d", &aC[i][j]), sum += aC[i][j];
    for (int i = 1; i <= n; i++) for (int j = 1; j < m; j++) 
        scanf("%d", &bC[i][j]);
    int index = 0;
    static int id[MAXN][MAXN][5];
    for (int i = 1; i <= n; i++) for (int j = 1; j <= m; j++) for (int k = 0; k < 5; k++) 
        id[i][j][k] = ++index;
    const int s = 0, t = n * m * 5 + 1;
    for (int i = 1; i <= n; i++) for (int j = 1; j <= m; j++) {
        int x = b[i][j] - a[i][j];
        if (x > 0) addEdge(s, id[i][j][0], x), sum += x;
        else addEdge(id[i][j][0], t, -x);
        if (i != n) {
            addEdge(id[i][j][1], t, aR[i][j]);
            addEdge(s, id[i][j][2], bR[i][j]), sum += bR[i][j];
            addEdge(id[i][j][2], id[i][j][0], INT_MAX);
            addEdge(id[i][j][2], id[i + 1][j][0], INT_MAX);
            addEdge(id[i][j][0], id[i][j][1], INT_MAX);
            addEdge(id[i + 1][j][0], id[i][j][1], INT_MAX);
        }
        if (j != m) {
            addEdge(id[i][j][3], t, aC[i][j]);
            addEdge(s, id[i][j][4], bC[i][j]), sum += bC[i][j];
            addEdge(id[i][j][4], id[i][j][0], INT_MAX);
            addEdge(id[i][j][4], id[i][j + 1][0], INT_MAX);
            addEdge(id[i][j][0], id[i][j][3], INT_MAX);
            addEdge(id[i][j + 1][0], id[i][j][3], INT_MAX);
        }
    }
    printf("%d\n", sum - dinic(s, t, t + 1));
    return 0;
}
```