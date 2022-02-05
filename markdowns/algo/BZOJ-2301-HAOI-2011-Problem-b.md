---
title: '[HAOI 2011] Problem b'
date: 2017-04-04 20:50:33
tags: [数论, 线性筛, 莫比乌斯反演]
categories: 题解（OI/XCPC）
---

## 题目大意

对于给出的 $n$ 个询问，每次求有多少个数对 $(x, y)$，满足 $a \leqslant x \leqslant b$，$c \leqslant y \leqslant d$，且$gcd(x, y) = k$。

$n, \ a, \ b, \ c, \ d, \ k \leqslant 50,000$

## 题目链接

[【HAOI 2011】problem b - Luogu 2522](https://www.luogu.com.cn/problem/P2522)

## 题解

莫比乌斯反演裸题（本来还有一个更裸，但被权限了。。。）。

首先，通过容斥，我们可以把问题化为求满足 $x \leqslant n$，$y \leqslant m$，且 $gcd(x, y) = k$ 的数对个数。即：

$$\sum_{x = 1}^{n} \sum_{y = 1}^{m} [gcd(x, \ y) = k]$$

我们这么对式子进行变换：
$$
\begin{align}
\sum_{x = 1}^{n} \sum_{y = 1}^{m} [gcd(x, \ y) = k] & = \sum_{x = 1}^{n'} \sum_{y = 1}^{m'} [gcd(x, \ y) = 1] \quad (n' = \lfloor \frac{n}k \rfloor, \ m' = \lfloor \frac{m}k \rfloor) \\
& = \sum_{x = 1}^{n'} \sum_{y = 1}^{m'} \sum_{d | gcd(x, y)} \mu(d) \\
& = \sum_{i = 1}^{min(n', m')} \mu(i) \lfloor \frac{n'}i \rfloor \lfloor \frac{m'}i \rfloor
\end{align}
$$
其中第二行的式子来源于 $\mu \ * \ 1 = e$，$\mu$ 为莫比乌斯函数，$1$ 为常函数（函数值为 $1$），$e$ 为元函数（除 $x = 1$ 处的值为 $1$ 以外函数值为 $0$），$*$ 为狄利克雷卷积。

注意到，在最后的式子中，$\lfloor \frac{n'}i \rfloor$ 只有 $2\sqrt{n}$ 个值（$\lfloor \frac{m'}i \rfloor$ 同），所以我们可以预处理出莫比乌斯函数的前缀和，分块进行计算。

## 代码

说来这道题还可以作为线性筛的模板。。。

```c++
#include <cstdio>
#include <algorithm>
const int MAXN = 100005;
long long mu[MAXN], prime[MAXN], primeCnt;
bool mark[MAXN];
void linearShaker() {
	mu[1] = 1;
	for (int i = 2; i < MAXN; i++) {
		if (!mark[i]) {
			prime[++primeCnt] = i;
			mu[i] = -1;
		}
		for (int j = 1; j <= primeCnt && i * prime[j] < MAXN; j++) {
			mark[i * prime[j]] = true;
			if (i % prime[j] == 0) {
				mu[i * prime[j]] = 0;
				break;
			}
			mu[i * prime[j]] = -mu[i];
		}
	}
	for (int i = 1; i < MAXN; i++) mu[i] += mu[i - 1];
}
long long calc(int n, int m, int k) {
	long long res = 0;
	n /= k, m /= k;
	int last;
	for (int i = 1; i <= m && i <= n; i = last + 1) {
		last = std::min(n / (n / i), m / (m / i));
		res += (mu[last] - mu[i - 1]) * (n / i) * (m / i);
	}
	return res;
}
int main() {
	linearShaker();
	int T;
	scanf("%d", &T);
	while (T--) {
		int a, b, c, d, k;
		scanf("%d %d %d %d %d", &a, &b, &c, &d, &k);
		printf("%lld\n", calc(b, d, k) - calc(a - 1, d, k) - calc(b, c - 1, k) + calc(a - 1, c - 1, k));
	}
	return 0;
}
```