---
title: '[POJ 1785]Binary Search Heap Construction'
date: 2017-07-05 20:46:55
tags: [Treap]
categories: 题解（OI/XCPC）
---

## 题目大意

以 `label/priority` 的形式给出 $n$ 个节点的值，以 `label` （由小写字母组成）为二分搜索树、`priority` （整数）为大根堆建立一棵 Treap ，并以 `(<left sub-treap><label>/<priority><right sub-treap>)` 的形式输出。多组数据。

$1 \leqslant n \leqslant 50,000$

## 题目链接

[POJ 1785](http://poj.org/problem?id=1785)

<!-- more -->

## 题解

无题解。。。

因为看到一道可持久化 Treap 的题才学的 Treap 。。。

## 代码

```c++
#include <cstdio>
#include <cstring>
#include <climits>
#include <string>
#include <algorithm>
const int MAXN = 50005;
const int MAXL = 105;
struct Pair {
    char key[MAXL];
    int val;
    bool operator<(const Pair &rhs) const {
        return strcmp(key, rhs.key) < 0;
    }
} a[MAXN];
struct Treap {
    struct Node {
        Node *c[2];
        int val;
        char key[MAXL];
        Node() : key(), val(0), c() {}
        Node(char *key, int val) : val(val), c() {
            std::copy(key, key + MAXL, this->key);
        }
        void print() const {
            putchar('(');
            if (c[0]) c[0]->print();
            printf("%s/%d", key, val);
            if (c[1]) c[1]->print();
            putchar(')');
        }
    } *root, _pool[MAXN], *_curr;
    Treap() : root(NULL) {
        init();
    }
    void init() {
        _curr = _pool;
    }
    void build(int n, Pair *a) {
        init();
        root = new (_curr++) Node("", INT_MAX);
        static Node *stack[MAXN];
        stack[0] = root;
        int top = 1;
        for (int i = 0; i < n; i++) {
            int p = top - 1;
            Node *u = new (_curr++) Node(a[i].key, a[i].val);
            while (stack[p]->val < u->val) p--;
            if (p != top - 1) u->c[0] = stack[p + 1];
            stack[p]->c[1] = u;
            top = p + 1;
            stack[top++] = u;
        }
        root = root->c[1];
    }
    void print() {
        root->print();
        puts("");
    }
} treap;
int main() {
    int n;
    while (scanf("%d", &n) && n) {
        for (int i = 0; i < n; i++) scanf(" %[a-z]/%d", a[i].key, &a[i].val);
        std::sort(a, a + n);
        treap.build(n, a);
        treap.print();
    }
    return 0;
}
```