---
title: '[LibreOJ 6109][2017 山东二轮集训 Day4]增添'
date: 2017-07-05 21:41:57
tags: [可持久化Treap, Treap]
categories: 学习笔记（OI/XCPC）
---

## 题目大意

有一个长度为 $n$ 的序列，要求支持三种操作（$m$ 次）：

* `1 l r x` 将 $[l, r]$ 中的数增加 $x$。
* `2 l r x` 用 $[l, l+x]$ 中的数对应替换 $[r, r+x]$ 中的数。
* `3 l r` 求 $[l, r]$ 中所有数的和。

$1 \leqslant n, m \leqslant 100,000$

$x \leqslant 10,000$

## 题目链接

[LibreOJ 6109](https://loj.ac/problem/6109)

<!-- more -->

## 题解

可持久化 Treap。

用可持久化 Treap 是因为，操作二要求用 Treap 在裂开时只是「假装」裂开，在合并时只是「假装」合并，用可持化的历史版本即可做到。

我因为所有操作都开新节点了（包括 `pushDown()` 、`add()`），所以用了 $250$ 倍空间。。。（LOJ 上第二大。。。）

## 代码

```c++
#include <cstdio>
#include <cstdlib>
#include <algorithm>
const int MAXN = 100005;
template <typename T, size_t SIZE>
struct MemoryPool {
    char mem[sizeof (T) * SIZE], *top;
    MemoryPool() : top(mem) {}
    void *alloc() {
        char *res = top;
        top += sizeof (T);
        return (void *) res;
    }
};
struct Treap {
    struct Node {
        int val, tag, size;
        long long sum;
        Node *lc, *rc;
        static const int POOL_SIZE = 250 * MAXN;
        static MemoryPool<Node, POOL_SIZE> pool;
        Node() {}
        Node(Node *lc, Node *rc, int val)
            : lc(lc), rc(rc), val(val), 
              sum((lc ? lc->sum : 0) + (rc ? rc->sum : 0) + val), tag(0), 
              size((lc ? lc->size : 0) + (rc ? rc->size : 0) + 1) {}
        Node(Node *lc, Node *rc, int val, long long sum, int tag)
            : lc(lc), rc(rc), val(val), sum(sum), tag(tag), 
              size((lc ? lc->size : 0) + 1 + (rc ? rc->size : 0)) {}
        void *operator new(size_t) {
            return pool.alloc();
        }
        Node *add(int d) {
            return new Node(lc, rc, val + d, sum + (long long) size * d, tag + d);
        }
        Node *pushDown() {
            if (tag) return new Node(lc ? lc->add(tag) : NULL, 
                                     rc ? rc->add(tag) : NULL, val);
            else return this;
        }
    } *root;
    Treap() : root(NULL) {}
    static int size(const Node *u) {
        return u ? u->size : 0;
    }
    Node *merge(Node *a, Node *b) {
        if (!a) return b;
        if (!b) return a;
        if (rand() % (a->size + b->size) < a->size) {
            a = a->pushDown();
            return new Node(a->lc, merge(a->rc, b), a->val);
        } else {
            b = b->pushDown();
            return new Node(merge(a, b->lc), b->rc, b->val);
        }
    }
    std::pair<Node *, Node *> split(Node *u, int pos) {
        std::pair<Node *, Node *> res(NULL, NULL);
        if (!u) return res;
        u = u->pushDown();
        if (size(u->lc) >= pos) {
            res = split(u->lc, pos);
            res.second = new Node(res.second, u->rc, u->val);
        } else {
            res = split(u->rc, pos - size(u->lc) - 1);
            res.first = new Node(u->lc, res.first, u->val);
        }
        return res;
    }
    Node *build(int *l, int *r) {
        if (l > r) return NULL;
        int *mid = l + (r - l) / 2;
        return new Node(build(l, mid - 1), build(mid + 1, r), *mid);
    }
    void add(int l, int r, int d) {
        std::pair<Node *, Node *> L = split(root, l - 1);
        std::pair<Node *, Node *> R = split(L.second, r - l + 1);
        R.first = R.first->add(d);
        root = merge(merge(L.first, R.first), R.second);
    }
    void copy(int l, int r, int len) {
        std::pair<Node *, Node *> L = split(root, l - 1);
        std::pair<Node *, Node *> R = split(L.second, len + 1);
        Node *target = R.first;
        L = split(root, r - 1);
        R = split(L.second, len + 1);
        root = merge(merge(L.first, target), R.second);
    }
    long long query(int l, int r) {
        std::pair<Node *, Node *> L = split(root, l - 1);
        std::pair<Node *, Node *> R = split(L.second, r - l + 1);
        return R.first->sum;
    }
} treap;
MemoryPool<Treap::Node, Treap::Node::POOL_SIZE> Treap::Node::pool;
int main() {
    int n, q;
    scanf("%d %d", &n, &q);
    static int a[MAXN];
    for (int i = 0; i < n; i++) scanf("%d", &a[i]);
    treap.root = treap.build(a, a + n);
    while (q--) {
        int op, l, r;
        scanf("%d %d %d", &op, &l, &r);
        if (op == 1) {
            int x;
            scanf("%d", &x);
            treap.add(l, r, x);
        }
        if (op == 2) {
            int x;
            scanf("%d", &x);
            treap.copy(l, r, x);
        }
        if (op == 3) printf("%lld\n", treap.query(l, r));
    }
    return 0;
}
```