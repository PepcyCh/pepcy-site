---
title: ZJU 七月集训题目选讲（Contest 4 ～ 6）
date: 2018-07-29 15:59:27
tags: [ZJU, 拓扑排序, Splay, DP, 树上倍增, 裴蜀定理, 可并堆, 博弈, 生成函数, FFT, Trie, 离线]
categories: 题解（OI/XCPC）
---

ZJU 七月集训（2018.7.13 ～ 2018.7.26）的题目。

* Contest 4 by ChugJug - 2018.7.16
* Contest 5 by DeepDark - 2018.7.17
* Contest 6 by SBconscious - 2018.7.19

所有题目都是多组数据，若无子测试点数的说明，则 $T$ 不会特别大。

## 目录

[Contest 4 - B - Positive Eigenvalues](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-4-%EF%BD%9E-6%EF%BC%89/#04b)

[Contest 4 - C - String Set](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-4-%EF%BD%9E-6%EF%BC%89/#04c)

[Contest 4 - E - Candies](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-4-%EF%BD%9E-6%EF%BC%89/#04e)

[Contest 5 - B - MUJI](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-4-%EF%BD%9E-6%EF%BC%89/#05b)

[Contest 5 - C - Magical Tree](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-4-%EF%BD%9E-6%EF%BC%89/#05c)

[Contest 5 - F - Simple Counting](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-4-%EF%BD%9E-6%EF%BC%89/#05f)

[Contest 6 - A - SmartLy's Game](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-4-%EF%BD%9E-6%EF%BC%89/#06a)

[Contest 6 - E - Heltion and Sequence](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-4-%EF%BD%9E-6%EF%BC%89/#06e)

[Contest 6 - G - Heltion and Backpack](http://pepcy.cf/ZJU-%E4%B8%83%E6%9C%88%E9%9B%86%E8%AE%AD%E9%A2%98%E7%9B%AE%E9%80%89%E8%AE%B2%EF%BC%88Contest-4-%EF%BD%9E-6%EF%BC%89/#06g)

<!--more-->

## Contest 4 - B - Positive Eigenvalues<span id = "04b"></span>

### 题目大意

给出一个 $n \times n$ 的 $01$ 矩阵，判断其是否为正定矩阵。

$1 \leq n \leq 50$

$1 \leq T \leq 1,000$，$n \geq 10$ 的子测试点不超过 $10$ 个。

时间限制：$1 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

$$
\begin{align}
&\because \lambda_i > 0 ~ (1 \leq i \leq n) \\
&\therefore \det(\textbf{A}) = \prod_{i = 1}^{n} \lambda_i > 0 \\
&\because \prod_{i = 1}^{n} \lambda_i \leq (\frac{\sum_{i = 1}^{n} \lambda_i}{n})^n = (\frac{\sum_{i = 1}^{n} \textbf{A}_{i, i}}{n})^n \leq 1 ~ (\textbf{A}_{i, i} \text { is either } 1 \text{ or } 0) \\
&\therefore \det(\textbf{A}) = 1 \\
&\therefore \lambda_i = 1 ~ (1 \leq i \leq n), \textbf{A}_{i, i} = 1 ~ (1 \leq i \leq n)
\end{align}
$$

根据行列式计算的定义式，我们知道，矩阵 $\textbf{A}$ 的除主对角线外其他 $1 \sim n$ 的置换上的位置的值不全为 $1$。即，以该矩阵为某一有向图的邻接矩阵，该有向图无环（自环除外）。判断主对角线上是否全为 $1$ ，然后建图跑拓扑排序判断是否存在环即可。

### 代码

```c++
#include <cstdio>
#include <stack>
#include <algorithm>

const int MAXN = 55;

struct Node;
struct Edge;

struct Node {
    Edge *e;
    int deg;
} N[MAXN];

struct Edge {
    Node *u, *v;
    Edge *next;

    Edge() {}
    Edge(Node *u, Node *v) : u(u), v(v), next(u->e) {}
} _pool[MAXN * MAXN], *_curr;

void addEdge(int u, int v) {
    N[u].e = new (_curr++) Edge(&N[u], &N[v]);
    ++N[v].deg;
}

bool topo(int n) {
    static std::stack<Node *> s;

    for (int i = 1; i <= n; i++) if (!N[i].deg) s.push(&N[i]);
    while (!s.empty()) {
        Node *u = s.top();
        s.pop();

        for (Edge *e = u->e; e; e = e->next) if (!(--e->v->deg)) s.push(e->v);
    }

    for (int i = 1; i <= n; i++) if (N[i].deg) return false;
    return true;
}

void clear(int n) {
    _curr = _pool;
    for (int i = 1; i <= n; i++) {
        N[i].e = NULL;
        N[i].deg = 0;
    }
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);
        clear(n);

        bool flag = true;
        for (int i = 1, x; i <= n; i++) for (int j = 1; j <= n; j++) {
            scanf("%d", &x);
            if (i != j && x) addEdge(i, j);
            if (i == j && !x) flag = false;
        }

        if (flag) flag = topo(n);

        puts(flag ? "YES" : "NO");
    }
    
    return 0;
}
```

## Contest 4 - C - String Set<span id = "04c"></span>

### 题目大意

给出一个字符串的可重集合 $A$（初始为空），定义一个字符串的最长匹配前缀长度（MMPL）为：
$$
\text{MMPL}(S) = \max(|T|, T \in A \text{ and } T \text{ is a prefix of } S)
$$
有 $n$ 次操作，每次操作向 $A$ 中放入一个字符串 $s$，或删除一个已有的字符串。有 $m$ 次询问。每次询问一个字符串 $S$ 在操作区间 $[l, r]$ 内 $\text{MMPL}(S)$ 变化了几次（操作从头开始，但只在询问区间计数）。

所有字符串为 $01$ 串。

$1 \leq n, m \leq 10^5$

$1 \leq |s| \leq 32, |S| = 32$

$1 \leq T \leq 10$，大测试点的数目不会超过 $4$。

时间限制：$2 \text{s}$，内存限制：$1024 \text{MB}$。

### 题解

离线所有的操作与查询（拆为两个）并按时间排序。建立一个 Trie。

插入时直接插入，给其单词计数器值加 $1$。如果是第一次插入该字符串，打一个 $-1$ 的 tag，给其贡献计数器值加 $1$。

删除入沿 Trie 走到末尾节点，给其单词计数器值减 $1$。如果此时计数器为 $0$，打一个 $-1$ 的tag，给其贡献计数器加 $1$。

查询时沿 Trie 走，累加贡献计数器的值即可。

传递标记时：如果其无子节点，不传递；如果子节点的单词计数器为 $0$，将 tag 的值加给子节点的 tag；如果子节点的单词计数器不为 $0$，将 tag 的值加给子节点的贡献计数器。

### 代码

```c++
#include <cstdio>
#include <cstring>
#include <algorithm>

const int N = 33;
const int MAXN = 100005;

template <typename T, size_t SIZE>
struct Pool {
    char mem[SIZE * sizeof (T)], *top;

    void init() {
        top = mem;
    }

    void *alloc() {
        char *res = top;
        top += sizeof (T);
        return (void *) res;
    }
};

struct Trie {
    static const int MAXN = ::MAXN << 1;

    struct Node {
        Node *c[2];
        int val, tag, cnt;
        static Pool<Node, MAXN> pool;

        Node() : val(0), tag(0), cnt(0) {
            c[0] = c[1] = NULL;
        }

        void *operator new(size_t) {
            return pool.alloc();
        }

        void pushDown() {
            if (tag) {
                if (c[0]) {
                    if (c[0]->cnt) c[0]->val += tag;
                    else c[0]->tag += tag;
                }
                if (c[1]) {
                    if (c[1]->cnt) c[1]->val += tag;
                    else c[1]->tag += tag;
                }
                tag = 0;
            }
        }

        void insert(char *l, char *r) {
            pushDown();
            if (l == r) {
                if (!cnt) {
                    ++val;
                    tag = -1;
                }
                ++cnt;
                return;
            }
            int L = *(l++) - '0';
            if (!c[L]) c[L] = new Node();
            c[L]->insert(l, r);
        }

        void erase(char *l, char *r) {
            pushDown();
            if (l == r) {
                --cnt;
                if (!cnt) {
                    ++val;
                    tag = -1;
                }
                return;
            }
            int L = *(l++) - '0';
            if (!c[L]) return;
            c[L]->erase(l, r);
        }

        void check(char *l, char *r, int &ans) {
            pushDown();
            ans += val;
            if (l == r) return;
            int L = *(l++) - '0';
            if (!c[L]) return;
            c[L]->check(l, r, ans);
        }
    } *root;

    void init() {
        root = new Node();
    }

    void insert(char *l, char *r) {
        root->insert(l, r);
    }

    void erase(char *l, char *r) {
        root->erase(l, r);
    }

    int check(char *l, char *r) {
        int res = 0;
        root->check(l, r, res);
        return res;
    }
} trie;
Pool<Trie::Node, Trie::MAXN> Trie::Node::pool;

struct Operation {
    int op, pos, len, ans, id;
    char s[N];

    Operation() {}
    Operation(int id, int op, int pos, char s[N]) : id(id), op(op), pos(pos), ans(0) {
        len = strlen(s);
        std::copy(s, s + len, this->s);
    }
} O[MAXN * 3];

bool compareByPos(const Operation &a, const Operation &b) {
    return a.pos == b.pos ? a.op < b.op : a.pos < b.pos;
}
bool compareById(const Operation &a, const Operation &b) {
    return a.id == b.id ? a.pos < b.pos : a.id < b.id;
}

void init() {
    Trie::Node::pool.init();
    trie.init();
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        init();

        int n, q;
        scanf("%d %d", &n, &q);
        for (int i = 0; i < n; i++) {
            static char op[2];
            scanf("%s %s", op, O[i].s);
            O[i].op = (op[0] == '+' ? 0 : 1);
            O[i].pos = i + 1;
            O[i].id = q;
            O[i].len = strlen(O[i].s);
        }
        for (int i = 0, l, r; i < q; i++) {
            static char s[N];
            scanf("%s %d %d", s, &l, &r);
            O[i + n] = Operation(i, 2, r, s);
            O[i + n + q] = Operation(i, 2, l - 1, s);
        }

        int N = n + q * 2;
        std::sort(O, O + N, compareByPos);

        for (int i = 0; i < N; i++) {
            if (O[i].op == 0) {
                trie.insert(O[i].s, O[i].s + O[i].len);
            } else if (O[i].op == 1) {
                trie.erase(O[i].s, O[i].s + O[i].len);
            } else {
                O[i].ans = trie.check(O[i].s, O[i].s + O[i].len);
            }
        }

        std::sort(O, O + N, compareById);
        for (int i = 0; i < 2 * q; i += 2) {
            int ans = O[i + 1].ans - O[i].ans;
            printf("%d\n", ans);
        }
    }
    
    return 0;
}
```

## Contest 4 - E - Candies<span id = "04e"></span>

### 题目大意

Alice 和 Bob 分 $n$ 支蜡烛，每只蜡烛有一个价值 $w_i$。两人分得的蜡烛数目应尽量相等，在此基础上，两人分得的蜡烛的价值和应尽量相等。若无法让二人的蜡烛价值和相等，Alice 分得的蜡烛价值和应小于 Bob 的。求二人分得的蜡烛价值和。

$1 \leq n \leq 100$

$\sum_{i = 1}^{n} w_i \leq 100,000$

$1 \leq T \leq 100$

时间限制：$2 \text{s}$，内存限制：$64 \text{MB}$。

### 题解

$f(i, j)$ 表示数 $i$ 是否可有 $j$ 个数拼成，第二维可用一个 `long long` 或 `std::bitset` 储存，则转移为： `f[i] |= (f[i - w[i]] << 1)`。

计算答案时从 $\sum w_i / 2$ 开始向下枚举，判断是否能由一半的 $n$ 的个数（$n$ 为奇数时要判断两个）的数拼成即可。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 105;
const int MAX_SUM = 100005;

long long f[MAX_SUM];
int w[MAXN];

int main() {
    int T;
    scanf("%d", &T);
    for (int kase = 1; kase <= T; kase++) {
        int n, sum = 0;
        scanf("%d", &n);

        std::fill(f, f + MAX_SUM, 0);
        f[0] = 1;
        for (int i = 0, x; i < n; i++) {
            scanf("%d", &w[i]);
            sum += w[i];
        }

        for (int i = 0; i < n; i++) for (int j = sum >> 1; j >= w[i]; j--)
            f[j] |= (f[j - w[i]] << 1);

        long long a = (1ll << ((n + 1) >> 1ll)), b = (1ll << (n >> 1ll));
        for (int i = sum >> 1; i; i--) if ((f[i] & a) || (f[i] & b)) {
            printf("Case %d: %d %d\n", kase, i, sum - i);
            break;
        }
    }
    
    return 0;
}
```

## Contest 5 - B - MUJI<span id = "05b"></span>

### 题目大意

给定一棵 $n$ 个点的树，每个点上有一个权值 $v_i$。有 $q$ 次询问，每次询问两个点，从一个点开始，沿它们间的简单路径走到另一个点，每到一个点（含端点），就可以选择获得或失去 $kv_i$ 个物品（$k \in \mathbb{N}$，自行选择），求最终所持有物品的正最小值。

$1 \leq n, q \leq 200,000$

$1 \leq T \leq 10$，大测试点不超过 $3$ 个。

时间限制：$2 \text{s}$，内存限制：$128 \text{MB}$。

### 题解

由裴蜀定理，答案即路径上顶点权值的最大公约数，树上倍增即可。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 200005;
const int MAXN_LOG = 18;

struct Node;
struct Edge;

struct Node {
    Edge *e;
    Node *f[MAXN_LOG];
    int dep, w, gcd[MAXN_LOG];
} N[MAXN];

struct Edge {
    Node *u, *v;
    Edge *next;

    Edge() {}
    Edge(Node *u, Node *v) : u(u), v(v), next(u->e)  {}
} _pool[MAXN << 1], *_curr;

void addEdge(int u, int v) {
    N[u].e = new (_curr++) Edge(&N[u], &N[v]);
    N[v].e = new (_curr++) Edge(&N[v], &N[u]);
}

void dfs(Node *u, Node *fa = NULL) {
    u->dep = (fa ? fa->dep : 0) + 1;
    u->f[0] = (fa ? fa : u);
    u->gcd[0] = (fa ? std::__gcd(u->w, fa->w) : u->w);

    for (int i = 1; i < MAXN_LOG; i++) {
        u->f[i] = u->f[i - 1]->f[i - 1];
        u->gcd[i] = std::__gcd(u->gcd[i - 1], u->f[i - 1]->gcd[i - 1]);
    }

    for (Edge *e = u->e; e; e = e->next) if (e->v != fa) dfs(e->v, u);
}

int lca(Node *u, Node *v) {
    int res = std::__gcd(u->w, v->w);
    if (u->dep < v->dep) std::swap(u, v);

    for (int i = MAXN_LOG - 1; ~i; i--) if (u->f[i]->dep >= v->dep) {
        res = std::__gcd(res, u->gcd[i]);
        u = u->f[i];
    }

    for (int i = MAXN_LOG - 1; ~i; i--) if (u->f[i] != v->f[i]) {
        res = std::__gcd(res, std::__gcd(u->gcd[i], v->gcd[i]));
        u = u->f[i];
        v = v->f[i];
    }

    if (u != v) res = std::__gcd(res, u->gcd[0]);
    return res;
}

void clear(int n) {
    _curr = _pool;
    for (int i = 1; i <= n; i++) N[i].e = NULL;
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);
        clear(n);

        for (int i = 1; i <= n; i++) scanf("%d", &N[i].w);

        for (int i = 1, u, v; i < n; i++) {
            scanf("%d %d", &u, &v);
            addEdge(u, v);
        }

        dfs(&N[1]);

        int q;
        scanf("%d", &q);
        while (q--) {
            int x, y;
            scanf("%d %d", &x, &y);
            int ans = lca(&N[x], &N[y]);
            printf("%d\n", ans);
        }
    }
    
    return 0;
}
```

## Contest 5 - C - Magical Tree<span id = "05c"></span>

### 题目大意

给定一棵 $n$ 个点的树，每个点是黑色或白色，每条边有一个权值 $c_i$。要求删去权值和最小的边集使得每一个连痛快内最多有一个黑点。同时有 $m$ 次操作，每次操作会把一个点变为黑点（该点原来可能为黑点）。求操作前以及每次操作后，需要删去的边集的最小权值和。

$1 \leq n, m \leq 100,000$

$1 \leq c_i \leq 1,000$

$1 \leq T \leq 10$

时间限制：$2 \text{s}$，空间限制：$64 \text{MB}$。

### 题解

考虑把操作倒过来，即把黑点变为白点，则每次需要恢复一条与该点所在连通块相连的、权值最小的边。一开始让所有的点为黑点，即所有的边都删去，为每一个连通块维护一个可并堆（小根堆），在恢复边时，选择堆顶的边，并弹出、标记这条边，将被合并的两个连通块的可并堆合并。连通块可用并查集维护。注意处理改变前后颜色不变的情况。

### 代码

```c++
#include <cstdio>
#include <ext/pb_ds/priority_queue.hpp>
#include <algorithm>

const int MAXN = 100005;

struct UFS {
    int f[MAXN];

    void init(int n) {
        for (int i = 1; i <= n; i++) f[i] = i;
    }

    int find(int x) {
        return x == f[x] ? x : f[x] = find(f[x]);
    }

    void merge(int x, int y) {
        f[find(y)] = find(x);
    }
} ufs;

struct Node {
    int c;
    __gnu_pbds::priority_queue<std::pair<int, int> > q;
} N[MAXN];

struct Edge {
    int u, v, w;
    bool used;
} E[MAXN];

struct Operation {
    int x, ans;
    bool exist;
} O[MAXN];

void clear(int n) {
    for (int i = 1; i <= n; i++)
        N[i].q.clear();

    ufs.init(n);
}

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n;
        scanf("%d", &n);
        clear(n);

        for (int i = 1; i <= n; i++) scanf("%d", &N[i].c);

        int sum = 0;
        for (int i = 1; i < n; i++) {
            scanf("%d %d %d", &E[i].u, &E[i].v, &E[i].w);
            sum += E[i].w;
            N[E[i].u].q.push(std::make_pair(E[i].w, i));
            N[E[i].v].q.push(std::make_pair(E[i].w, i));
            E[i].used = false;
        }

        int q;
        scanf("%d", &q);
        for (int i = 1; i <= q; i++) {
            scanf("%d", &O[i].x);
            if (N[O[i].x].c == 1) {
                O[i].exist = true;
            } else {
                O[i].exist = false;
                N[O[i].x].c = 1;
            }
        }

        for (int i = 1; i <= n; i++) if (!N[i].c) {
            int x = ufs.find(i);

            while (N[x].q.size() && E[N[x].q.top().second].used) N[x].q.pop();
            if (N[x].q.empty()) continue;
            auto e = N[x].q.top();
            sum -= e.first;
            N[x].q.pop();
            E[e.second].used = true;

            int y = (ufs.find(E[e.second].u) == x ? ufs.find(E[e.second].v) : ufs.find(E[e.second].u));
            ufs.merge(x, y);
            N[x].q.join(N[y].q);
        }

        for (int i = q; i; i--) if (!O[i].exist) {
            O[i].ans = sum;

            int x = ufs.find(O[i].x);

            while (N[x].q.size() && E[N[x].q.top().second].used) N[x].q.pop();
            if (N[x].q.empty()) continue;
            auto e = N[x].q.top();
            sum -= e.first;
            N[x].q.pop();
            E[e.second].used = true;

            int y = (ufs.find(E[e.second].u) == x ? ufs.find(E[e.second].v) : ufs.find(E[e.second].u));
            ufs.merge(x, y);
            N[x].q.join(N[y].q);
        }

        printf("%d\n", O[0].ans = sum);
        for (int i = 1; i <= q; i++) {
            // printf("i = %d, i.before = %d\n", i, O[i].before);
            printf("%d\n", O[i].exist ? (O[i].ans = O[i - 1].ans) : O[i].ans);
        }
    }
    
    return 0;
}
```

## Contest 5 - F - Simple Counting<span id = "05f"></span>

### 题目大意

求有多少个 $X$ 满足：
$$
\lfloor \sqrt[K]{X} \rfloor \;|\; X ~ (X \in [1, N], X \in \mathbb{Z})
$$
其中 $a | b$ 表示 $b$ 是 $a$ 的倍数。

$1 \leq N \leq 2^{31} - 1$

$2 \leq K \leq 4$

$1 \leq T \leq 1,000,000$

时间限制：$2 \text{s}$，空间限制：$64 \text{MB}$。

### 题解

$[x^K, (x + 1)^K)$ 内的数的 $K$ 次根号的值均为 $x$，该区间内合法的数的个数为：
$$
\frac{(x + 1)^K - x^K - 1}{x} + 1
$$
按 $K$ 分类手推式子，对于最后一个不完整的区间单独考虑即可。

```c++
#include <cstdio>
#include <cmath>

int main() {
    int n, k;
    while (scanf("%d %d", &n, &k) == 2) {
        if (k == 2) {
            long long t = pow(n, 0.5);
            if ((t + 1) * (t + 1) <= n) ++t;
            long long ans = 2 * (t - 1) + n / t;
            printf("%d\n", ans);
        } else if (k == 3) {
            long long t = pow(n, 1.0 / 3.0);
            if ((t + 1) * (t + 1) * (t + 1) <= n) ++t;
            long long ans = t * (5 + t) / 2 - 3 + n / t;;
            printf("%d\n", ans);
        } else {
            long long t = pow(n, 0.25);
            if ((t + 1) * (t + 1) * (t + 1) * (t + 1) <= n) ++t;
            long long ans = t * (t * t + 8) / 3 + t * t - 4 + n / t;
            printf("%d\n", ans);
        }
    }
    
    return 0;
}
```

## Contest 6 - A - SmartLy's Game<span id = "06a"></span>

### 题目大意

两个人玩游戏：有 $n$ 堆石子，每堆有一些石子。先手可以在石子数不少于 $A$ 的石堆中取走 $A$ 个石子，后手可以在石子数不少于 $B$ 的石堆中取走 $B$ 个石子。一方不能行动则另一方获胜，问先手是否必胜。

$1 \leq N \leq 100$

$1 \leq A, B \leq 10^9$

$1 \leq T \leq 10,000$

时间限制：$1 \text{s}$，空间限制：$64 \text{MB}$。

### 题解

每堆石子数可以对 $A + B$ 取模而对答案无影响。取模后按石子数分类：

* $a_i < \min(A, B)$：双方都不能取，可以无视；
* $\min(A, B) \leq a_i < \max(A, B)$：只有小的一方可以取，如果存在这样的堆，则小的一方必胜；
* $a_i \geq \max(A, B) \text{ and } a_i \geq 2\min(A, B)$：大的一方只能取一次，而小的一方可以取多次。当小的一方取过一次后转化为情况二，而大的一方取过一次后转化为情况一。如果存在这样的堆且先手为小者，先手必胜；存在至少 $2$ 个这样的堆且后手为小者，后手必胜；
* $a_i \geq \max(A, B)$：当不能根据之前的情况判定胜负时，每一个这种堆只能被双方取一次（只存在一个情况三的堆且后手为小者时，先手会先抢走这堆），故根据这种堆数目的奇偶判断胜负。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MAXN = 105;

int a[MAXN];

int main() {
    int T;
    scanf("%d", &T);
    while (T--) {
        int n, A, B;
        scanf("%d %d %d", &n, &B, &A);
        int min = std::min(A, B), max = std::max(A, B);

        int t = A + B;
        for (int i = 0; i < n; i++) {
            scanf("%d", &a[i]);
            a[i] %= t;
        }

        bool flag = false;
        for (int i = 0; i < n; i++) if (min <= a[i] && a[i] < max) {
            puts(A < B ? "GIRL" : "JSB");
            flag = true;
            break;
        }

        if (flag) continue;

        int cnt2Min = 0;
        for (int i = 0; i < n; i++) if (max <= a[i] && 2 * min <= a[i]) ++cnt2Min;
        if (A < B && cnt2Min) {
            puts("GIRL");
            continue;
        }
        if (B < A && cnt2Min > 1) {
            puts("JSB");
            continue;
        }

        int cnt = 0;
        for (int i = 0; i < n; i++) if (max <= a[i]) ++cnt;
        puts(cnt % 2 ? "GIRL" : "JSB");
    }

    return 0;
}
```

## Contest 6 - E - Heltion and Sequence<span id = "06e"></span>

### 题目大意

给定一个初始有 $n$ 个元素的数组 $\{a_i\}$，之后有 $q$ 个操作。一共有 $4$ 种操作：

* `1 u v`：在位置 $u$ 前插入数 $v$；
* `2 l r`：询问区间 $[l, r]$ 的元素和；
* `3 u v`：将位置 $u$ 的数改为 $v$；
* `4`：按升序排序数组。

$1 \leq n, q \leq 200,000$

$0 \leq v, a_i \leq 32767$

$\sum n \leq 233,333, ~ \sum q \leq 466,666$

时间限制：$10 \text{s}$，空间限制：$1024 \text{MB}$。

### 题解

除操作四外均为 Splay 的基本操作。我们每修改或插入一个数后，将对应节点放入一个修改数组，排序时将修改数组内的点全部删除，然后重新插入到 Splay 里即可。

### 代码

```c++
#include <cstdio>
#include <cassert>

const int MAXN = 400005;

template <typename T, size_t SIZE>
struct Pool {
    char mem[SIZE * sizeof (T)], *top, *del[SIZE], **delTop;

    void init() {
        top = mem;
        delTop = del;
    }

    void *alloc() {
        if (delTop != del) return (void *) *(--delTop);
        char *res = top;
        top += sizeof (T);
        return (void *) res;
    }

    void free(void *p) {
        *(delTop++) = (char *) p;
    }
};

struct Splay {
    struct Node {
        Node *c[2], *fa, **root;
        int val, size;
        long long sum;
        bool isBound;
        static Pool<Node, MAXN> pool;

        Node () {}
        Node(Node **root, Node *fa, int val, bool isBound = false)
            : root(root), fa(fa), size(1), val(val), sum(isBound ? 0 : val), isBound(isBound) {
            c[0] = c[1] = nullptr;
        }

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

        void maintain() {
            size = (c[0] ? c[0]->size : 0) + 1 + (c[1] ? c[1]->size : 0);
            sum = (c[0] ? c[0]->sum : 0) + (isBound ? 0 : val) + (c[1] ? c[1]->sum : 0);
        }

        int rel() {
            return fa->c[1] == this;
        }

        void rotate() {
            Node *o = fa;
            int x = rel();

            fa = o->fa;
            if (fa) fa->c[o->rel()] = this;

            o->c[x] = c[x ^ 1];
            if (c[x ^ 1]) c[x ^ 1]->fa = o;

            c[x ^ 1] = o;
            o->fa = this;

            o->maintain();
            maintain();

            if (!fa) *root = this;
        }

        Node *splay(Node *tarFa = nullptr) {
            while (fa != tarFa) {
                if (fa->fa == tarFa) rotate();
                else if (rel() == fa->rel()) fa->rotate(), rotate();
                else rotate(), rotate();
            }
            return this;
        }

        Node *pred() {
            Node *u = c[0];
            while (u->c[1]) u = u->c[1];
            return u;
        }

        Node *succ() {
            Node *u = c[1];
            while (u->c[0]) u = u->c[0];
            return u;
        }

        int lSize() const {
            return (c[0] ? c[0]->size : 0) + 1;
        }
    } *root, *modified[MAXN];
    int modifiedCnt;

    void init() {
        root = new Node(&root, nullptr, -1, true);
        root->c[1] = new Node(&root, root, 32768, true);
        root->size = 2;
        modifiedCnt = 0;
    }

    Node *select(int pos) {
        Node *u = root;
        while (pos != u->lSize()) {
            if (pos < u->lSize()) {
                u = u->c[0];
            } else {
                pos -= u->lSize();
                u = u->c[1];
            }
        }
        return u;
    }

    Node *insert(int pos, int val) {
        Node *pred = select(pos), *succ = select(pos + 1);
        pred->splay();
        succ->splay(pred);

        succ->c[0] = new Node(&root, succ, val);
        modified[modifiedCnt++] = succ->c[0];

        succ->maintain();
        pred->maintain();
    }

    Node *insert(int val) {
        Node **u = &root, *fa = nullptr;
        while ((*u) && (*u)->val != val) {
            fa = *u;
            ++fa->size;
            fa->sum += val;
            u = &(*u)->c[val > (*u)->val];
        }

        if (*u) {
            Node *v = (*u)->splay();
            Node *succ = v->succ();
            succ->splay(v);
            succ->c[0] = new Node(&root, succ, val);

            succ->maintain();
            v->maintain();

            u = &succ->c[0];
        } else {
            (*u) = new Node(&root, fa, val);
        }

        return (*u)->splay();
    }

    void modify(int pos, int val) {
        Node *u = select(pos + 1);
        u->splay();

        u->val = val;
        u->maintain();
        modified[modifiedCnt++] = u;
    }

    void erase(Node *u) {
        u->splay();
        Node *pred = u->pred(), *succ = u->succ();
        pred->splay();
        succ->splay(pred);

        u->size = -1;
        delete succ->c[0];
        succ->c[0] = nullptr;
        
        succ->maintain();
        pred->maintain();
    }

    long long sum(int l, int r) {
        Node *pred = select(l), *succ = select(r + 2);
        pred->splay();
        succ->splay(pred);

        long long res = succ->c[0] ? succ->c[0]->sum : 0;
        return res;
    }

    void sort() {
        static int val[MAXN];
        int p = 0;
        for (int i = 0; i < modifiedCnt; i++) {
            Node *u = modified[i];
            if (u->size == -1) continue;
            val[p++] = u->val;
            erase(u);
        }
        for (int i = 0; i < p; i++) insert(val[i]);
        modifiedCnt = 0;
    }
} splay;
Pool<Splay::Node, MAXN> Splay::Node::pool;

void init() {
    Splay::Node::pool.init();
    splay.init();
}

int main() {
    int n;
    while (scanf("%d", &n) == 1) {
        init();

        for (int i = 1, x; i <= n; i++) {
            scanf("%d", &x);
            splay.insert(i, x);
        }

        int q;
        scanf("%d", &q);
        while (q--) {
            int op;
            scanf("%d", &op);

            if (op == 1) {
                int u, v;
                scanf("%d %d", &u, &v);
                splay.insert(u, v);
            } else if (op == 2) {
                int l, r;
                scanf("%d %d", &l, &r);
                long long ans = splay.sum(l, r);
                printf("%lld\n", ans);
            } else if (op == 3) {
                int u, v;
                scanf("%d %d", &u, &v);
                splay.modify(u, v);
            } else {
                splay.sort();
            }
        }
    }
    
    return 0;
}
```

## Contest 6 - G - Heltion and Backpack<span id = "06g"></span>

### 题目大意

有 $n$ 种物品，每种物品的价值是 $v_i$，每种物品有无数个，求取出正好 $k$ 个物品能得到多少种价值和。

$1 \leq n, k, v_i \leq 1,000$

$1 \leq T \leq 10, ~ \sum \max(n, k, v_i) \leq 3,333$

时间限制：$3 \text{s}$，空间限制：$512 \text{MB}$。

### 题解

对于每个价值为 $v_i$ 的物品，令多项式 $f(x)$ 中 $x^{v_i}$ 的系数为 $1$，则答案为 $f(x)^k$ 的非零系数个数。

### 代码

```c++
#include <cstdio>
#include <algorithm>

const int MOD = 998244353;
const int G = 3;

long long qpow(long long a, long long n) {
    long long res = 1;
    for (; n; n >>= 1, a = a * a % MOD) if (n & 1) res = res * a % MOD;
    return res;
}

long long inv(long long a) {
    return qpow(a, MOD - 2);
}

namespace FFT {
    const int N = 1048576;

    long long omega[2][N];

    void init() {
        long long g = qpow(G, (MOD - 1) / N), ig = inv(g);
        omega[0][0] = omega[1][0] = 1;
        for (int i = 1; i < N; i++) {
            omega[0][i] = omega[0][i - 1] * g % MOD;
            omega[1][i] = omega[1][i - 1] * ig % MOD;
        }
    }

    int extend(int n) {
        int res = 1;
        while (res < n) res <<= 1;
        return res;
    }

    void reverse(long long *a, int n) {
        for (int i = 0, j = 0; i < n; i++) {
            if (i < j) std::swap(a[i], a[j]);
            for (int l = n >> 1; (j ^= l) < l; l >>= 1) {}
        }
    }

    void transform(long long *a, int n, long long *omega) {
        reverse(a, n);

        for (int l = 2; l <= n; l <<= 1) {
            int hl = l >> 1, k = N / l;
            for (long long *x = a; x != a + n; x += l) {
                for (int i = 0; i < hl; i++) {
                    long long t = omega[k * i] * x[i + hl] % MOD;
                    x[i + hl] = (x[i] - t + MOD) % MOD;
                    x[i] += t;
                    x[i] >= MOD ? x[i] -= MOD : 0;
                }
            }
        }
    }

    void dft(long long *a, int n) {
        transform(a, n, omega[0]);
    }

    void idft(long long *a, int n) {
        transform(a, n, omega[1]);
        long long ni = inv(n);
        for (int i = 0; i < n; i++) a[i] = a[i] * ni % MOD;
    }
}

long long A[FFT::N];

void clear() {
    std::fill(A, A + FFT::N, 0ll);
}

int main() {
    FFT::init();

    int n, k;
    while (scanf("%d %d", &n, &k) == 2) {
        clear();

        int max = 0, min = 1000;
        for (int i = 0, x; i < n; i++) {
            scanf("%d", &x);
            A[x] = 1;
            max = std::max(x, max);
            min = std::min(x, min);
        }
        int sum = max * k + 1;
        int sum_min = min * k;
        int N = FFT::extend(sum);

        FFT::dft(A, N);
        for (int j = 0; j < N; j++) A[j] = qpow(A[j], k);
        FFT::idft(A, N);

        int ans = 0;
        for (int i = sum_min; i < sum; i++) if (A[i]) ++ans;
        printf("%d\n", ans);
    }
    
    return 0;
}
```