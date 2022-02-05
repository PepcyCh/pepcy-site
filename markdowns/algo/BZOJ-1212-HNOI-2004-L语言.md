---
title: '[HNOI 2004] L语言'
date: 2017-04-13 22:12:10
tags: [Trie]
categories: 题解（OI/XCPC）
---

## 题目大意

称一段文章 $T$ 在某个字典 $D$ 是可以被理解的，是指如果文章 $T$ 可以被分成若干部分，且每一个部分都是字典 $D$ 中的单词。

给定一个字典 $D$（有单词 $n$ 个），判断 $m$ 段文章在字 $D$ 下是否能够被理解。并给出其在字典 $D$ 下能够被理解的最长前缀的位置。

$1 \leqslant n, \; m \leqslant 20$

$1 \leqslant len \leqslant 1M$ （每段文章的长度）

## 题目链接

[【HNOI 2004】L 语言 - Luogu 2292](https://www.luogu.com.cn/problem/P2292)

<!-- more -->

## 题解

对字典建立 Trie，在判断每段文章时，记 $f[i]$ 表示文章的前 $i$ 个字符是否可理解。若 $f[i - 1]$ 为真，则可以判断 $f[i - 1 + m]$ 是否为真（$m$ 为字典中单词的长度）。最大的满足 $f[i]$ 为真的 $i$ 就是答案。

## 代码

```c++
#include <cstdio>
#include <cstring>
#include <algorithm>
const int CHARSET_SIZE = 'z' - 'a' + 1;
const char BASE_CHAR = 'a';
const int MAXLEN = 1048580;
struct Trie {
    struct Node {
        Node *c[CHARSET_SIZE];
        bool isWord;
        Node (bool isWord) : isWord(isWord) {
            for (int i = 0; i < CHARSET_SIZE; i++) c[i] = NULL;
        }
    } *root;
    Trie() : root(NULL) {}
    void insert(char *begin, char *end) {
        Node **u = &root;
        for (char *p = begin; p != end; p++) {
            if (!*u) *u = new Node(false);
            u = &(*u)->c[*p];
        }
        if (!*u) *u = new Node(true);
        else (*u)->isWord = true;
    }
} trie;
int main() {
    int n, m;
    scanf("%d %d", &n, &m);
    static char s[MAXLEN];
    while (n--) {
        scanf("%s", s);
        int len = strlen(s);
        for (int i = 0; i < len; i++) s[i] -= 'a';
        trie.insert(s, s + len);
    }
    while (m--) {
        scanf("%s", s + 1);
        int len = strlen(s + 1);
        static bool f[MAXLEN];
        for (int i = 1; i <= len; i++) s[i] -= 'a', f[i] = false;
        int ans = 0;
        f[ans] = true;
        for (int i = 1; i <= len; i++) {
            if (!f[i - 1]) continue;
            Trie::Node *u = trie.root;
            for (int j = i; j <= len; j++) {
                if (!u->c[s[j]]) break;
                u = u->c[s[j]];
                if (u->isWord) f[j] = true, ans = std::max(ans, j);
            }
        }
        printf("%d\n", ans);
    }
    return 0;
}
```