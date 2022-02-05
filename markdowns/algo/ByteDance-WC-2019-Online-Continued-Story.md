---
title: '[ByteDance WC 2019 Online] Continued Story'
date: 2018-12-01 20:30:39
tags: [博弈, Hackenbush]
categories: 题解（OI/XCPC）
---

> 2019 字节跳动冬令营网络赛 C 题。

## 题目大意

给定一棵 $n$ 个节点的带权有根树，两人轮流操作，每次操作使某条边的边权减一，若边权减至 $0$，则删去改边，并删去不含根的连通块，判断先手是否必胜，并输出先手若要胜利，所有第一步可能的决策。

$T$ 组数据。

$1 \leq n \leq 10^6$

$1 \leq w_i \leq 10^9$

$\sum n \leq 10^6$

## 题目链接

[ByteDance WC 2019 Online - C](https://ac.nowcoder.com/acm/contest/296/C)

<!-- more -->

## 题解

现学的 [Hackenbush 博弈](http://pepcy.cf/Hackenbush-博弈学习笔记/)。

计算 SG 函数：

$$
\begin{align}
SG(u) &= f(v_1) \oplus f(v_2) \oplus \cdots \oplus f(v_m) \\
f(v) &= \begin{cases}
SG(v) + 1 ~ &, w_{E(u, v)} = 1 \\
SG(v) ~ &, w_{E(u, v)} \text{ is even} \\
SG(v) \oplus 1 ~ &, w_{E(u, v)} > 1 \text{ and } w_{E(u, v)} \text{ is odd} 
\end{cases}
\end{align}
$$

如何输出方案：

我们从根节点开始 dfs，同时传递一个变量 $need$（代码中为 `val`）表示需要用该子树进行一次操作凑出的 SG 函数值，初始为 $0$。对于每一个点，枚举与它儿子相连的每一条边，可以分情况计算出该儿子对应子树的 $need$ 的值（$other$ 为其余部分的 SG 值，$u, v$ 分别表示父节点、子节点）：

* 边权为 $1$：$(need_v + 1) \oplus other = need_u$
* 边权大于 $1$ 且为奇数：$need_v \oplus 1 \oplus other = need_u$
* 边权为偶数：$need_v \oplus other = need_u$

同时，可以考虑若操作该边是否能得到该点的 $need$ 值：

* 边权为 $1$：$other = need_u$ 则可以
* 边权为 $2$：$(SG(v) + 1) \oplus other = need_u$ 则可以
* 边权大于 $2$：$SG(v) \oplus 1 = need_u$ 则可以

## 代码

吐槽1：比赛时没有给异或运算加括号，于是输给了运算符优先级。。。QAQ

吐槽2：代码中的 `MAXN = 1000105` 原本是 `MAXN = 1000005`，但发生了迷之「内部错误」。

```c++
#include <cstdio>
#include <vector>
#include <algorithm>
 
const int MAXN = 1000105;
 
struct Edge;
 
struct Node {
    Edge *e;
    int sg;
} N[MAXN];
 
struct Edge {
    Node *u, *v;
    Edge *next;
    int w;
 
    Edge() {}
    Edge(Node *u, Node *v, int w) : u(u), v(v), w(w), next(u->e) {}
} _pool[MAXN], *_curr;
 
void addEdge(int f, int s, int w) {
    N[f].e = new (_curr++) Edge(&N[f], &N[s], w);
}
 
void dfs(Node *u) {
    u->sg = 0;
    for (Edge *e = u->e; e; e = e->next) {
        dfs(e->v);
 
        if (e->w == 1) u->sg ^= (e->v->sg + 1);
        else if (e->w % 2 == 0) u->sg ^= e->v->sg;
        else u->sg ^= (e->v->sg ^ 1);
    }
}
 
std::vector<int> ans;
void dfs(Node *u, int val) {
    if (val < 0) return;
    for (Edge *e = u->e; e; e = e->next) {
        int other;
        if (e->w == 1) other = u->sg ^ (e->v->sg + 1);
        else if (e->w % 2 == 0) other = u->sg ^ e->v->sg;
        else other = u->sg ^ e->v->sg ^ 1;
 
        if (other == val && e->w == 1) ans.push_back(e->v - N);
        else if (((e->v->sg + 1) ^ other) == val && e->w == 2) ans.push_back(e->v - N);
        else if (val == (u->sg ^ 1) && e->w > 2) ans.push_back(e->v - N);
        
        if (e->w == 1) dfs(e->v, (val ^ other) - 1);
        else if (e->w % 2 == 0) dfs(e->v, val ^ other);
        else dfs(e->v, val ^ other ^ 1);
    }
}
 
void init(int n) {
    _curr = _pool;
    for (int i = 1; i <= n; i++) N[i].e = NULL;
}
 
int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);
        init(n);
 
        for (int i = 2, f, w; i <= n; i++) {
            scanf("%d %d", &f, &w);
            addEdge(f, i, w);
        }
 
        dfs(&N[1]);
        ans.clear();
        dfs(&N[1], 0);
        
        std::sort(ans.begin(), ans.end());
        printf("%d\n", ans.size());
        for (int i : ans) printf("%d ", i);
        puts("");
    }
     
    return 0;
}
```