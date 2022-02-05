---
title: '[POJ 3580]SuperMemo'
date: 2017-07-05 21:01:10
tags: [Treap]
categories: 题解（OI/XCPC）
---

## 题目大意

给定一个长为 $n$ 的整数列，并给出 $m$ 次操作。操作如下：

* `ADD x y D` ：区间 $[x, y]$ 加 $D$。
* `REVERSE x y` ：区间 $[x, y]$ 翻转。
* `REVOLVE x y T` ：区间 $[x, y]$ 循环向右移动 $T$ 次。
* `INSERT x P` ：在第 $x$ 个数后插入 $P$。
* `DELETE x` ：删除第 $x$ 个数。
* `MIN x y` ：询问区间 $[x, y]$ 的最小值。

$1 \leqslant n, m \leqslant 100,000$

## 题目链接

[POJ 3580](http://poj.org/problem?id=3580)

<!-- more -->

## 题解

Treap（其实 Splay 也可以）。

因为是为了可持久化 Treap 而学的 Treap，所以是 $split-merge$ 式的。

其余操作不说了，对于 `revolve` 操作，Treap 直接裂开合并即可，Splay 用 $3$ 次翻转。

我的实现其实是「伪·无旋式 Treap」，因为在插入时旋转了。如果不用随机权值小根堆，而是随机合并，就变成了「真·无旋式 Treap」。

## 代码

本来有个 `srand(2333)` ，但加上就 WA 了。。。

以及析构器应该是不需要的，毕竟一次只会删除一个节点。

```c++
#include <cstdio>
#include <cstdlib>
#include <climits>
#include <algorithm>
const int MAXN = 100005;
template <typename T, size_t SIZE>
struct MemoryPool {
    char mem[sizeof (T) * SIZE], *del[SIZE], *memTop, **delTop;
    MemoryPool() : memTop(mem), delTop(del) {}
    void *alloc() {
        if (delTop != del) return (void *) *--delTop;
        char *res = memTop;
        memTop += sizeof (T);
        return (void *) res;
    }
    void free(void *p) {
        *delTop++ = (char *) p;
    }
};
struct Treap {
    struct Node {
        int val, min, tag, key, size;
        bool rev;
        Node *c[2];
        static MemoryPool<Node, MAXN << 1> pool;
        Node() {}
        Node(int val, int key = rand() & ~(1u << 31)) 
            : val(val), key(key), min(val), size(1), tag(0), rev(false), c() {}
        ~Node() {
            if (c[0]) delete c[0];
            if (c[1]) delete c[1];
        }
        void *operator new(size_t) {
            return pool.alloc();
        }
        void operator delete(void *p) {
            pool.free(p);
        }
        void reverse() {
            std::swap(c[0], c[1]);
            rev ^= 1;
        }
        void add(int d) {
            min += d;
            val += d;
            tag += d;
        }
        void pushDown() {
            if (rev) {
                if (c[0]) c[0]->reverse();
                if (c[1]) c[1]->reverse();
                rev = false;
            }
            if (tag) {
                if (c[0]) c[0]->add(tag);
                if (c[1]) c[1]->add(tag);
                tag = 0;
            }
        }
        void maintain() {
            min = std::min(val, std::min(c[0] ? c[0]->min : INT_MAX, 
                                         c[1] ? c[1]->min : INT_MAX));
            size = (c[0] ? c[0]->size : 0) + 1 + (c[1] ? c[1]->size : 0);
        }
    } *root;
    Treap() : root(NULL) {}
    static int size(const Node *u) {
        return u ? u->size : 0;
    }
    Node *merge(Node *a, Node *b) {
        if (!a) return b;
        if (!b) return a;
        if (a->key < b->key) {
            a->pushDown();
            a->c[1] = merge(a->c[1], b);
            a->maintain();
            return a;
        } else {
            b->pushDown();
            b->c[0] = merge(a, b->c[0]);
            b->maintain();
            return b;
        }
    }
    std::pair<Node *, Node *> split(Node *u, int pos) {
        std::pair<Node *, Node *> res(NULL, NULL);
        if (!u) return res;
        u->pushDown();
        if (size(u->c[0]) >= pos) {
            res = split(u->c[0], pos);
            u->c[0] = res.second;
            u->maintain();
            res.second = u;
        } else {
            res = split(u->c[1], pos - size(u->c[0]) - 1);
            u->c[1] = res.first;
            u->maintain();
            res.first = u;
        }
        return res;
    }
    void build(int n, int *a) {
        static Node *stack[MAXN];
        stack[0] = root = new Node(0, INT_MIN);
        int top = 1;
        for (int i = 0; i < n; i++) {
            int p = top - 1;
            Node *u = new Node(a[i]);
            while (stack[p]->key > u->key) stack[p--]->maintain();
            if (p != top - 1) u->c[0] = stack[p + 1];
            stack[p]->c[1] = u;
            top = p + 1;
            stack[top++] = u;
        }
        while (top) stack[--top]->maintain();
        root = root->c[1];
    }
    void insert(Node *&u, int pos, int val) {
        if (!u) {
            u = new Node(val);
            return;
        }
        u->pushDown();
        int lSize = size(u->c[0]), x = lSize < pos;
        if (!x) insert(u->c[0], pos, val);
        else insert(u->c[1], pos - lSize - 1, val);
        Node *v = u->c[x];
        if (v->key < u->key) {
            v->pushDown();
            u->c[x] = v->c[x ^ 1];
            v->c[x ^ 1] = u;
            u = v;
            u->c[x ^ 1]->maintain();
        }
        u->maintain();
    }
    void insert(int pos, int val) {
        insert(root, pos, val);
    }
    void del(int pos) {
        std::pair<Node *, Node *> L = split(root, pos - 1);
        std::pair<Node *, Node *> R = split(L.second, 1);
        root = merge(L.first, R.second);
        delete R.first;
        R.first = NULL;
    }
    void add(int l, int r, int d) {
        std::pair<Node *, Node *> L = split(root, l - 1);
        std::pair<Node *, Node *> R = split(L.second, r - l + 1);
        R.first->add(d);
        merge(merge(L.first, R.first), R.second);
    }
    void reverse(int l, int r) {
        std::pair<Node *, Node *> L = split(root, l - 1);
        std::pair<Node *, Node *> R = split(L.second, r - l + 1);
        R.first->reverse();
        merge(merge(L.first, R.first), R.second);
    }
    void revolve(int l, int r, int k) {
        int len = (r - l + 1);
        k = (k % len + len) % len;
        if (!k) return;
        std::pair<Node *, Node *> L = split(root, l - 1);
        std::pair<Node *, Node *> R = split(L.second, len);
        std::pair<Node *, Node *> M = split(R.first, len - k);
        merge(merge(L.first, M.second), merge(M.first, R.second));
    }
    int query(int l, int r) {
        std::pair<Node *, Node *> L = split(root, l - 1);
        std::pair<Node *, Node *> R = split(L.second, r - l + 1);
        int res = R.first->min;
        merge(merge(L.first, R.first), R.second);
        return res;
    }
} treap;
MemoryPool<Treap::Node, MAXN << 1> Treap::Node::pool;
int main() {
    int n;
    scanf("%d", &n);
    static int a[MAXN];
    for (int i = 0; i < n; i++) scanf("%d", &a[i]);
    treap.build(n, a);
    int q;
    scanf("%d", &q);
    while (q--) {
        char op[10];
        scanf("%s", op);
        if (op[0] == 'A') {
            int l, r, d;
            scanf("%d %d %d", &l, &r, &d);
            treap.add(l, r, d);
        } else if (op[0] == 'I') {
            int pos, val;
            scanf("%d %d", &pos, &val);
            treap.insert(pos, val);
        } else if (op[0] == 'D') {
            int pos;
            scanf("%d", &pos);
            treap.del(pos);
        } else if (op[0] == 'M') {
            int l, r;
            scanf("%d %d", &l, &r);
            printf("%d\n", treap.query(l, r));
        } else if (op[3] == 'E')  {
            int l, r;
            scanf("%d %d", &l, &r);
            treap.reverse(l, r);
        } else {
            int l, r, k;
            scanf("%d %d %d", &l, &r, &k);
            treap.revolve(l, r, k);
        }
    }
    return 0;
}
```