---
title: '[POJ 2774] Long Long Message'
date: 2017-04-14 22:13:02
tags: [字符串, 后缀数组]
categories: 题解（OI/XCPC）
---

## 题目大意

给定两个字符串 $S$、$T$。求两个字符串的最长公共字串。

$1 \leqslant |S|, \; |T| \leqslant 10,000$

## 题目链接

[POJ 2774](http://poj.org/problem?id=2774)

<!-- more -->

## 题解

连接两个字符串，中间用不属于原字符集的字符隔开。

建立后缀数组，当排名相连的两个后缀满足：一个的第一个字符在前一个字符串中，另一在后一个字符串中时，用 $heigh[i]$ 更新答案。

## 代码

```c++
#include <cstdio>
#include <cstring>
#include <algorithm>
// #define DBG
const int MAXN = 100005;
namespace SuffixArray {
    const int MAXN = ::MAXN << 1;
    int rank[MAXN], sa[MAXN], height[MAXN], n;
    char str[MAXN];
    void buildSA(int m) {
        static int fir[MAXN], sec[MAXN], temp[MAXN], cnt[MAXN], i;
        n = strlen(str);
        memset(cnt, 0, sizeof (cnt));
        for (i = 0; i < n; i++) cnt[(int) str[i]]++;
        for (i = 1; i < m; i++) cnt[i] += cnt[i - 1];
        for (i = 0; i < n; i++) rank[i] = cnt[(int) str[i]] - 1;
        for (int l = 1; l < n; l <<= 1) {
            for (i = 0; i < n; i++)
                fir[i] = rank[i], sec[i] = i + l < n ? rank[i + l] : 0;
            memset(cnt, 0, sizeof (cnt));
            for (i = 0; i < n; i++) cnt[sec[i]]++;
            for (i = 1; i < n; i++) cnt[i] += cnt[i - 1];
            for (i = n - 1; ~i; i--) temp[--cnt[sec[i]]] = i;
            memset(cnt, 0, sizeof (cnt));
            for (i = 0; i < n; i++) cnt[fir[i]]++;
            for (i = 1; i < n; i++) cnt[i] += cnt[i - 1];
            for (i = n - 1; ~i; i--) sa[--cnt[fir[temp[i]]]] = temp[i];
            bool unique = true;
            rank[sa[0]] = 0;
            for (i = 1; i < n; i++) {
                rank[sa[i]] = rank[sa[i - 1]];
                if (fir[sa[i]] == fir[sa[i - 1]] && sec[sa[i]] == sec[sa[i - 1]]) 
                    unique = false;
                else rank[sa[i]]++;
            }
            if (unique) break;
        }
    }
    void calcHeight() {
        for (int i = 0, k = 0; i < n - 1; i++) {
            k ? k-- : 0;
            int j = sa[rank[i] - 1];
            while (str[i + k] == str[j + k]) k++;
            height[rank[i]] = k;
        }
    }
}
int main() {
    static char str1[MAXN], str2[MAXN];
    scanf("%s %s", str1, str2);
    int l1 = strlen(str1), l2 = strlen(str2);
    str1[l1++] = '#';
    str2[l2++] = '$';
    char *str = SuffixArray::str;
    for (int i = 0; i < l1; i++) str[i] = str1[i];
    for (int i = 0; i < l2; i++) str[i + l1] = str2[i];
    SuffixArray::buildSA(128);
    SuffixArray::calcHeight();
    int ans = 0;
    int *height = SuffixArray::height + 2, *sa = SuffixArray::sa + 2, 
        n = l1 + l2 - 2;
    l1--;
#ifdef DBG
    printf("new string is: %s\n", SuffixArray::str);
    printf("sa: ");
    for (int i = 0; i < n; i++) printf("%d%c", sa[i], i == n - 1 ? '\n' : ' ');
    printf("height: ");
    for (int i = 1; i < n; i++) printf("%d%c", height[i], i == n - 1 ? '\n' : ' ');
#endif
    for (int i = 1; i < n; i++) {
        if ((sa[i] < l1 && sa[i - 1] > l1) || (sa[i] > l1 && sa[i - 1] < l1)) 
            ans = std::max(ans, height[i]);
    }
    printf("%d\n", ans);
    return 0;
}
```