---
title: ZJU 七月集训题目选讲（Contest 7 ～ 9）
date: 2018-08-01 14:57:16
tags: [ZJU, DP, 矩阵乘法, 贪心, 原根, Burnside, 网络流, 二分图博弈, 博弈]
categories: 题解（OI/XCPC）
---

ZJU 七月集训（2018.7.13 ～ 2018.7.26）的题目。

* Contest 7 by SBconscious - 2018.7.20
* Contest 8 by SBconscious - 2018.7.21
* Contest 9 by Astolfo - 2018.7.23

所有题目都是多组数据，若无子测试点数的说明，则 $T$ 不会特别大。

## 目录

[Contest 7 - B - Die Wacht am Rhein](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-7-%EF%BD%9E-9%EF%BC%89/#07b)

[Contest 7 - C - Yet another Chess Puzzle](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-7-%EF%BD%9E-9%EF%BC%89/#07c)

[Contest 8 - C - Bulbasaur and Prime Game](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-7-%EF%BD%9E-9%EF%BC%89/#08c)

[Contest 8 - F - Totodile and Graph Theory](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-7-%EF%BD%9E-9%EF%BC%89/#08f)

[Contest 8 - G - Totodile and Another Data Structure Problem?](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-7-%EF%BD%9E-9%EF%BC%89/#08g)

[Contest 9 - A - LYK and Heltion's gifts](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-7-%EF%BD%9E-9%EF%BC%89/#09a)

[Contest 9 - H - LYK and Painting](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-7-%EF%BD%9E-9%EF%BC%89/#09h)

<!--more-->

## Contest 7 - B - Die Wacht am Rhein<span id = "07b"></span>

### 题目大意

给定一张 $N$ 个点、$M$ 条边的森林，每个点有一个权值 $a_i$。现要增加若干条边，使得图连通，且满足每个顶点最多连接了一条新边。定义新增一条边 $(i, j)$ 的代价为 $a_i + a_j$，求最小代价，或判断它不可能实现。

$1 \leq N \leq 100,000$

$0 \leq M \leq N - 1$

$1 \leq a_i \leq 10^9$

$\sum N \leq 2,500,000$

时间限制：$1 \text{s}$，内存限制：$256 \text{MB}$。

### 题解

如果原图只有一个连通块，则答案为 $0$；否则每个连通块内至少有一个点连接了新边，我们选择连通块内权值最小的点加入答案。

由于输入是一个森林，我们可以知道原图有 $N - M$ 个连通块，连接它们需要 $N - M - 1$ 条边，即 $2(N - M - 1)$ 个点。在每个连通块加入一个点后，仍需要 $N - M - 2$ 个点。我们发现，连边 $(p, q)$ 和 $(s, t)$ 所需的代价等于连边 $(p, t)$ 和 $(s, q)$，即代价与具体连边方式无关。我们把剩下的点的权值按升序排序，取最小的 $N - M - 2$ 个点加入答案即可。

### 代码

```c++
#include <cstdio>
#include <vector>
#include <algorithm>

const int MAXN = 100005;

struct Node;
struct Edge;

struct Node {
    Edge *e;
    bool used, vis;
    int w;
} N[MAXN];

struct Edge {
    Node *u, *v;
    Edge *next;

    Edge() {}
    Edge(Node *u, Node *v) : u(u), v(v), next(u->e) {}
} _pool[MAXN << 1], *_curr;

void addEdge(int u, int v) {
    N[u].e = new (_curr++) Edge(&N[u], &N[v]);
    N[v].e = new (_curr++) Edge(&N[v], &N[u]);
}

int min;
Node *minV;

void dfs(Node *u) {
    u->vis = true;
    if (!minV || u->w < min) {
        min = u->w;
        minV = u;
    }

    for (Edge *e = u->e; e; e = e->next) if (!e->v->vis) dfs(e->v);
}

void clear(int n) {
    _curr = _pool;
    for (int i = 0; i < n; i++) {
        N[i].e = NULL;
        N[i].used = false;
        N[i].vis = false;
    }
}

int main() {
    int n, m;
    while (scanf("%d %d", &n, &m) == 2) {
        clear(n);

        for (int i = 0; i < n; i++) scanf("%d", &N[i].w);

        for (int i = 0, u, v; i < m; i++) {
            scanf("%d %d", &u, &v);
            addEdge(u, v);
        }

        int need = n - m - 1;
        if (n / 2 < need) {
            puts("Impossible");
            continue;
        } else if (need == 0) {
            puts("0");
            continue;
        } else {
            long long ans = 0;
            for (int i = 0; i < n; i++) if (!N[i].vis) {
                minV = NULL;
                dfs(&N[i]);
                ans += min;
                minV->used = true;
            }

            --need;
            static std::vector<int> vec;
            vec.clear();
            for (int i = 0; i < n; i++) if (!N[i].used) vec.push_back(N[i].w);
            std::sort(vec.begin(), vec.end());

            for (int i = 0; i < need; i++) ans += vec[i];

            printf("%lld\n", ans);
        }
    }
    
    return 0;
}
```

## Contest 7 - C - Yet another Chess Puzzle<span id = "07c"></span>

### 题目大意

给定一个 $n \times n$ 的网格，保证 $n + 1$ 为质数。在网格内放入 $n$ 个棋子，使它们满足：

* 任意两个棋子不在同一行、列；
* 不存在两个点对 $(A, B)$ 和 $(C, D)$ （点不完全相同即算数），满足 $|x_A - x_B| = |x_C - x_D|$ 且 $|y_A - y_B| = |y_C - y_D|$。

输出一个满足以上条件的摆放方式。

$2 \leq n \leq 5,000 ~ (n + 1 \text{ is a prime number})$

$1 \leq T \leq 1,000$

时间限制：$1 \text{s}$，内存限制：$256 \text{MB}$。

### 题解

我们可以求出 $n + 1$ 的一个原根 $g$，则 $(0, g^0), (1, g^1), \dots, (n - 1, g^{n - 1})$ 即是答案。

第一个限制显然满足了，下证其满足第二个限制：

假设存在两个点对不满足限制二，则有：

$$
\begin{align}
g^{x_2} - g^{x_1} &\equiv g^{x_4} - g^{x_3} (\bmod n + 1) \\
g^{x_1 + dx} - g^{x_1} &\equiv g^{x_3 + dx} - g^{x_3} (\bmod n + 1) \\
g^{x_1}(g^{dx} - 1) &\equiv g^{x_3}(g^{dx} - 1) (\bmod n + 1) \\
g^{x_1} &\equiv g^{x_3} (\bmod n + 1)
\end{align}
$$

与原根的性质不符。

### 代码

```c++
#include <cstdio>

const int MAXN = 5005;

int prime[MAXN], primeCnt;

void sieve() {
    static bool notPrime[MAXN];
    notPrime[0] = notPrime[1] = true;
    primeCnt = 0;

    for (int i = 2; i < MAXN; i++) {
        if (!notPrime[i]) prime[primeCnt++] = i;

        for (int j = 0; j < primeCnt && i * prime[j] < MAXN; j++)
            notPrime[i * prime[j]] = true;
    }
}

int pow(int a, int n, int p) {
    int res = 1;
    for (; n; n >>= 1, a = a * a % p) if (n & 1) res = res * a % p;
    return res;
}

int getRoot(int p) {
    for (int g = 2, pp = p - 1; ; g++) {
        bool flag = true;
        for (int i = 0; i < primeCnt && prime[i] < p; i++) {
            if (pp % prime[i] == 0 && pow(g, pp / prime[i], p) == 1) {
                flag = false;
                break;
            }
        }

        if (flag) return g;
    }
}

int main() {
    sieve();

    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);

        int g = getRoot(n + 1), k = 1;
        for (int i = 0; i < n; i++) {
            k = k * g % (n + 1);
            printf("%d%c", k, " \n"[i == n - 1]);
        }
    }
    
    return 0;
}
```

## Contest 8 - C - Bulbasaur and Prime Game<span id = "08c"></span>

### 题目大意

有 $n$ 个数 $\{a_i\}$ 。两个人进行游戏：当上一个被取走的数为 $a_x$ 时，该行动的一方可以从这 $n$ 个数中选取一个数 $a_y$，满足 $a_x + a_y$ 是一个质数；起初 $a_x = 0$；一方不能行动时另一方获胜。求必胜方。

$1 \leq n \leq 1,000$

$2 \leq a_i \leq 10^9$

$\sum n \leq 5,000$

时间限制：$1 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

对于每一对满足 $a_x + a_y$ 为质数的数对，建一条边 $(x, y)$，则可以得到一个二分图。则问题转换为一个[二分图游戏](http://cogs.pro:8080/cogs/problem/problem.php?pid=1720)。建图跑一遍网络流/匈牙利并判断即可。

对于判质数，我们可以预处理一定范围内的质数，或用 Pollard's Rho。

### 代码

```c++
#include <cstdio>
#include <climits>
#include <queue>
#include <algorithm>

const int MAXN = 1005;
const int MAXM = 300005;
const int M = 100000;

int prime[M], primeCnt;
bool notPrime[M];

void init() {
    notPrime[0] = notPrime[1] = true;
    primeCnt = 0;
    for (int i = 2; i < M; i++) {
        if (!notPrime[i]) prime[primeCnt++] = i;
        for (int j = 0; j < primeCnt && i * prime[j] < M; j++) notPrime[i * prime[j]] = true;
    }
}

bool isPrime(int x) {
    if (x < M) return !notPrime[x];

    bool res = true;
    for (int i = 0; i < primeCnt && (long long) prime[i] * prime[i] <= x; i++) if (x % prime[i] == 0) {
        res = false;
        break;
    }

    return res;
}

struct Node;
struct Edge;

struct Node {
    Edge *e, *curr;
    int level;
    bool canS, canT;
} N[MAXN];

struct Edge {
    Node *u, *v;
    Edge *next, *rev;
    int cap, flow;

    Edge() {}
    Edge(Node *u, Node *v, int cap) : u(u), v(v), cap(cap), flow(0), next(u->e) {}
} _pool[MAXM << 1], *_curr;

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
        int flow = findPath(e->v, t, std::min(limit, e->cap - e->flow));
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
} // namespace Dinic

void bfs(int s, int t) {
    static std::queue<Node *> q;

    q.push(&N[s]);
    N[s].canS = true;
    while (!q.empty()) {
        Node *u = q.front();
        q.pop();

        for (Edge *e = u->e; e; e = e->next) if (e->cap > e->flow && !e->v->canS) {
            e->v->canS = true;
            q.push(e->v);
        }
    }

    q.push(&N[t]);
    N[t].canT = true;
    while (!q.empty()) {
        Node *u = q.front();
        q.pop();

        for (Edge *e = u->e; e; e = e->next) if (e->rev->cap > e->rev->flow && !e->v->canT) {
            e->v->canT = true;
            q.push(e->v);
        }
    }
}

void clear(int n) {
    _curr = _pool;
    for (int i = 0; i < n; i++) {
        N[i].e = NULL;
        N[i].canS = N[i].canT = false;
    }
}

int main() {
    init();

    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);

        int odd = 0, even = 0;
        static int w[2][MAXN];
        for (int i = 0, x; i < n; i++) {
            scanf("%d", &x);

            if (x % 2) w[1][++odd] = x;
            else w[0][++even] = x;
        }

        int s = 0, t = odd + even + 1;
        n = odd + even + 2;
        clear(n);

        for (int i = 1; i <= odd; i++) addEdge(s, i, 1);
        for (int i = 1; i <= even; i++) addEdge(i + odd, t, 1);
        for (int i = 1; i <= odd; i++) for (int j = 1; j <= even; j++) if (isPrime(w[1][i] + w[0][j]))
            addEdge(i, j + odd, 1);

        Dinic::solve(s, t, n);

        bfs(s, t);

        bool flag = false;
        for (int i = 1; i <= odd; i++) if (N[i].canS && isPrime(w[1][i])) {
            flag = true;
            break;
        }
        for (int i = 1; i <= even; i++) if (N[i + odd].canT && isPrime(w[0][i])) {
            flag = true;
            break;
        }

        puts(flag ? "Bulbasaur" : "Totodile");
    }
    
    return 0;
}
```

## Contest 8 - F - Totodile and Graph Theory<span id = "08f"></span>

### 题目大意

给定一张 $n$ 个点、$m$ 条边的图和一个数 $K$。要求给图进行 $K$ 染色，或输出一条长为 $K$ 的简单路径，或判断以上两问均无法实现。

$2 \leq n \leq 100,000$

$0 \leq m \leq \min(\frac{n(n - 1)}{2}, 100,000)$

$2 \leq K \leq n$

$\sum n, \sum m \leq 1,000,000$

时间限制：$1 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

随便选取一个点作为起点进行 dfs。若最深深度大于等于 $K$，则一定存在一条长度为 $K$ 的路径；否则为深度相同的点染相同的色、不同的点染不同的色，由于 dfs 树的性质，不存在一条非树边连接两个深度相同的点，即一定可以 $K$ 染色。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 100005;

struct Node;
struct Edge;

struct Node {
    Edge *e, *pre;
    int dep;
} N[MAXN];

struct Edge {
    Node *u, *v;
    Edge *next;

    Edge() {}
    Edge(Node *u, Node *v) : u(u), v(v), next(u->e) {}
} _pool[MAXN << 1], *_curr;

void addEdge(int u, int v) {
    N[u].e = new (_curr++) Edge(&N[u], &N[v]);
    N[v].e = new (_curr++) Edge(&N[v], &N[u]);
}

void dfs(Node *u) {
    for (Edge *e = u->e; e; e = e->next) if (!e->v->dep) {
        e->v->pre = e;
        e->v->dep = u->dep + 1;
        dfs(e->v);
    }
}

void clear(int n) {
    _curr = _pool;
    for (int i = 1; i <= n; i++) {
        N[i].e = N[i].pre = NULL;
        N[i].dep = 0;
    }
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n, m, k;
        scanf("%d %d %d", &n, &m, &k);
        clear(n);

        for (int i = 0, u, v; i < m; i++) {
            scanf("%d %d", &u, &v);
            addEdge(u, v);
        }

        for (int i = 1; i <= n; i++) if (!N[i].dep) {
            N[i].dep = 1;
            dfs(&N[i]);
        }

        Node *max = NULL;
        for (int i = 1; i <= n; i++) if (!max || N[i].dep > max->dep) max = &N[i];

        if (max->dep >= k) {
            printf("2\n%d", max - N);
            int cnt = 1;
            for (Edge *e = max->pre; e, cnt < k; e = e->u->pre, cnt++) printf(" %d", e->u - N);
            puts("");
        } else {
            puts("1");
            for (int i = 1; i <= n; i++) printf("%d%c", N[i].dep, " \n"[i == n]);
        }
    }
    return 0;
}
```

## Contest 8 - G - Totodile and Another Data Structure Problem?<span id = "08g"></span>

### 题目大意

给定一个长为 $n$ 的序列 $\{a_i\} ~ (a_i \in \{1, 2\})$。定义一个数 $g$ 是好的，当且仅当存在区间 $[l, r]$ 满足 $\sum_{i = l}^{r} a_i = g$。求所有不超过 $k$ 的好数的异或和。

$1 \leq n \leq 100,000$

$1 \leq k \leq 10^9$

$\sum n \leq 1,000,000$

时间限制：$1 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

可以证明，一个至少有一端为 $1$ 的序列的好数为 $[1, \sum a_i]$ 内的所有整数（称这种序列为好序列）。对于两端均为 $2$ 的序列，可以拆成一段连续的 $2$ 和一个好序列 $\{b_i\}$。枚举 $2$ 的个数 $k$，则 $2k + \sum b_i$ 为好数。最后直接计算异或和即可。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 100005;

int sum[MAXN], pre[MAXN], a[MAXN], sor[MAXN << 1];

void init() {
    for (int i = 1; i < MAXN << 1; i++) sor[i] = sor[i - 1] ^ i;
}

int main() {
    init();

    int T;
    scanf("%d", &T);
    while (T--) {
        int n, k;
        scanf("%d %d", &n, &k);

        for (int i = 1; i <= n; i++) scanf("%d", &a[i]);
        sum[0] = pre[n + 1] = 0;
        for (int i = 1; i <= n; i++) sum[i] = sum[i - 1] + a[i];
        for (int i = n; i; i--) pre[i] = pre[i + 1] + a[i];
        k = std::min(k, sum[n]);

        if (!(a[1] == a[n] && a[1] == 2)) {
            printf("%d\n", sor[k]);
            continue;
        }

        int left = -1, right = -1;
        for (int i = 1; i <= n; i++) if (a[i] == 1) {
            left = i - 1;
            break;
        }
        if (left == -1) left = n;
        for (int i = n; i; i--) if (a[i] == 1) {
            right = i + 1;
            break;
        }
        if (right == -1) right = 1;

        int lc = left, rc = n - right + 1, S;
        if (lc < rc) S = pre[left + 1];
        else S = sum[right - 1];
        int ans = sor[std::min(S, k)];
        for (int i = 1; i <= std::min(lc, rc); i++) {
            if (S + 2 * i <= k) ans ^= (S + 2 * i);
            else break;
        }

        printf("%d\n", ans);
    }
    return 0;
}
```

## Contest 9 - A - LYK and Heltion's gifts<span id = "09a"></span>

### 题目大意

用两种颜色染一个有 $n$ 个珠子的首饰，要求不能有相邻的黑色珠子，求可能的染色方案数，旋转相同视为同一种，答案对 $1,000,000,007$ 取模。

$2 \leq n \leq 10^6$

$1 \leq T \leq 1,000$

时间限制：$2 \text{s}$，内存限制：$512 \text{MB}$。

### 题解

旋转同构用 Burnside 引理处理。对于不考虑旋转时的答案（记为 $f(n)$）可以发现是：

$$
\begin{align}
f(2) &= 3 \\
f(3) &= 4 \\
f(n) &= f(n - 1) + f(n - 2)
\end{align}
$$

### 代码

```c++
#include <cstdio>

const int MAXN = 1000005;
const int MOD = 1000000007;

int phi[MAXN], prime[MAXN], primeCnt;

void sieve() {
    static bool notPrime[MAXN];
    notPrime[0] = notPrime[1] = true;
    primeCnt = 0;
    for (int i = 2; i < MAXN; i++) {
        if (!notPrime[i]) {
            prime[primeCnt++] = i;
            phi[i] = i - 1;
        }

        for (int j = 0; j < primeCnt && i * prime[j] < MAXN; j++) {
            notPrime[i * prime[j]] = true;
            if (i % prime[j] == 0) {
                phi[i * prime[j]] = phi[i] * prime[j];
                break;
            }
            phi[i * prime[j]] = phi[i] * (prime[j] - 1);
        }
    }
}

long long f[MAXN], inv[MAXN];

void pre() {
    sieve();

    f[1] = 1;
    f[2] = 3;
    f[3] = 4;
    for (int i = 4; i < MAXN; i++) {
        f[i] = f[i - 1] + f[i - 2];
        f[i] >= MOD ? f[i] -= MOD : 0;
    }

    inv[1] = 1;
    for (int i = 2; i < MAXN; i++) inv[i] = (MOD - MOD / i) * inv[MOD % i] % MOD;
}

long long calc(int n) {
    int x = n;
    long long res = f[n] + phi[n] * f[1] % MOD;
    res >= MOD ? res -= MOD : 0;
    for (int i = 2; i <= n / 2; i++) if (n % i == 0) {
        int t = phi[n / i];
        res += f[i] * t % MOD;
        res >= MOD ? res -= MOD : 0;
    }
    res = res * inv[n] % MOD;
    return res;
}

int main() {
    pre();

    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);
        
        long long ans = calc(n);
        printf("%lld\n", ans);
    }
    
    return 0;
}
```

## Contest 9 - H - LYK and Painting<span id = "09h"></span>

### 题目大意

长为 $N$ 的序列上，每次可以让一段长为 $K$ 的子区间染成同一种颜色（可覆盖），一共有 $M$ 种颜色。最终使得每个位置上都有颜色，求能得到的最终序列的种数，答案对 $1,000,000,007$ 取模。

$1 \leq N \leq 2^{31} - 1$

$1 \leq M \leq 100,000$

$1 \leq K \leq 100$

$1 \leq T \leq 10$，大测试点不超过 $3$ 个。

时间限制：$2 \text{s}$，内存限制：$512 \text{MB}$。

### 题解

最终的序列中，必有一段长度至少为 $K$ 的子区间上颜色相同。由于 $N$ 太大，我们可以考虑用 $M^N$ 减去不存在这样子区间的种数。记 $f(n)$ 为长为 $n$ 的、不存在这样子区间的种数，则转移为：

$$
f(n) = \begin{cases}
M^n &1 \leq n < M \\
(M - 1) \sum_{i = 1}^{K - 1} f(n - 1) &n \geq M
\end{cases}
$$

用矩阵乘法快速幂优化即可。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXK = 105;
const int MOD = 1000000007;

struct Matrix {
    long long a[MAXK][MAXK];
    int n, m;

    Matrix(int n, int m, bool eye = false) : n(n), m(m) {
        for (int i = 0; i < n; i++) std::fill(a[i], a[i] + m, 0);
        if (eye) for (int i = 0; i < n; i++) a[i][i] = 1;
    }

    long long *operator[](int index) {
        return a[index];
    }
    const long long *operator[](int index) const {
        return a[index];
    }

    Matrix operator*(const Matrix &rhs) const {
        Matrix res(n, rhs.m);
        for (int i = 0; i < n; i++) for (int j = 0; j < m; j++) if (a[i][j]) {
            for (int k = 0; k < rhs.m; k++) {
                res[i][k] += a[i][j] * rhs[j][k] % MOD;
                res[i][k] >= MOD ? res[i][k] -= MOD : 0;
            }
        }
        return res;
    }
};

Matrix pow(Matrix &a, long long n) {
    Matrix res(a.n, a.m, true);
    for (; n; n >>= 1, a = a * a) if (n & 1) res = res * a;
    return res;
}

long long qpow(long long a, long long n) {
    long long res = 1;
    for (; n; n >>= 1, a = a * a % MOD) if (n & 1) res = res * a % MOD;
    return res;
}

int main() {
    int n, m, k;
    while (scanf("%d %d %d", &n, &m, &k) == 3) {
        long long ans = qpow(m, n);

        if (n < k) {
            puts("0");
            continue;
        }

        Matrix A(k - 1, 1);
        for (int i = 0; i < k - 1; i++) A[i][0] = qpow(m, k - i - 1);

        Matrix T(k - 1, k - 1);
        for (int i = 0; i < k - 1; i++) {
            T[0][i] = m - 1;
            i < k - 2 ? T[i + 1][i] = 1 : 0;
        }

        Matrix temp = pow(T, n - k + 1);
        temp = temp * A;

        ans = (ans - temp[0][0] + MOD) % MOD;
        printf("%lld\n", ans);
    }
    
    return 0;
}
```