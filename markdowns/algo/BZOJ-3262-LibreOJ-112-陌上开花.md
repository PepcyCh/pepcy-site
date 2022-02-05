---
title: '三维偏序（陌上开花）'
date: 2017-04-01 21:49:59
tags: [模版, CDQ分治]
categories: 题解（OI/XCPC）
---

## 题目大意

给定 $n$ 朵花，每种花有三个属性 $a$、$b$、$c$（属性值为 $k$ 以内的正整数）。求有多少花的三个属性比 $d\ (d = 0, \ 1, \ 2 \dots, \ n - 1)$ 朵花都大（大于等于）。

$1 \leqslant n \leqslant 100,000$

$1 \leqslant k \leqslant 200,000$

## 题目链接

[三维偏序 - LibreOJ 112](https://loj.ac/problem/112)

<!-- more -->

## 题解

CDQ 分治模板题。

CDQ 分治：对区间 $[l,  \ r]$ 分为 $[l, \ mid]$ 与 $[mid + 1, \ r]$ 两个区间分别计算，最后计算左区间对右区间的贡献。

一句话概括三维偏序：一维排序，二维归并，三维树状数组。

排序保证了第一维的大小关系，在第一维有序之后的归并保证了第二维的大小关系，建立在权值上的树状数组保证了第三维的大小关系。

具体实现见代码。

另外，这道题可能会有两朵花一模一样，应当在处理前将同一种花归在一起。

## 代码

```c++
#include <cstdio>
#include <algorithm>
// #define DBG
const int MAXN = 100005;
const int MAXK = 200005;
struct Data {
	int a, b, c;
	int cnt, ans;
	bool operator<(const Data &another) const {
		return a < another.a || (a == another.a && b < another.b) || (a == another.a && b == another.b && c < another.c);
	}
#ifdef DBG
	void print() {
		printf("Data[(%d, %d, %d), cnt = %d, ans = %d]\n", a, b, c, cnt, ans);
	}
#endif	
} a[MAXN], A[MAXN];
struct BinaryIndexedTree {
	int c[MAXK], k;
	static int lowbit(int x) {
		return x & -x;
	}
	void update(int pos, int d) {
		for (int i = pos; i <= k; i += lowbit(i)) c[i] += d;
	}
	int query(int pos) {
		int res = 0;
		for (int i = pos; i; i -= lowbit(i)) res += c[i];
		return res;
	}
	void clear(int pos) {
		for (int i = pos; i <= k; i += lowbit(i)) {
			if (c[i]) c[i] = 0;
			else break;
		}
	}
	void init(int k) {
		this->k = k;
	}
} bit;
void cdq(Data *l, Data *r) {
	if (l == r) {
		l->ans += l->cnt - 1;
		return;
	}
	Data *mid = l + (r - l) / 2;
	cdq(l, mid);
	cdq(mid + 1, r);
	static Data temp[MAXN];
	for (Data *p = temp, *pl = l, *pr = mid + 1; p <= temp + (r - l); p++) {
		if (pr > r || (pl <= mid && pl->b <= pr->b)) {
			*p = *pl++;
			bit.update(p->c, p->cnt);
		} else {
			*p = *pr++;
			p->ans += bit.query(p->c);
		}
	}
	for (Data *p = temp, *q = l; q <= r; q++, p++) {
		*q = *p;
		bit.clear(p->c);
	}
}
int main() {
	int n, k;
	scanf("%d %d", &n, &k);
	for (int i = 0; i < n; i++) scanf("%d %d %d", &a[i].a, &a[i].b, &a[i].c), a[i].cnt = 1;
	std::sort(a, a + n);
#ifdef DBG
	for (int i = 0; i < n; i++) a[i].print();
#endif	
	int cnt = 0;
	for (int i = 0; i < n; i++) {
		if (i == 0 || !(a[i].a == a[i - 1].a && a[i].b == a[i - 1].b && a[i].c == a[i - 1].c)) A[++cnt] = a[i];
		else A[cnt].cnt++;
	}
	bit.init(k);
	cdq(A + 1, A + cnt);
	static int ans[MAXN];
	for (int i = 1; i <= cnt; i++) ans[A[i].ans] += A[i].cnt;
	for (int i = 0; i < n; i++) printf("%d\n", ans[i]);
	return 0;
}
```