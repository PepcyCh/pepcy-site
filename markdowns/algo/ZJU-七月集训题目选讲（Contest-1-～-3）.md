---
title: ZJU 七月集训题目选讲（Contest 1 ～ 3）
date: 2018-07-28 18:18:33
tags: [ZJU, 单调队列, DP, 贪心, 数位DP, 计算几何, 凸包, 单源最短路, 并查集, 线段树]
categories: 题解（OI/XCPC）
---

# ZJU 七月集训题目选讲（Contest 1 ～ 3）

ZJU 七月集训（2018.7.13 ～ 2018.7.26）的题目。

* Contest 1 by SBconscious - 2018.7.13
* Contest 2 by Astolfo - 2018.7.14
* COntest 3 by bits/stdc++.h - 2018.7.15

所有题目都是多组数据，若无子测试点数的说明，则 $T$ 不会特别大。

## 目录

[Contest 1 - A - Saber](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#01a)

[Contest 1 - B - Lancer](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#01b)

[Contest 1 - G - Berserker](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#01g)

[Contest 2 - C - Ibrahimovic and the Angry Cashier](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#02c)

[Contest 2 - D - Kylian Mbappe and His Challenge](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#02d)

[Contest 2 - G - Neuer and His Manhattan Counting](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#02g)

[Contest 3 - A - #include &lt;set>](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#03a)

[Contest 3 - B - #include &lt;numeric>](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#03b)

[Contest 3 - C - #include &lt;ctime>](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#03c)

[Contest 3 - E - #include &lt;deque>](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#03e)

[Contest 3 - G - #include &lt;bits/stdc++.h>](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-1-%EF%BD%9E-3%EF%BC%89/#03g)

<!--more-->

## Contest 1 - A - Saber<span id = "01a"></span>

### 题目大意

给定一棵 $n$ 个节点的树，每条边有一个权值 $w$。有 $Q$ 次询问，每次询问两个点，判断它们之间的路径上的边中，是否存在三条边使得它们的权值可以组成一个三角形。

$1 \leq n, Q \leq 100,000$

$1 \leq w \leq 10^9$

$1 \leq T \leq 5$

时间限制：$2 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

权值最小的一段不能组成三角形的权值是斐波那契数列，而斐波那契数列的第 $45$ 项已经超过了 $10^9$ ，所以，当路径上的边数超过 $45$ 时即可直接输出 `YES`，否则暴力判断。

### 代码

```c++
#include <cstdio>
#include <set>
#include <algorithm>

const int MAXN = 100005;
const int MAXN_LOG = 18;

struct Edge;
struct Node;

struct Node {
    Edge *e;
    Node *f[MAXN_LOG];
    int dep, w;
} N[MAXN];

struct Edge {
    Node *u, *v;
    Edge *next;
    int w;

    Edge() {}
    Edge(Node *u, Node *v, int w) : u(u), v(v), w(w), next(u->e) {}
} _pool[MAXN << 1], *_curr;

void addEdge(int u, int v, int w) {
    N[u].e = new (_curr++) Edge(&N[u], &N[v], w);
    N[v].e = new (_curr++) Edge(&N[v], &N[u], w);
}

void dfs(Node *u, Node *fa = NULL) {
    u->dep = (fa ? fa->dep : 0) + 1;
    u->f[0] = fa;

    for (int i = 1; i < MAXN_LOG; i++) {
        if (u->f[i - 1]) u->f[i] = u->f[i - 1]->f[i - 1];
        else break;
    }

    for (Edge *e = u->e; e; e = e->next) if (e->v != fa) {
        e->v->w = e->w;
        dfs(e->v, u);
    }
}

std::pair<Node *, int> lca(Node *u, Node *v) {
    int dist = 0;

    if (u->dep < v->dep) std::swap(u, v);

    for (int i = MAXN_LOG - 1; ~i; i--) if (u->f[i] && u->f[i]->dep >= v->dep) {
        dist += (1 << i);
        u = u->f[i];
    }

    for (int i = MAXN_LOG - 1; ~i; i--) if (u->f[i] && v->f[i] && u->f[i] != v->f[i]) {
        dist += (1 << (i + 1));
        u = u->f[i];
        v = v->f[i];
    }

    if (u != v) {
        u = u->f[0];
        dist += 2;
    }

    return std::make_pair(u, dist);
}

bool brute(Node *u, Node *v, Node *lca) {
    static std::set<int> set;
    set.clear();

    for (; u != lca; u = u->f[0]) set.insert(u->w);
    for (; v != lca; v = v->f[0]) set.insert(v->w);

    bool res = false;
    for (auto i = set.begin(); i != set.end(); i++) {
        auto j = i;
        ++j;
        auto k = j;
        ++k;
        if (k == set.end()) break;

        if ((*i) + (*j) > (*k)) {
            res = true;
            break;
        }
    }

    return res;
}

void init(int n) {
    _curr = _pool;
    for (int i = 1; i <= n; i++) N[i].e = NULL;
}

int main() {
    int T;
    scanf("%d", &T);
    for (int kase = 1; kase <= T; kase++) {
        printf("Case #%d:\n", kase);

        int n;
        scanf("%d", &n);
        init(n);

        for (int i = 1, u, v, w; i < n; i++) {
            scanf("%d %d %d", &u, &v, &w);
            addEdge(u, v, w);
        }

        dfs(&N[1]);

        int q;
        scanf("%d", &q);
        while (q--) {
            int u, v;
            scanf("%d %d", &u, &v);

            auto ret = lca(&N[u], &N[v]);

            bool ans = ret.second < 44 ? (ret.second >= 3 ? brute(&N[u], &N[v], ret.first) : false) : true;
            puts(ans ? "Yes" : "No");
        }
    }
    
    return 0;
}
```

## Contest 1 - B - Lancer<span id = "01b"></span>

### 题目大意

有 $n$ 个物品，每个物品有一个字符（小写字母）和一个权值（$1 \sim 9$）。从这些物品中连续取出 $i \times l$ 个，其中 $i \geq S$，$l \in [L, R]$，并且满足对于这 $i$ 个长为 $l$ 的子段，不存在某两个子段在某一个相对位置上的字符相同，可获得它们权值和的价值。求可获得的最大价值。

$1 \leq n \leq 500,000$

$0 \leq R - L \leq 10$

$1 \leq T \leq 10, \sum n \leq 2,500,000$

时间限制：$2 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

注意到 $R - L$ 很小，我们可以枚举长度 $l$。对于每一个长度，暴力搞出从某一个位置开始，每次向后跳 $l$ 个物品所能到的最远距离，答案就是每一个长为 $l$ 的连续子段中该最远距离的最小值，可以用单调队列来维护。

### 代码

```c++
#include <cstdio>
#include <queue>
#include <algorithm>

const int MAXN = 500005;

char str[MAXN], num[MAXN];
int sum[MAXN], max[MAXN];

struct MonoQ {
    std::deque<int> q, m;

    void push(int x) {
        q.push_back(x);
        while (!m.empty() && m.back() > x) m.pop_back();
        m.push_back(x);
    }

    void pop() {
        int x = q.front();
        q.pop_front();
        if (x == m.front()) m.pop_front();
    }

    int top() {
        return m.front();
    }

    void clear() {
        q.clear();
        m.clear();
    }
} q;

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n, l, r, s;
        scanf("%d %d %d %d %s %s", &n, &l, &r, &s, str, num); 

        sum[0] = 0;
        for (int i = 0; i < n; i++) sum[i + 1] = sum[i] + num[i] - '0';

        int ans = 0;
        for (int len = l; len <= std::min(r, n); len++) {
            for (int x = 0; x < n; x++) {
                static bool exist[128];
                std::fill(exist, exist + 128, false);
                exist[str[x]] = true;

                int i;
                for (i = len; x + i < n; i += len) {
                    if (!exist[str[x + i]]) exist[str[x + i]] = true;
                    else break;
                }
                max[x] = i / len;
            }

            q.clear();
            for (int i = 0; i < len - 1; i++) q.push(max[i]);
            for (int i = len - 1; i < n; i++) {
                q.push(max[i]);
                if (q.top() >= s) ans = std::max(ans, sum[i + (q.top() - 1) * len + 1] - sum[i - len + 1]);
                q.pop();
            }
        }

        printf("%d\n", ans);
    }
    
    return 0;
}
```

## Contest 1 - G - Berserker<span id = "01g"></span>

### 题目大意

求从 $1 \sim n$ 中选出不超过 $k$ 个不同的数满足它们的乘积不是除 $1$ 外其他完全平方数的倍数的方案数，答案对 $1,000,000,007$ 取模。

$1 \leq n, k \leq 500$

$1 \leq T \leq 10$

时间限制：$2 \text{s}$，内存限制：$512 \text{MB}$。

### 题解

对每个数进行质因数分解并按质因子分组，每组最多选一个数以保证乘积无平方因子（忽略有平方因子的数）。但有一些数可以被分到多个组中。

注意到 $19^2 = 361, 23^2 = 529$，即只有 $\{2, 3, 5, 7, 11, 13, 17, 19\}$ 的倍数会被分到多组。除去这些质因子后，每个数可以被分到唯一的组。

对于这 $8$ 个质因子，用二进制位的方式保证乘积无平方因子。

具体地说，对于只含这 $8$ 个质因子的数，我们进行如下 DP 转移：
$$
f(j, s) = \sum_{g_i \subset s} f(j - 1, s \oplus g_i)
$$
其中 $f(i, s)$ 表示选择了 $i$ 个数，它们乘积的质因子为 $s$（状压表示）的方案数，$g_i$ 表示数 $i$ 的质因子（状压表示）。

对于其他的组，也进行类似的 DP，但是其他组需要满足组内只能选一个数。我们可以通过改变枚举顺序来实现。

### 代码

```c++
#include <cstdio>
#include <cstring>
#include <vector>
#include <algorithm>

const int MAXN = 505;
const int MOD = 1000000007;

const int PRIME[8] = {2, 3, 5, 7, 11, 13, 17, 19};

std::vector<int> vec[MAXN];

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n, k;
        scanf("%d %d", &n, &k);

        for (int i = 1; i <= n; i++) vec[i].clear();

        for (int i = 1; i <= n; i++) {
            int x = i, temp = 0;
            bool flag = false;
            for (int j = 0; j < 8; j++) if (x % PRIME[j] == 0) {
                x /= PRIME[j];
                if (x % PRIME[j] == 0) {
                    flag = true;
                    break;
                }
                temp |= (1 << j);
            }
            if (flag) continue;
            vec[x].push_back(temp);
        }

        static int f[MAXN][1 << 8];
        memset(f, 0, sizeof (f));
        f[0][0] = 1;

        for (auto i : vec[1]) {
            for (int j = k; j; j--) for (int s = 0; s < (1 << 8); s++) if (f[j - 1][s] && !(s & i)) {
                f[j][s | i] += f[j - 1][s];
                f[j][s | i] >= MOD ? f[j][s | i] -= MOD : 0;
            }
        }

        for (int i = 2; i <= n; i++) if (!vec[i].empty()) {
            for (int j = k; j; j--) for (int s = 0; s < (1 << 8); s++) if (f[j - 1][s]) for (auto p : vec[i]) if (!(s & p)) {
                f[j][s | p] += f[j - 1][s];
                f[j][s | p] >= MOD ? f[j][s | p] -= MOD : 0;
            }
        }

        int ans = 0;
        for (int i = 1; i <= k; i++) for (int s = 0; s < (1 << 8); s++) {
            ans += f[i][s];
            ans >= MOD ? ans -= MOD : 0;
        }

        printf("%d\n", ans);
    }

    return 0;
}
```

## Contest 2 - C - Ibrahimovic and the Angry Cashier<span id = "02c"></span>

### 题目大意

有 $m$ 张一元的硬币和无限张一百元的纸币，进行 $n$ 次购物，每次的价格为 $c_i$，每次售货员会获得 $x_i \times w_i$ 的怒气值，其中 $x_i$ 为本次需要找零的钱数。售货员找零时只会给一百元的纸币和少于 $100$ 个的一元的硬币，并且这些钱可以用于后续的购物。求 $n$ 次购物后售货员怒气值的最小值。

$1 \leq n, c_i, w_i \leq 100,000$

$0 \leq m \leq 10^9$

$1 \leq T \leq 20$

时间限制：$1 \text{s}$，内存限制：$256 \text{MB}$。

### 题解

显然，所有的 $c_i$ 可以对 $100$ 取模。每次都用硬币付款，当硬币不够时，可以从之前的几次中选择未选择过的、 $(100 - c_i)w_i$ 最小的一次，使用纸币付款，相当于换到了 $100$ 个硬币。用优先队列维护最小的 $(100 - c_i)w_i$ 即可。

### 代码

```c++
#include <cstdio>
#include <queue>
#include <algorithm>

const int MAXN = 100005;

int c[MAXN], w[MAXN];

int main() {
    int T;
    scanf("%d", &T);
    while(T--) {
        int n, m;
        scanf("%d %d", &n, &m);

        for (int i = 0; i < n; i++) {
            scanf("%d", &c[i]);
            c[i] %= 100;
        }
        for (int i = 0; i < n; i++) {
            scanf("%d", &w[i]);
            w[i] = w[i] * (100 - c[i]);
        }

        long long ans = 0;
        static std::priority_queue<int, std::vector<int>, std::greater<int> > q;
        while (!q.empty()) q.pop();

        for (int i = 0; i < n; i++) if (c[i]) {
            q.push(w[i]);
            m -= c[i];
            if (m < 0) {
                ans += q.top();
                q.pop();
                m += 100;
            }
        }

        printf("%lld\n", ans);
    }
    
    return 0;
}
```

## Contest 2 - D - Kylian Mbappe and His Challenge<span id = "02d"></span>

### 题目大意

有两个数 $a$ 和 $b$，初始时 $a = 1, b = 0$，每次操作可以让 $a = a \times b$ 或让 $b = b + 1$。求通过不超过 $p$ 次操作，可以得到多少个在 $[l, r]$ 范围内的数。

$0 \leq l, r \leq 10^9$

$0 \leq q \leq 100$

$1 \leq T \leq 10$， $r \geq 10^8$ 的子测试点不超过 $4$ 个。

时间限制：$5 \text{s}$，内存限制：$256 \text{MB}$。

### 题解

因为 $p \leq 100$，所以得到的数的质因数不会超过 $100$。用 $100$ 以内的质数 dfs 一遍求出用它们能得到的数（数量级在 $10^6$）并排序，用 $a_i$ 表示这些数中的第 $i$ 个（排序后）。

$f(i)$ 表示得到 $a_i$ 所需的最少操作， $g(i)$ 表示得到 $a_i$ 所需的最少的乘法操作，则转移为：
$$
g(i) = \min\limits_{1 \leq k \leq 100 \text{ and } a_j \times k = a_i} g(j) + 1 \\
f(i) = \min\limits_{1 \leq k \leq 100 \text{ and } a_j \times k = a_i} g(i) + k
$$
 对同一个 $k$，满足 $a_j \times k = a_i$ 的 $i$ 是随 $j$ 递增的。我们先枚举 $k$，后枚举 $j$，记录指针地扫可得到满足条件的 $i$，并更新 $g(i)$ 和 $f(i)$。

询问时在 $\{a_i\}$ 里枚举判断即可。

### 代码

```c++
#include <cstdio>
#include <vector>
#include <algorithm>

const int MAXA = 1000000000;
const int MAXN = 3000000;
const int PRIME_CNT = 25;
const int PRIME[PRIME_CNT] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97};

std::vector<long long> num;
void dfs(int p, long long curr) {
    if (p == PRIME_CNT) {
        num.push_back(curr);
        return;
    }

    do {
        dfs(p + 1, curr);
        curr *= PRIME[p];
    } while (curr <= MAXA);
}

int mulCnt[MAXN], cnt[MAXN];
void pre() {
    dfs(0, 1);
    std::sort(num.begin(), num.end());

    for (int i = 0; i < num.size(); i++) {
        mulCnt[i] = MAXA;
        cnt[i] = num[i];
    }
    mulCnt[0] = cnt[0] = 0;
    for (int i = 1; i <= 100; i++) {
        int k = 0;
        for (int j = 0; j < num.size() && num[j] * i <= MAXA; j++) {
            while (num[k] < num[j] * i && k < num.size()) ++k;
            if (num[k] == num[j] * i) {
                mulCnt[k] = std::min(mulCnt[k], mulCnt[j] + 1);
                cnt[k] = std::min(cnt[k], mulCnt[k] + i);
            }
        }
    }
}

int main() {
    pre();

    int T;
    scanf("%d", &T);
    while (T--) {
        int l, r, p;
        scanf("%d %d %d", &l, &r, &p);

        int ans = 0;
        for (int i = 0; i < num.size(); i++) if (l <= num[i] && num[i] <= r && cnt[i] <= p) ++ans;
        printf("%d\n", ans);
    }
    
    return 0;
}
```

## Contest 2 - G - Neuer and His Manhattan Counting<span id = "02g"></span>

### 题目大意

平面上有 $K$ 个整点 $(x_i, y_i)$ ，其中 $1 \leq x_i、y_i \leq N$。定义函数
$$
F(u, v) = \sum_{i = 1}^{K} f(u, v, i) \\
f(u, v, i) = \begin{cases}
(u - x_i) + (v - y_i) &u \geq x_i \text{ and } v \geq y_i \\
0 &\text{else}
\end{cases}
$$
有 $Q$ 次询问，每次询问有多少 $1 \leq u、 v \leq N$ 的整点 $(u, v)$ 满足 $F(u, v) = C_i$。

$1 \leq K, N \leq 10^5$

$1 \leq Q \leq 10$

$0 \leq C_i \leq 10^9$

$N, K \geq 10^4$ 的子测试点不超过 $20$ 个，小测试点有 $1,000$ 个。

时间限制：$2 \text{s}$，内存限制：$256 \text{MB}$。

### 题解

在同一 $x$轴上 $F(u, v)$ 随 $y$ 坐标递增，同一 $y$轴同理。我们从点 $(1, N)$ 开始，按如下方式扫一遍统计答案：

* $F(u, v) > C$ ：下移一格；
* $F(u, v) = C$ ：下移一格，右移一格；
* $F(u, v) > C$ ：右移一格。

考虑如何快速计算 $F(u, v)$ ：我们动态地维护 $(u, v)$ 左下部分的点数 $c$ 和这些点的横纵坐标和 $sum$，则 $F(u, v) = (u + y)c - sum$。把点按横、纵坐标各排一个序，在两个数组里记录指针地扫即可在移动时更新 $c$ 和 $sum$。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 100005;

struct Point {
    int x, y;

    Point(int x = 0, int y = 0) : x(x), y(y) {}
} X[MAXN], Y[MAXN], *px, *py;
bool cmpX(const Point &a, const Point &b) {
    return a.x == b.x ? a.y < b.y : a.x < b.x;
}
bool cmpY(const Point &a, const Point &b) {
    return a.y == b.y ? a.x < b.x : a.y < b.y;
}

int count, n, k;
long long sum;
int calc(int x, int y, long long c) {
    int res = 0;
    bool flag = false;
    while (y > 0 && x <= n) {
        for (; px->x <= x && px < X + k; px++) if (px->y <= y) {
            ++count;
            sum += px->x + px->y;
        }
        for (; py->y > y && py >= Y; py--) if (py->x <= (flag ? x - 1 : x)) {
            --count;
            sum -= py->x + py->y;
        }

        long long F = (long long) count * (x + y) - sum;
        if (F == c) {
            ++res;
            ++x;
            --y;
            flag = true;
        } else if (F > c) {
            --y;
            flag = false;
        } else {
            ++x;
            flag = false;
        }
    }
    return res;
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        scanf("%d %d", &n, &k);
        for (int i = 0; i < k; i++) scanf("%d %d", &X[i].x, &X[i].y);

        std::copy(X, X + k, Y);
        std::sort(X, X + k, cmpX);
        std::sort(Y, Y + k, cmpY);

        int q;
        scanf("%d", &q);
        while (q--) {
            long long c;
            scanf("%lld", &c);

            count = 0;
            sum = 0;
            px = X;
            py = Y + k - 1;

            int ans = calc(1, n, c);
            printf("%d%c", ans, "\n "[q]);
        }
    }
    
    return 0;
}
```

## Contest 3 - A - include &lt;set><span id = "03a"></span>

### 题目大意

给定 $n$ 个点，每个点有一个质量 $m_i$。你可以平移或旋转点集，但必须：

* 保持各个点之间的相对位置不变；
* 点的纵坐标不得为负。

以 $x$轴为零势面，最小化点的重力势能和。

$0 \leq n, x, y \leq 10,000$

$0 \leq m \leq 100$

时间限制：$1 \text{s}$，内存限制：$256 \text{MB}$。

### 题解

答案最小时，凸包的一条边与 $x$轴重合。求出重心和凸包，然后求出重心到凸包上的边的最短距离即可。

### 代码

```c++
#include <cstdio>
#include <cmath>
#include <cfloat>
#include <algorithm>

const int MAXN = 100005;

struct Point {
    double x, y;

    Point(double x = 0, double y = 0) : x(x), y(y) {}

    bool operator<(const Point &rhs) {
        return x == rhs.x ? y < rhs.y : x < rhs.x;
    }

    Point operator+(const Point &rhs) const {
        return Point(x + rhs.x, y + rhs.y);
    }

    Point operator-(const Point &rhs) const {
        return Point(x - rhs.x, y - rhs.y);
    }

    Point operator*(double rhs) const {
        return Point(x * rhs, y * rhs);
    }

    friend double cross(const Point &a, const Point &b) {
        return a.x * b.y - a.y * b.x;
    }
} P[MAXN], ch[MAXN], M;
int mass[MAXN];

int getConvexHell(int n) {
    int p = 1;
    std::sort(P, P + n);
    for (int i = 1; i < n; i++) if (P[i].x != P[i - 1].x || P[i].y != P[i - 1].y) P[p++] = P[i];
    n = p;
    std::sort(P, P + n);

    int m = 0;
    for (int i = 0; i < n; i++) {
        while (m > 1 && cross(ch[m - 1] - ch[m - 2], P[i] - ch[m - 2]) <= 0) m--;
        ch[m++] = P[i];
    }

    int k = m;
    for (int i = n - 1; ~i; i--) {
        while (m > k && cross(ch[m - 1] - ch[m - 2], P[i] - ch[m - 2]) <= 0) m--;
        ch[m++] = P[i];
    }

    m > 1 ? --m : 0;
    return m;
}

double distFromPointToLine(const Point &p, const Point &a, const Point &b) {
    double A = a.y - b.y;
    double B = b.x - a.x;
    double C = cross(a, b);
    return std::abs(A * p.x + B * p.y + C) / sqrt(A * A + B * B);
}

int main() {
    int n;
    while (scanf("%d", &n), n) {
        M = Point(0, 0);
        int sumM = 0;
        for (int i = 0; i < n; i++) {
            scanf("%d %lf %lf", &mass[i], &P[i].x, &P[i].y);
            M = M + P[i] * mass[i];
            sumM += mass[i];
        }
        M = M * (1.0 / sumM);

        int m = getConvexHell(n);
        double ans = m == 1 ? 0 : DBL_MAX;
        ch[m] = ch[0];

        for (int i = 0; i < m; i++) {
            double dist = distFromPointToLine(M, ch[i], ch[i + 1]);
            ans = std::min(ans, dist * sumM * 9.8);
        }

        printf("%.4f\n", ans);
    }
    
    return 0;
}
```

## Contest 3 - B - include &lt;numeric><span id = "03b"></span>

### 题目大意

求 $[L, R]$ 内有多少数满足偶数位和与奇数位和的最大公约数不超过 $k$。

$1 \leq L \leq R \leq 10^{18}$

$1 \leq T \leq 2,000$

时间限制：$1 \text{s}$，内存限制：$256 \text{MB}$。

### 题解

直接数位 DP 会超时。

因为 $k$ 相同的点，数为 DP 的数组在不顶头时的值是相同的，所以我们先读入所有的测试点，按 $k$ 排序，同一 $k$ 下 DP 时只清空顶头的位置的值即可。

（「顶头」指下一位的数不能随便选的位置）

### 代码

```c++
#include <cstdio>
#include <cstring>
#include <algorithm>

const int MAXT = 2005;
const int MAXBIT = 19;

long long F[MAXBIT + 1][100][100][2];
int num[20], K;

long long calc(int i, int f, int g, int limits) {
    if (i == 0) return F[i][f][g][limits] = int(f && g && std::__gcd(f, g) <= K);
    if (F[i][f][g][limits] != -1) return F[i][f][g][limits];

    F[i][f][g][limits] = 0;
    if (!limits) for (int n = 0; n <= 9; n++) {
        if (i % 2) F[i][f][g][limits] += calc(i - 1, f + n, g, 0);
        else F[i][f][g][limits] += calc(i - 1, f, g + n, 0);
    } else {
        for (int n = 0; n < num[i]; n++) {
            if (i % 2) F[i][f][g][limits] += calc(i - 1, f + n, g, 0);
            else F[i][f][g][limits] += calc(i - 1, f, g + n, 0);
        }
        if (i % 2) F[i][f][g][limits] += calc(i - 1, f + num[i], g, 1);
        else F[i][f][g][limits] += calc(i - 1, f, g + num[i], 1);
    }

    return F[i][f][g][limits];
}

long long calc(long long M) {
    num[0] = 0;
    for (int i = 1; i <= MAXBIT; i++) {
        num[i] = M % 10;
        M /= 10;
    }

    long long res = calc(MAXBIT, 0, 0, 1);

    return res;
}

void clear() {
    int o = 0, e = 0;
    for (int i = MAXBIT; ~i; i--) {
        F[i][o][e][1] = -1;

        if (i % 2) o += num[i];
        else e += num[i];
    }
}

struct Query {
    long long l, r, ans;
    int k, id;

    Query() {}
    Query(long long l, long long r, int k, int id) : l(l), r(r), k(k), id(id) {}
} Q[MAXT];

bool compareByK(const Query &a, const Query &b) {
    return a.k < b.k;
}

bool compareById(const Query &a, const Query &b) {
    return a.id < b.id;
}

int main() {
    memset(F, -1, sizeof (F));
    int T;
    scanf("%d", &T);
    for (int i = 0; i < T; i++) {
        long long l, r, k;
        scanf("%lld %lld %lld", &k, &l, &r);
        if (k > 90) k = 90;
        Q[i] = Query(l, r, k, i);
    }

    std::sort(Q, Q + T, compareByK);

    int p = 0;
    for (K = 1; K <= 90; K++) {
        for (; Q[p].k == K; p++) {
            long long L = calc(Q[p].l - 1);
            clear();

            long long R = calc(Q[p].r);
            clear();

            Q[p].ans = R - L;
        }

        memset(F, -1, sizeof (F));
    }

    std::sort(Q, Q + T, compareById);
    for (int i = 0; i < T; i++) printf("%lld\n", Q[i].ans);

    return 0;
}
```

## Contest 3 - C - include &lt;ctime><span id = "03c"></span>

### 题目大意

给定一张 $n$ 个顶点、$m$ 条边的带权无向图，求从 $1$ 号顶点出发、又回到 $1$ 号顶点的最短路径距离。

$1 \leq n \leq 10,000$

$1 \leq m \leq 40,000$

$1 \leq w_i \leq 10,000$

$1 \leq T \leq 5$

时间限制：$1 \text{s}$，内存限制：$256 \text{MB}$。

### 题解

给与 $1$ 号顶点相邻的顶点重新编号，按二进制位分类设为起点与终点跑多源最短路并更新答案。

### 代码

```c++
#include <cstdio>
#include <climits>
#include <vector>
#include <queue>
#include <algorithm>

const int MAXN = 10005;
const int MAXN_LOG = 14;
const int MAXM = 40005;

struct Node;
struct Edge;

struct Node {
    Edge *e;
    bool vis;
    int dist;
} N[MAXN];

struct Edge {
    Node *u, *v;
    Edge *next;
    int w;

    Edge() {}
    Edge(Node *u, Node *v, int w) : u(u), v(v), w(w), next(u->e) {}
} _pool[MAXM << 1], *_curr;

void addEdge(int u, int v, int w) {
    N[u].e = new (_curr++) Edge(&N[u], &N[v], w);
    N[v].e = new (_curr++) Edge(&N[v], &N[u], w);
}

std::vector<std::pair<int, int> > vec;

namespace Dijkstra {
struct HeapNode {
    Node *u;
    int dist;

    HeapNode() {}
    HeapNode(Node *u, int dist) : u(u), dist(dist) {}

    bool operator<(const HeapNode &rhs) const {
        return dist > rhs.dist;
    }
};

std::priority_queue<HeapNode> q;

void solve() {
    while (!q.empty()) {
        Node *u = q.top().u;
        q.pop();

        if (u->vis) continue;
        u->vis = true;

        for (Edge *e = u->e; e; e = e->next) if (e->v->dist > u->dist + e->w) {
            e->v->dist = u->dist + e->w;
            q.push(HeapNode(e->v, e->v->dist));
        }
    }
}

void init(int n) {
    for (int i = 1; i <= n; i++) {
        N[i].vis = false;
        N[i].dist = INT_MAX >> 1;
    }
}
} // namespace Dijkstra

void clear(int n) {
    for (int i = 1; i <= n; i++) N[i].e = NULL;
    _curr = _pool;
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n, m;
        scanf("%d %d", &n, &m);
        clear(n);

        vec.clear();
        for (int i = 0, u, v, w; i < m; i++) {
            scanf("%d %d %d", &u, &v, &w);
            if (u != 1 && v != 1) addEdge(u, v, w);
            else {
                if (v == 1) std::swap(u, v);
                vec.push_back(std::make_pair(v, w));
            }
        }

        int ans = INT_MAX;
        for (int bit = 0; (1 << bit) < vec.size(); bit++) {
            Dijkstra::init(n);
            for (int i = 0; i < vec.size(); i++) if (i & (1 << bit)) {
                N[vec[i].first].dist = vec[i].second;
                Dijkstra::q.push(Dijkstra::HeapNode(&N[vec[i].first], vec[i].second));
            }
            Dijkstra::solve();

            for (int i = 0; i < vec.size(); i++) if (!(i & (1 << bit)))
                ans = std::min(ans, N[vec[i].first].dist + vec[i].second);
        }

        printf("%d\n", ans == INT_MAX ? -1 : ans);
    }
    
    return 0;
}
```

## Contest 3 - E - #include &lt;deque><span id = "03e"></span>

### 题目大意

要清空一个目前有 $n$ 个元素的双端队列，每个元素为 $w_i$。从左侧弹出元素会有 $w_i \times L$ 的不高兴值，从右侧则会有 $w_i \times R$ 的不高兴值。如果本次与上次均从左侧弹出，会有 $q_L$ 的额外不高兴值，右侧则会有 $q_R$ 的额外不高兴值。求最小的不高兴值。

$1 \leq n \leq 100,000$

$1 \leq L, R, q_L, q_R \leq 100$

时间限制：$2 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

以双端队列的某个位置位为界，左侧的只能从左侧弹出，右侧的只能从右侧弹出，为了最小化不高兴值，弹出方式只能为 $LRLRLRLLLL$ 或 $RLRLRLRRRR$ 。求出前缀后缀和，枚举分界线计算即可。

### 代码

```c++
#include <cstdio>
#include <climits>
#include <algorithm>

const int MAXN = 100005;

int w[MAXN];
long long pre[MAXN], suf[MAXN], ex[MAXN];

int main() {
    int n, l, r, ql, qr;
    while (scanf("%d %d %d %d %d", &n, &l, &r, &ql, &qr) == 5) {
        for (int i = 1; i <= n; i++) scanf("%d", &w[i]);
        pre[0] = 0;
        for (int i = 1; i <= n; i++) pre[i] = pre[i - 1] + w[i];
        suf[n + 1] = 0;
        for (int i = n; i; i--) suf[i] = suf[i + 1] + w[i];
        
        for (int i = 0; i <= n; i++) {
            if (n - 2 * i - 1 > 0) ex[i] = (n - 2 * i - 1ll) * qr;
            else if (n - 2 * (n - i) - 1 > 0) ex[i] = (n - 2 * (n - i) - 1ll) * ql;
            else ex[i] = 0;
        }

        long long ans = LLONG_MAX;
        for (int i = 0; i <= n; i++)
            ans = std::min(ans, ex[i] + l * pre[i] + r * suf[i + 1]);

        printf("%lld\n", ans);
    }
    
    return 0;
}
```

## Contest 3 - G - #include &lt;bits/stdc++.h><span id = "03g"></span>

### 题目大意

做一个支持区间合并的并查集，有 $n$ 个元素， $q$ 次操作，操作有四种：

* 合并元素 $x$ 与元素 $y$
* 合并 $[x, y]$ 内的元素
* 询问 $x$ 与 $y$ 是否合并
* 询问 $x$ 所在的块有多少个元素

$1 \leq n, q \leq 100,000$

$1 \leq T \leq 5$

时间限制：$2 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

用一个线段树维护并查集的 `fa` 数组。合并两个元素时修改单点；合并区间时，递归到区间内的元素的 `fa` 值相等时才进行修改。

由于一段区间之可能从不等变为相等，不可能从相等变为不等，所以区间修改时并不需要懒标记。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 100005;

struct UFS {
    int f[MAXN], size[MAXN];

    void init(int n) {
        for (int i = 1; i <= n; i++) f[i] = i;
        for (int i = 1; i <= n; i++) size[i] = 1;
    }

    int find(int x) {
        if (x == f[x]) return x;
        size[f[x]] += size[x];
        return f[x] = find(f[x]);
    }

    void merge(int x, int y) {
        if (!test(x, y)) {
            size[find(x)] += size[find(y)];
            size[find(y)] = 0;
            f[find(y)] = find(x);
        }
    }

    bool test(int x, int y) {
        return find(x) == find(y);
    }
} ufs;

struct SegT {
    struct Node {
        Node *lc, *rc;
        int val;

        Node(int val = -1) : val(val), lc(lc), rc(rc) {}
        Node(Node *lc, Node *rc) : lc(lc), rc(rc), val(lc->val == rc->val ? lc->val : -1) {}

        void maintain() {
            val = (lc->val == rc->val ? lc->val : -1);
        }

        void update(int l, int r, int L, int R, int x) {
            if (R < l || r < L) return;
            if (L <= l && r <= R && val != -1) {
                ufs.merge(x, val);
                val = ufs.find(x);
                return;
            }
            int mid = l + ((r - l) >> 1);
            lc->update(l, mid, L, R, x);
            rc->update(mid + 1, r, L, R, x);
            maintain();
        }
    } *root, _pool[MAXN << 1], *_curr;
    int n;

    Node *build(int l, int r) {
        if (l == r) return new (_curr++) Node(l);
        int mid = l + ((r - l) >> 1);
        return new (_curr++) Node(build(l, mid), build(mid + 1, r));
    }

    void init(int n) {
        _curr = _pool;
        this->n = n;
        root = build(1, n);
    }

    void update(int l, int r, int x) {
        root->update(1, n, l, r, x);
    }
} segT;

void clear(int n) {
    ufs.init(n);
    segT.init(n);
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n, q;
        scanf("%d %d", &n, &q);

        clear(n);

        while (q--) {
            int op, x, y;
            scanf("%d %d", &op, &x);

            if (op == 1) {
                scanf("%d", &y);
                segT.update(x, x, ufs.find(y));
            } else if (op == 2) {
                scanf("%d", &y);
                if (x > y) std::swap(x, y);
                segT.update(x, y, ufs.find(x));
            } else if (op == 3) {
                scanf("%d", &y);
                puts(ufs.test(x, y) ? "YES" : "NO");
            } else {
                printf("%d\n", ufs.size[ufs.find(x)]);
            }
        }
    }
    
    return 0;
}
```