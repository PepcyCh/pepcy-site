---
title: ZJU 七月集训题目选讲（Contest 10 ～ 12）
date: 2018-08-01 16:17:08
tags: [ZJU, DP, 矩阵乘法, 佩尔方程, 线段树, 网络流, 最小割]
categories: 题解（OI/XCPC）
---

ZJU 七月集训（2018.7.13 ～ 2018.7.26）的题目。

* Contest 10 by bits/stdc++.h - 2018.7.24
* Contest 11 by ChugJug - 2018.7.25
* Contest 12 by DeepDark - 2018.7.26

所有题目都是多组数据，若无子测试点数的说明，则 $T$ 不会特别大。

## 目录

[Contest 10 - A - Bnumbers](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-10-%EF%BD%9E-12%EF%BC%89/#10a)

[Contest 10 - D - Osu](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-10-%EF%BD%9E-12%EF%BC%89/#10d)

[Contest 10 - F - Pokemon](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-10-%EF%BD%9E-12%EF%BC%89/#10f)

[Contest 11 - A - Haibara and Bit Strings](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-10-%EF%BD%9E-12%EF%BC%89/#11a)

[Contest 11 - E - JSB's Happy Life 1](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-10-%EF%BD%9E-12%EF%BC%89/#11e)

[Contest 12 - B - zx2018 and Study](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-10-%EF%BD%9E-12%EF%BC%89/#12b)

[Contest 12 - E - zx2018 and Treasures](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-10-%EF%BD%9E-12%EF%BC%89/#12e)

[Contest 12 - F - zx2018 and sequence](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-10-%EF%BD%9E-12%EF%BC%89/#12f)

<!--more-->

## Contest 10 - A - Bnumbers<span id = "10a"></span>

### 题目大意

一个数 $k$ 在 $a$ 进制下的表示为 $x_nx_{n-1} \dots x_1x_0$。定义 $k$ 是 $a$ 进制下的一个 B 数，当且仅当：

$$
\sum_{i = 0}^{n} x_i^2 = k
$$

求 $a$ 进制下共有多少个 B 数。

$2 \leq N \leq 1,000$

$2 \leq a \leq 10^6$

$\sum a \leq 10^8$

时间限制：$2 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

可以证明，B 数要么是 $1$，要么是一个 $a$ 进制下的两位数。写出对应的不定方程，$O(a)$ 枚举求解另一变量并判断计数即可。

### 代码

```c++
#include <cstdio>
#include <cmath>
#include <algorithm>

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);

        int ans = 1;
        for (int i = 1; i < n; i++) {
            long long temp = (long long) (n - i) * i;
            long long delta = 1 + temp * 4;
            long long st = (int) (std::sqrt(delta) + 0.001);
            if (st * st != delta) continue;
            int j = (1 + st) / 2;
            if (j < n) ++ans;
        }

        printf("%d\n", ans);
    }

    return 0;
}
```

## Contest 10 - D - Osu<span id = "10d"></span>

### 题目大意

一个长为 $n$ 的 $01$ 序列，每个位置有 $p_i$ 的概率为 $1$。一段长为 $len$ 的连续的 $1$ 的贡献为 $len^3$。有 $Q$ 次操作，每次操作会将位置 $i$ 的概率改变，或询问 $[l, r]$ 内总贡献的期望。

$1 \leq n, Q \leq 100,000$

$1 \leq T \leq 5$

时间限制：$2 \text{s}$，内存限制：$128 \text{MB}$。

### 题解

长度从 $len$ 变为 $len + 1$，贡献值会增加 $(len + 1)^3 - len^3 = 3len^2 + 3len + 1$。

记 $f(i)$ 为以 $i$ 结尾的答案，$E(i)$ 为以 $i$ 结尾的长度期望，$E2(i)$ 为以 $i$ 结尾的长度平方的期望，则转移为：

$$
f(i) = f(i - 1) + p_i(3E2(i) + 3E(i) + 1) \\
E(i) = p_i(E(i - 1) + 1) \\
E2(i) = p_i(E2(i - 1) + 2E(i - 1) + 1)
$$

考虑矩阵乘法，为每一个位置构造一个转移矩阵，用线段树维护区间信息，修改时单点修改矩阵，询问时直接查询即可。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 100005;

int dcmp(double a, double b = 0) {
    const static double EPS = 1e-12;
    double d = a - b;
    return std::abs(d) <= EPS ? 0 : (d > 0 ? 1 : -1);
}

struct Matrix {
    const static int N = 4;

    double a[N][N];
    int n, m;

    Matrix(int n = N, int m = N, bool eye = false) : n(n), m(m) {
        for (int i = 0; i < n; i++) std::fill(a[i], a[i] + m, 0);
        if (eye) for (int i = 0; i < n; i++) a[i][i] = 1;
    }

    double *operator[](int i) {
        return a[i];
    }
    const double *operator[](int i) const {
        return a[i];
    }

    Matrix operator*(const Matrix &rhs) const {
        Matrix res(n, rhs.m);
        for (int i = 0; i < n; i++) for (int j = 0; j < m; j++) if (dcmp(a[i][j]))
            for (int k = 0; k < rhs.m; k++) res[i][k] += a[i][j] * rhs[j][k];
        return res;
    }
} A;

Matrix getMat(double p) {
    Matrix res(4, 4);
    res[0][0] = res[3][3] = 1;
    res[0][1] = res[0][2] = 3 * p;
    res[0][3] = res[1][1] = res[1][3] = res[2][2] = res[2][3] = p;
    res[2][1] = 2 * p;
    return res;
}

struct SegT {
    struct Node {
        Node *lc, *rc;
        Matrix val;

        Node() {}
        Node(const Matrix &val) : lc(lc), rc(rc), val(val) {}
        Node(Node *lc, Node *rc) : lc(lc), rc(rc), val(rc->val * lc->val) {}

        void maintain() {
            val = rc->val * lc->val;
        }

        void update(int l, int r, int pos, double p) {
            if (l == r) {
                val = getMat(p);
                return;
            }
            int mid = l + ((r - l) >> 1);
            if (pos <= mid) lc->update(l, mid, pos, p);
            else rc->update(mid + 1, r, pos, p);
            maintain();
        }

        void query(int l, int r, int L, int R, Matrix &t) {
            if (R < l || r < L) return;
            if (L <= l && r <= R) {
                t = val * t;
                return;
            }
            int mid = l + ((r - l) >> 1);
            lc->query(l, mid, L, R, t);
            rc->query(mid + 1, r, L, R, t);
        }
    } *root, _pool[MAXN << 1], *_curr;
    int n;

    void init() {
        _curr = _pool;
        root = NULL;
    }

    Node *build(int l, int r, double *p) {
        if (l == r) return new (_curr++) Node(getMat(p[l]));
        int mid = l + ((r - l) >> 1);
        return new (_curr++) Node(build(l, mid, p), build(mid + 1, r, p));
    }
    void build(double *p, int n) {
        this->n = n;
        root = build(1, n, p);
    }

    void update(int pos, double p) {
        root->update(1, n, pos, p);
    }

    Matrix query(int l, int r) {
        Matrix res = A;
        root->query(1, n, l, r, res);
        return res;
    }
} segT;

int main() {
    A = Matrix(4, 1);
    A[3][0] = 1;

    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);

        static double P[MAXN];
        for (int i = 1; i <= n; i++) scanf("%lf", &P[i]);

        segT.init();
        segT.build(P, n);

        int q;
        scanf("%d", &q);
        while (q--) {
            int op;
            scanf("%d", &op);

            if (op == 1) {
                int x;
                double p;
                scanf("%d %lf", &x, &p);

                segT.update(x, p);
            } else {
                int l, r;
                scanf("%d %d", &l, &r);

                Matrix ans = segT.query(l, r);
                printf("%.2f\n", ans[0][0]);
            }
        }
    }
    
    return 0;
}
```

## Contest 10 - F - Pokemon<span id = "10f"></span>

### 题目大意

给定一个长为 $n$ 的序列 $\{a_i\}$，求至少删除多少的数后，序列的最长上升子序列的长度会减少。

$0 \leq n \leq 1,000$

$0 \leq a_i \leq 10^9$

$1 \leq T \leq 5$

时间限制：$1 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

考虑计算 LIS 的 DP 过程，如果 $f(i) = f(j) + 1$，则建一单向条边 $(j, i)$ ，拆点跑最小割即为答案。

### 代码

```c++
#include <cstdio>
#include <climits>
#include <queue>
#include <algorithm>

const int MAXN = 1005;

int a[MAXN], f[MAXN];

struct Node;
struct Edge;

struct Node {
    Edge *e, *curr;
    int level;
} N[MAXN << 1];

struct Edge {
    Node *u, *v;
    Edge *next, *rev;
    int cap, flow;

    Edge() {}
    Edge(Node *u, Node *v, int cap) : u(u), v(v), next(u->e), cap(cap), flow(0) {}
} _pool[MAXN * MAXN], *_curr;

void addEdge(int u, int v, int cap) {
    N[u].e = new (_curr++) Edge(&N[u], &N[v], cap);
    N[v].e = new (_curr++) Edge(&N[v], &N[u], 0);
    N[u].e->rev = N[v].e;
    N[v].e->rev = N[u].e;
}

namespace Dinic {
bool level(Node *s, Node *t, int n) {
    for (int i = 0; i < n; i++) N[i].level = 0;

    static std::queue<Node *> q;
    while (!q.empty()) q.pop();
    q.push(s);
    s->level = 1;
    while (!q.empty()) {
        Node *u = q.front();
        q.pop();

        for (Edge *e = u->e; e; e = e->next) if (e->cap > e->flow && !e->v->level) {
            e->v->level = u->level + 1;
            if (e->v == t) return true;
            q.push(e->v);
        }
    }

    return false;
}

int findPath(Node *u, Node *t, int limit = INT_MAX) {
    if (u == t) return limit;
    for (Edge *&e = u->curr; e; e = e->next) if (e->cap > e->flow && e->v->level == u->level + 1) {
        int flow = findPath(e->v, t, std::min(e->cap - e->flow, limit));
        if (flow > 0) {
            e->flow += flow;
            e->rev->flow -= flow;
            return flow;
        }
    }
    return 0;
}

int solve(int s, int t, int n) {
    int res = 0;
    while (level(&N[s], &N[t], n)) {
        for (int i = 0; i < n; i++) N[i].curr = N[i].e;
        int flow;
        while ((flow = findPath(&N[s], &N[t])) > 0) res += flow;
    }
    return res;
}
}

void clear(int n) {
    _curr = _pool;
    for (int i = 0; i < n; i++) {
        N[i].e = NULL;
    }
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);
        if (!n) {
            puts("0");
            continue;
        }
        for (int i = 0; i < n; i++) scanf("%d", &a[i]);

        int lis = 0;
        int s = 0, t = 2 * n + 1;
        clear(t + 1);
        for (int i = 0; i < n; i++) {
            int max = 0;
            for (int j = 0; j < i; j++) if (a[j] < a[i]) max = std::max(max, f[j]);
            f[i] = max + 1;
            lis = std::max(lis, f[i]);

            for (int j = 0; j < i; j++) if (a[j] < a[i] && f[j] == max) addEdge(n + j, i, INT_MAX);
            addEdge(i, i + n, 1);
        }

        for (int i = 0; i < n; i++) {
            if (f[i] == 1) addEdge(s, i, INT_MAX);
            if (f[i] == lis) addEdge(i + n, t, INT_MAX);
        }

        int ans = Dinic::solve(s, t, t + 1);
        printf("%d\n", ans);
    }
    return 0;
}
```

## Contest 11 - A - Haibara and Bit Strings<span id = "11a"></span>

### 题目大意

长为 $n = 10^m$ 的 $01$ 串中，$01$ 作为子串出现了两次的有多少个，答案对 $1,000,000,007$ 取模。

$1 \leq m \leq 10^7$

$1 \leq T \leq 10,000$

时间限制：$1 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

满足条件的串都长这样：

$$
1\dots10\dots01\dots10\dots01\dots10\dots0
$$

问题转化为 $a + b + c + d + e + f = n$ 的非负整数解的个数，由隔板法知答案为 $\binom{n + 1}{5}$。

### 代码

```c++
#include <cstdio>

const int MOD = 1000000007;

long long qpow(long long a, long long n) {
    long long res = 1;
    for (; n; n >>= 1, a = a * a % MOD) if (n & 1) res = res * a % MOD;
    return res;
}

long long inv = 808333339;

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int m;
        scanf("%d", &m);
        int n = qpow(10, m);

        long long ans = (n == MOD - 1 ? 0 : n + 1);
        for (int i = 1; i < 5; i++) {
            long long t = n + 1 - i;
            t < 0 ? t += MOD : 0;
            ans = ans * t % MOD;
        }
        ans = ans * inv % MOD;

        printf("%lld\n", ans);
    }
    
    return 0;
}
```

## Contest 11 - E - JSB's Happy Life 1<span id = "11e"></span>

### 题目大意

有 $n$ 个数 $\{a_i\}$，定义其一个子集的价值为集合中 $a_i$ 的和，求其所有非空子集中，价值第 $k$ 小的价值。

$1 \leq n, k, a_i \leq 100,000$

$1 \leq T \leq 20$

时间限制：$1 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

数的顺序对答案无影响，故可以现按升序对其排序。对于每一个状态，其后继都有两种选择：加上下一个；或先删去当前的，再加上下一个。用一个优先队列储存，先放入所有元素，每次弹出一个并放入其两个后继至完成 $k - 1$ 次操作，此时优先队列的队首即为答案。

### 代码

```c++
#include <cstdio>
#include <queue>
#include <algorithm>

const int MAXN = 100005;

struct HeapNode {
    int sum, pos;

    HeapNode(int sum, int pos) : sum(sum), pos(pos) {}

    bool operator<(const HeapNode &rhs) const {
        return sum > rhs.sum;
    }
};

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n, k;
        scanf("%d %d", &n, &k);

        static int a[MAXN];
        for (int i = 0; i < n; i++) scanf("%d", &a[i]);
        std::sort(a, a + n);

        static std::priority_queue<HeapNode> q;
        while (!q.empty()) q.pop();
        q.push(HeapNode(a[0], 0));

        for (int i = 1; i < k; i++) {
            HeapNode u = q.top();
            q.pop();

            if (u.pos < n - 1) {
                q.push(HeapNode(u.sum + a[u.pos + 1], u.pos + 1));
                q.push(HeapNode(u.sum + a[u.pos + 1] - a[u.pos], u.pos + 1));
            }
        }

        printf("%d\n", q.top().sum);
    }
    
    return 0;
}
```

## Contest 12 - B - zx2018 and Study<span id = "12b"></span>

### 题目大意

求方程

$$
\sum_{i = 1}^{a}i = 2\sum_{i = 1}^{b - 1} + b
$$

从小到大的第 $n$ 组解。

$1 \leq n \leq 10^{12}$

$1 \leq T \leq 1,000$

时间限制：$2 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

方程可变形为：

$$
(2a + 1)^2 - 8b^2 = 1
$$

令 $u = 2a + 1, v = b$，则方程为一个佩尔方程。由相关理论知：

$$
\begin{align}
u_n &= u_{n - 1}u_0 + v_{n - 1} + 8 \\
v_n &= v_{n - 1}u_0 + u_{n - 1}v_0
\end{align}
$$

复原 $a, b$ ，用矩阵乘法快速幂计算即可。

### 代码

```c++
#include <cstdio>
#include <climits>
#include <vector>
#include <queue>
#include <stack>
#include <algorithm>

const int MOD = 1000000009;

struct Matrix {
    static const int N = 3;

    long long a[N][N];
    int n, m;

    Matrix(int n = N, int m = N, bool eye = false) : n(n), m(m) {
        for (int i = 0; i < n; i++) std::fill(a[i], a[i] + m, 0);
        if (eye) for (int i = 0; i < n; i++) a[i][i] = 1;
    }

    long long *operator[](int i) {
        return a[i];
    }
    const long long *operator[](int i) const {
        return a[i];
    }

    Matrix operator*(const Matrix &rhs) const {
        Matrix res(n, rhs.m);
        for (int i = 0; i < n; i++) for (int j = 0; j < m; j++) for (int k = 0; k < rhs.m; k++) {
            res[i][k] += a[i][j] * rhs[j][k] % MOD;
            res[i][k] >= MOD ? res[i][k] -= MOD : 0;
        }
        return res;
    }

    void print() const {
        printf("Matrix(%d, %d)\n", n, m);
        for (int i = 0; i < n; i++) for (int j = 0; j < m; j++) printf("%lld%c", a[i][j], " \n"[j == m - 1]);
    }
} T, A;

Matrix pow(Matrix a, long long n) {
    Matrix res(a.n, a.m, true);
    for (; n; n >>= 1, a = a * a) if (n & 1) res = res * a;
    return res;
}

void pre() {
    T[0][0] = 3;
    T[0][1] = 4;
    T[0][2] = 1;
    T[1][0] = 2;
    T[1][1] = 3;
    T[1][2] = 1;
    T[2][2] = 1;

    A.m = 1;
    A[0][0] = A[1][0] = A[2][0] = 1;
}

int main() {
    pre();

    int t;
    scanf("%d", &t);
    while (t--) {
        long long n;
        scanf("%lld", &n);

        Matrix temp = pow(T, n - 1);
        temp = temp * A;

        printf("%lld %lld\n", temp[1][0], temp[0][0]);
    }
    
    return 0;
}
```

## Contest 12 - E - zx2018 and Treasures<span id = "12e"></span>

### 题目大意

给定 $n$ 和函数 $G(s) ~ (s \in [0, 2^n))$ 的函数值。求

$$
F(0) \oplus F(1) \oplus \cdots \oplus F(2^n - 1) \\
F(S) \sum_{S = S | T} G(T)(|S| - |T| + 1)
$$

其中 $|$ 表示按位或运算，$|S|$ 表示 $S$ 在二进制表示下 $1$ 的个数。

$0 \leq n \leq 18$

$G(s) \in \{0, 1\}$

$1 \leq T \leq 20$

### 题解

$F(S) = \sum G(T)(|S| - |T| + 1) = (|S| + 1)\sum G(T) - \sum G(T)|T|$，分别计算 $G(s)$ 和 $G(s)|s|$ 的 $n$ 维前缀和计算即可。

### 代码

```c++
#include <cstdio>

const int MAXN = (1 << 18) + 5;

int g[MAXN], g2[MAXN], f[MAXN];

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);

        for (int i = 0; i < (1 << n); i++) {
            scanf("%d", &g[i]);
            g2[i] = g[i] * __builtin_popcount(i);
        }

        for (int i = 0; i < n; i++) for (int j = 0; j < (1 << n); j++) if (j & (1 << i)) {
            g[j] += g[j ^ (1 << i)];
            g2[j] += g2[j ^ (1 << i)];
        }

        int ans = 0;
        for (int i = 0; i < (1 << n); i++) {
            f[i] = g[i] * (__builtin_popcount(i) + 1) - g2[i];
            ans ^= f[i];
        }

        printf("%d\n", ans);
    }
    
    return 0;
}
```

## Contest 12 - F - zx2018 and sequence<span id = "12f"></span>

### 题目大意

一个长为 $n$ 的数列 $\{a_i\}$，有 $m$ 次操作。操作有三种：

* 区间按位与上同一个数
* 区间按位或上同一个数
* 询问区间最大值

$1 \leq n, m \leq 10^5$

$1 \leq T \leq 5$

### 题解

用线段树维护数列，当进行区间修改时，直到区间内的数均相同后才开始修改。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 100005;

struct SegT {
    struct Node {
        Node *lc, *rc;
        int val, max, tag;

        Node(int val = 0) : val(val), max(val), tag(0), lc(NULL), rc(NULL) {}
        Node(Node *lc, Node *rc) : lc(lc), rc(rc), tag(0) {
            maintain();
        }

        void maintain() {
            val = (lc->val == rc->val ? lc->val : -1);
            max = std::max(lc->max, rc->max);
        }

        void add(int d) {
            tag += d;
            val += d;
            max += d;
        }

        void pushDown() {
            if (tag) {
                lc->add(tag);
                rc->add(tag);
                tag = 0;
            }
        }

        void Add(int l, int r, int L, int R, int d) {
            if (L > r || l > R) return;
            if (L <= l && r <= R && val != -1) {
                int temp = (val & d) - val;
                add(temp);
                return;
            }
            pushDown();
            int mid = l + ((r - l) >> 1);
            lc->Add(l, mid, L, R, d);
            rc->Add(mid + 1, r, L, R, d);
            maintain();
        }

        void Or(int l, int r, int L, int R, int d) {
            if (L > r || l > R) return;
            if (L <= l && r <= R && val != -1) {
                int temp = (val | d) - val;
                add(temp);
                return;
            }
            pushDown();
            int mid = l + ((r - l) >> 1);
            lc->Or(l, mid, L, R, d);
            rc->Or(mid + 1, r, L, R, d);
            maintain();
        }

        int query(int l, int r, int L, int R) {
            if (L > r || l > R) return 0;
            if (L <= l && r <= R) return max;
            pushDown();
            int mid = l + ((r - l) >> 1);
            return std::max(lc->query(l, mid, L, R), rc->query(mid + 1, r, L, R));
        }
    } *root, _pool[MAXN << 1], *_curr;
    int n;

    void init() {
        _curr = _pool;
    }

    Node *build(int *l, int *r) {
        if (l == r) return new (_curr++) Node(*l);
        int *mid = l + ((r - l) >> 1);
        return new (_curr++) Node(build(l, mid), build(mid + 1, r));
    }

    void build(int *a, int n) {
        this->n = n;
        root = build(a, a + n - 1);
    }

    void Add(int l, int r, int d) {
        root->Add(1, n, l, r, d);
    }

    void Or(int l, int r, int d) {
        root->Or(1, n, l, r, d);
    }
    
    int query(int l, int r) {
        return root->query(1, n, l, r);
    }
} segT;

int main() {
    int n, q;
    while (scanf("%d %d", &n, &q) == 2) {
        static int a[MAXN];
        for (int i = 0; i < n; i++) scanf("%d", &a[i]);

        segT.init();
        segT.build(a, n);

        while (q--) {
            int op, l, r;
            scanf("%d %d %d", &op, &l, &r);

            if (op == 1) {
                int d;
                scanf("%d", &d);
                segT.Add(l, r, d);
            } else if (op == 2) {
                int d;
                scanf("%d", &d);
                segT.Or(l, r, d);
            } else {
                int ans = segT.query(l, r);
                printf("%d\n", ans);
            }
        }
    }
    
    return 0;
}
```