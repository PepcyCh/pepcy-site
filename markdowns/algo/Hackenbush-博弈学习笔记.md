---
title: Hackenbush 博弈学习笔记
date: 2018-12-01 20:00:06
tags: [博弈,Hackenbush,学习笔记]
categories: 学习笔记（OI/XCPC）
---

两人在一张有根图上依次操作，每次操作为删去一条边，若有连通块内不含根节点，则删去整个连通块。先不能操作者输，判断先手是否必胜。

<!-- more -->

## 链的版本 - Bamboo Stalks

两人操作 $n$ 条长度为 $l_i$ 的链，链的一端连在一条线上（图中虚线，不可操作），这一端作为链的根，具体操作同上，判断先手是否必胜。

![Figure - Bamboo Stalks](https://i.loli.net/2018/12/01/5c026a735d14d.jpg)

其实 Bamboo Stalks 游戏可以转化为普通的取石子 Nim 游戏：删去某条边等于在这一堆石子中取走被删掉的点数个石子。所以 Bamboo Stalks 游戏可以像取石子游戏一般定义 SG 函数。

## 树的版本 - Hackenbush-Trees

即在 Bamboo Stalks 的基础上将链改为树，判断先手是否必胜。

![Figure - Hackenbush-Trees](https://i.loli.net/2018/12/01/5c026a74191d1.jpg)

可以用一条链来代替一个顶点的儿子们，其长度等于它们的异或和，如图：

![Figure - Hackenbush-Trees-alter](https://i.loli.net/2018/12/01/5c02745dec0b3.jpg)

于是可按这个规律定义一棵子树的 SG 函数：

$$
SG(u) = (SG(v_1) + 1) \oplus (SG(v_2) + 1) \oplus \cdots \oplus (SG(v_m) + 1)
$$

其中 $v_1, v_2, \dots, v_m$ 为 $u$ 的所有儿子，单节点的 SG 函数值为 $0$。

## 原题 - Hackenbush-Graphs

![Figure - Hackenbush-Graphs](https://i.loli.net/2018/12/01/5c026a73b9cc6.jpg)

在 Hackenbush-Graphs 中，任何环内的节点可以融合成一点而不会改变图的 SG 值；同时，一个自环可以等价于一条伸出来的长为 $1$ 的链。如图：

![Figure - Hackenbush-Graphs-alter-1](https://i.loli.net/2018/12/01/5c02763ba2dec.jpg)

![Figure - Hackenbush-Graphs-alter-2](https://i.loli.net/2018/12/01/5c026a73c90ca.jpg)

即偶数长的环可缩为一点，奇数长的环可缩为一点加一个长为 $1$ 的链。然后就可以按照 Hackenbush-Trees 做了。

## 扩展

在 Hackenbush-Graphs 的基础上，为每一条边加一个边权，每次操作只能使边权减一，边在权值减为 $0$ 时被删去，判断先手是否必胜。

把每一条边分成权值条重边即转化为 Hackenbush-Graphs。