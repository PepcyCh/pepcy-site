---
title: '[ZJOI 2013] k 大数查询'
date: 2017-05-07 20:43:04
tags: [二分, 整体二分, 线段树]
categories: 题解（OI/XCPC）
---

## 题目大意

有 $n$ 个位置，$m$ 个操作。操作有两种：

* `1 a b c` ：在第 $a$ 个位置到第 $b$ 个位置，每个位置加入一个数 $c$。
* `2 a b c` ：询问从第 $a$ 个位置到第 $b$ 个位置，第 $c$ 大的数是多少。

$1 \leqslant n, \; m \leqslant 50,000$

$1 \leqslant c \leqslant n$ （操作一中，并没有负数）

$1 \leqslant c \leqslant 2147483647$ 

## 题目链接

[【ZJOI 2013】K 大数查询 - Luogu 3332](https://www.luogu.com.cn/problem/P3332)

<!-- more -->

## 题解

整体二分。

二分权值，我们计算右侧对左侧的贡献（当然，可以反过来），用两个容器保存左右两侧要计算哪些操作。

对于插入，如果权值在右侧，我们在线段树／树状数组对应的区间加 $1$ ，放入右侧的容器中；否则直接放入左侧的容器中。（用 `std::vector` 的 `push_down()` 方法可以保证，全程从容器首开始，操作的相对顺序始终就是读入顺序）

对于查询，我们计算出对应区间的个数，如果小于 $c$ ，把 $c$ 减去计算结果放入左侧的容器；否则直接放入右侧的容器。

当权值区间长为 $1$ 时，查询操作的答案就是当前权值。

做这题时，顺便学了一发树状数组区间操作，虽然代码短、快（好像），但没有线段树好写。。。

另外，线段树／树状数组的值会爆 `int`，一些别的地方也会爆。于是光荣的 WA 了五、六次，中间还放了相当长一段时间，代码似乎不够整洁吧。。。

本题亦有权值线段树套区间线段树的做法，感觉除了可能会更慢以外和整体二分其实没什么区别。。。

## 代码

```c++
#include <cstdio>
#include <vector>
// #define DBG
const int MAXN = 50005;
struct Operation {
    int op, l, r, id;
    long long c;
    Operation(int id) : id(id) {}
#ifdef DBG
    void print() {
        printf("Operation(id = %d)[op = %d, [%d, %d], c = %lld]\n", id, op, l, r, c);
    }
#endif  
};
struct BinaryIndexedTree {
    long long c[2][MAXN];
    int n;
    static int lowbit(int x) {
        return x & -x;
    }
    void update(long long *c, int pos, long long d) {
        for (int i = pos; i <= n; i += lowbit(i)) c[i] += d;
    }
    void update(int l, int r, long long d) {
        update(c[0], l, d);
        update(c[0], r + 1, -d);
        update(c[1], l, -(l - 1) * d);
        update(c[1], r + 1, r * d);
    }
    long long query(long long *c, int pos) {
        long long res = 0;
        for (int i = pos; i; i -= lowbit(i)) res += c[i];
        return res;
    }
    long long query(int l, int r) {
        return query(c[0], r) * r + query(c[1], r) - query(c[0], l - 1) * (l - 1) - query(c[1], l - 1);
    }
    void init(int n) {
        this->n = n;
    }
#ifdef DBG
    void print() {
        puts("BIT :");
        for (int i = 1; i <= n; i++) printf("%d%c", query(i, i), i == n ? '\n' : ' ');
    }
#endif  
} bit;
int ans[MAXN];
void divide(int l, int r, std::vector<Operation> &vec) {
#ifdef DBG
    printf("Divide[%d, %d]\n", l, r);
    for (int i = 0; i < vec.size(); i++) vec[i].print();
#endif  
    if (vec.empty()) return;
    if (l == r) {
        for (int i = 0; i < vec.size(); i++) ans[vec[i].id] = l;
        return;
    }
    int mid = l + (r - l) / 2;
    std::vector<Operation> lv, rv;
    for (int i = 0; i < vec.size(); i++) {
        Operation &op = vec[i];
        if (op.op == 1) {
            if (op.c > mid) bit.update(op.l, op.r, 1), rv.push_back(op);
            else lv.push_back(op);
        } else {
            long long cnt = bit.query(op.l, op.r);
            if (cnt < op.c) op.c -= cnt, lv.push_back(op);
            else rv.push_back(op);
        }
#ifdef DBG
        bit.print();
#endif      
    }
    for (int i = 0; i < vec.size(); i++) if (vec[i].op == 1 && vec[i].c > mid)
        bit.update(vec[i].l, vec[i].r, -1);
    divide(l, mid, lv);
    divide(mid + 1, r, rv);
}
int main() {
    int n, m;
    scanf("%d %d", &n, &m);
    static bool isQuery[MAXN];
    std::vector<Operation> vec;
    for (int i = 1; i <= m; i++) {
        Operation op(i);
        scanf("%d %d %d %lld", &op.op, &op.l, &op.r, &op.c);
        if (op.op == 2) isQuery[i] = true;
        vec.push_back(op);
    }
    bit.init(n);
    divide(1, n, vec);
    for (int i = 1; i <= m; i++) if (isQuery[i]) printf("%d\n", ans[i]);
    return 0;
}
```