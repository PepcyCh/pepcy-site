---
title: k-d树学习笔记
date: 2017-04-29 21:45:32
tags: [k-d树, 模版, 学习笔记]
categories: 学习笔记（OI/XCPC）
---

我学习的博客：[k-d树学习笔记 | Sengxian's Blog](https://blog.sengxian.com/algorithms/k-dimensional-tree)。

## k-d树简介

k-d 树，就是 $k$ 维树，可以看作是数据是多维向量的二叉搜索树。

一维的情况就是一般的二叉搜索树。

一般常用二维情况，求解平面$k$近点对，之后的笔记也以二维为主。

<!-- more -->

## 数据结构的介绍与实现

### 节点

像二叉搜索树一样，每个节点有两个儿子，同时保存自己所存储的数据，一般还保存它和它的子树的数据范围。

```c++
struct Node {
    Node *c[2];
    Point p, r1, r2; // r1为范围左下角，r2为右上角
    Node(Point p) : p(p), r1(p), r2(p) {
        c[0] = c[1] = NULL;
    }
};
```

### 建树

一维时，我们如何比较数据是确定的，然而在多维情况下，比较哪一维是不一定的。一般我们轮流比较每一维，比如二位情况就是先比较 $x$ ，再比较 $y$ ，再比较 $x$ …… 对于每个节点，如果它的儿子是以某一维比较的，那么该节点满足，在这一维上，其左侧的点都比其小，右侧的都比它大。

现在，我们要以一堆点建立 k-d 树还需解决另一个问题，即如何取根节点让两颗子树大小尽量相等。参照二叉搜索树，取中位数为根，可以用 `algorithm` 头文件中的 `std::nth_element(起点指针, 第k位指针, 终点指针, 比较方法)` 方法取得中位数。

关于该方法，它会把范围内第 $k$ 小的数放在「第 $k$ 位指针」上，比它小的乱序置于前面，大的乱序置于后面，复杂度为 $O(n)$。

那么建树方法就可以写出来了。

```c++
static bool cmp1(const Point &a, const Point &b) {
    return a.y < b.y ? a.y < b.y : (a.y == b.y && a.x < b.x);
}
static bool cmp2(const Point &a, const Point &b) {
    return a.x < b.x ? a.x < b.x : (a.x == b.x && a.y < b.y);
}

Node *build(Point *l, Point *r, int d = 0) {
    if (l > r) return NULL;
    if (l == r) return new Node(*l);
    Point *mid = l + (r - l) / 2;
    std::nth_element(l, mid, r + 1, d ? cmp1 : cmp2);
    Node *u = new Node(*mid);
    u->c[0] = build(l, mid - 1, d ^ 1);
    u->c[1] = build(mid + 1, r, d ^ 1);
    u->maintain();
    return u;
}
```

其中 `maintain()` 方法更新点的范围。

```c++
void maintain() {
    if (c[0]) {
        r1.x = std::min(r1.x, c[0]->r1.x);
        r1.y = std::min(r1.y, c[0]->r1.y);
        r2.x = std::max(r2.x, c[0]->r2.x);
        r2.y = std::max(r2.y, c[0]->r2.y);
    }
    if (c[1]) {
        r1.x = std::min(r1.x, c[1]->r1.x);
        r1.y = std::min(r1.y, c[1]->r1.y);
        r2.x = std::max(r2.x, c[1]->r2.x);
        r2.y = std::max(r2.y, c[1]->r2.y);
    }
}
```

### 插入

类比二叉搜索树，从根节点一路比较下去就行了。

```c++
void insert(const Point &p) {
    Node **u = &root;
    int d = 0;
    while (*u) {
        int k = (d ? cmp1(p, (*u)->p) : cmp2(p, (*u)->p)) ^ 1;
        d ^= 1;
        (*u)->r1.x = std::min(p.x, (*u)->r1.x);
        (*u)->r1.y = std::min(p.y, (*u)->r1.y);
        (*u)->r2.x = std::max(p.x, (*u)->r2.x);
        (*u)->r2.y = std::max(p.y, (*u)->r2.y);
        u = &(*u)->c[k];
    }
    *u = new Node(p);
}
```

### 查询最近点

先设答案为正无穷，从根节点开始，计算询问点到两个儿子节点范围矩形的距离（在内部为 $0$），先去小的一方查询，更新答案后，以询问点为中心画圆（欧几里得距离）／矩形（曼哈顿距离），如果与另一个子节点的范围矩形相交，表明另一个子节点的子树中有可能有答案，在该子点中继续查询。

过程中，把答案放在外面，似乎有利于剪枝（放在内部时，BZOJ 2648 就是 TLE。。。）。

```c++
int dist(const Point &p) {
    int res = 0;
    if (p.x < r1.x || r2.x < p.x) res += p.x < r1.x ? r1.x - p.x : p.x - r2.x;
    if (p.y < r1.y || r2.y < p.y) res += p.y < r1.y ? r1.y - p.y : p.y - r2.y;
    return res;
}

void query(const Point &p, int &res) {
    res = std::min(res, ::dist(this->p, p));
    if (!(c[0] || c[1])) return;
    int k = c[0] && c[1] ? c[0]->dist(p) > c[1]->dist(p) : (c[0] ? 0 : 1);
    if (c[k]->dist(p) < res) c[k]->query(p, res);
    if (c[k ^ 1] && c[k ^ 1]->dist(p) < res) c[k ^ 1]->query(p, res);
}
```

外围的 `dist(Point a, Point b)` 方法根据是哪种距离来写。

### 查询$k$远点

用一个优先队列／小根堆保存答案，先放入 $k$ 个 $-1$（只要肯定比答案小就行），之后进行查询。

在更新答案时，队顶元素就是目前的第$k$远。

从根节点开始，计算询问点到两个儿子节点范围内的点的最远距离（就是到范围矩形四个顶点的最远距离），先去大的一方查询，更新答案后，如果到另一个子节点的最大距离比目前答案大，则在另一个子节点中查询。

```c++
long long dist(const Point &p) {
    return std::max(std::max(::dist(p, r1), ::dist(p, r2)), 
                    std::max(::dist(p, Point(r1.x, r2.y)), ::dist(p, Point(r2.x, r1.y))));
}

void query(const Point &p, std::priority_queue<long long, std::vector<long long>, std::greater<long long> > &q) {
    long long d = ::dist(p, this->p);
    if (d > q.top()) q.pop(), q.push(d);
    if (!(c[0] || c[1])) return;
    long long dis[2] = {c[0] ? c[0]->dist(p) : INT_MIN, c[1] ? c[1]->dist(p) : INT_MIN};
    int k = dis[0] < dis[1];
    if (dis[k] > q.top()) c[k]->query(p, q);
    if (c[k ^ 1] && dis[k ^ 1] > q.top()) c[k ^ 1]->query(p, q);
}
```

## 模版题

曼哈顿距离最近点：[BZOJ 2648](http://www.lydsy.com/JudgeOnline/problem.php?id=2648) （[题解](http://pepcy.cf/BZOJ-2648-SJY摆棋子/)）

欧几里得距离$k$远点对：[BZOJ 4520](http://www.lydsy.com/JudgeOnline/problem.php?id=4520) （[题解](http://pepcy.cf/BZOJ-4520-CQOI-2016-K远点对/)）