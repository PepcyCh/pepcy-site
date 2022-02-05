---
title: 理解 ReSTIR
date: 2021-11-04 22:05:00
tags: [CG, 学习笔记, 实现笔记, 实时光线追踪, 光线追踪]
categories: 学习笔记（CG）
---

ReSTIR（Reservoir-based Spatiotemporal Importance Resampling）是 NVIDIA 在 2020 年发表的一个实时光追中对直接光更好地进行采样的算法，其实现相对简单、效果好、并且支持动态场景，相应的论文为 SIGGRAPH 2020 的[《Spatiotemporal Reservoir Resampling for Real-time Ray Tracing with Dynamic Direct Lighting》](https://research.nvidia.com/publication/2020-07_Spatiotemporal-reservoir-resampling)。

本人在实现这一算法时（[ReSTIR-OptiX](https://github.com/PepcyCh/ReSTIR-OptiX)）却在一开始遇到了一些问题，具体表现为理解与代码对不上，从而导致实现有误。NVIDIA 在 HPG 2021 中发表了论文《[Rearchitecting Spatiotemporal Resampling for Production](https://research.nvidia.com/publication/2021-07_Rearchitecting-Spatiotemporal-Resampling)》，对原始的 ReSTIR 做了一些修改使其更适用于实际应用（此改动不在本文讨论范围内），同时也对 ReSTIR 算法做了更细致的讲解。本人结合该文章的解释与自己在实现时的理解，指出了 ReSTIR 实为多级 RIS 的本质，并解释了一些原论文（指 SIGGRAPH 2020 的那篇，后同）伪代码与理解上的差异，做笔记于此。

<!-- more -->

## RIS (Resample Importance Sampling)

### RIS 的一种由来

Monte Carlo 估计，其中 $f$ 为目标函数，$q$ 为采样的 PDF
$$
I \approx \frac{1}{N} \sum_{i = 1}^N \frac{f(x_i)}{q(x_i)}
$$
若使用非归一化的函数 $\hat{p}$ 采样，则相应的 PDF $p$ 为
$$
p(x) = \frac{\hat{p}(x)}{\int_{\Omega} p(x) \mathrm{d}x}
$$
相应的 MC 估计为
$$
I \approx \frac{1}{N} \sum_{i = 1}^N \frac{f(x_i)}{\hat{p}(x_i)} \int_{\Omega} p(x) \mathrm{d}x
$$
用于归一化的积分项也用 MC 估计，其采样 PDF 为 $q$，有
$$
I \approx \frac{1}{N} \sum_{i = 1}^N \left( \frac{f(x_i)}{\hat{p}(x_i)} \frac{1}{M_i} \sum_{j = 1}^{M_i} \frac{\hat{p}(x_{ij})}{q(x_{ij})} \right)
$$
（后续可能为了方便略去 $M_i$ 的下标）

其中
$$
w(x_i) = \frac{1}{\hat{p}(x_i)} \frac{1}{M_i} \sum_{j = 1}^{M_i} \frac{\hat{p}(x_{ij})}{q(x_{ij})}
$$
就是实际最终采出样本 $x_i$ 的 PDF 的倒数，其期望值就是 $1 / p(x_i)$。

用于外层 MC 估计的样本 $x_i$ 可以从内层 MC 估计时生成的样本 $x_{ij}$ 中以某种方式取得而不再次生成一个样本，这就是 RIS（Resample Importance Sampling，重采样重要性采样）。

无偏的条件同 MC 估计，即目标函数有非零值的地方都有概率被采到，$\hat{p}$ 与 $q$ 的支集之间的关系到没有什么要求（应该）。

### WRS (Weighted Reservoir Sampling)

WRS（Weighted Reservoir Sampling，加权水槽采样）可以用于上述 RIS 操作。

原始的水槽采样是为了解决以下问题：输入一串不知道有多少个但有限的元素，在线、等概率地选取其中一个。算法可以大致理解为，当一个新元素进来时，认为它是最后一个元素，以相应的概率替换当前选择的元素。容易证明这是对的。

WRS 则是为每一个元素赋予一个权值，使得最后选取某元素的概率线性相关与该值，算法同理。RIS 中的这个权重就是 $\hat{p} / q$，因为样本已经是按 $q$ 为 PDF 采出的，所以要除以 $q$。

WRS 本身是在线的这一特征使其不用存储输入样本。

### RIS 的边界情况

当 $\hat{p} = q$ 时，退化到只有 $N$ 个样本的 MC 估计，而且花费了更多的时间。

当 $f = \hat{p}$ 时，是有 $NM$ 个样本的 MC 估计，不过从算法上来说，呈现出了分层的感觉。在使用 RIS 的时候，可以生成一批统一的内层 MC 的样本用于外层 MC 样本的重采样，可以减少总共生成的初始样本的数目。

当 $M = 1$ 时，就是只有 $N$ 个样本的 MC 估计，对应 $w = 1 / q$。

当 $M \to \infty$ 时，可以认为内部 MC 收敛的期望值，对应 $w = 1 / p = \int \hat{p} \mathrm{d}x / \hat{p}$。

$M$ 可以视为一种插值系数，让最终样本的采样 PDF 在 $q$ 与 $p$ 之间变化。

### 多级 RIS

我们上面得出 RIS 的过程可以继续把 $q$ 展开，形成多级 RIS。如二级的 RIS：
$$
I \approx \frac{1}{N} \sum_{i = 1}^N \left( \frac{f(x_i)}{\hat{p}_0(x_i)} \frac{1}{M_i} \sum_{j = 1}^{M_i} \left( \frac{\hat{p}_0(x_{ij})}{\hat{p}_1(x_{ij})} \frac{1}{M_{ij}} \sum_{k = 1}^{M_{ij}}\frac{\hat{p}_1(x_{ijk})}{q(x_{ijk})} \right) \right)
$$

### 不同内层 PDF 的 RIS

内层的 PDF $q$ 可以不同，如每个外层样本都使用一个不同内层 PDF $q_j$：
$$
I \approx \frac{1}{N} \sum_{i = 1}^N \left( \frac{f(x_i)}{\hat{p}(x_i)} \frac{1}{M} \sum_{j = 1}^M \frac{\hat{p}(x_{ij})}{q_j(x_{ij})} \right)
$$
因为不同的 PDF $q_j$ 的支集可能不同，可能会使得 $q_j(x_i) = 0$，此时 RIS 估计不再无偏。可以证明此时的期望是（证明不算很难，就是有点麻烦，可以参考原论文的附录）
$$
E[w(x_i)] = \frac{1}{p(x_i)} \frac{\vert Z(x_i) \vert}{M} \\
Z(x_i) = \{ j \mid 1 \leq j \leq M ~\land~ q_j(x_i) > 0 \}
$$
一个简单的想法是，使用 $1 / \vert Z(x) \vert$ 代替原来的 $1 / M$。另一点，也可以使用不平均的系数
$$
I \approx \frac{1}{N} \sum_{i = 1}^N \left( \frac{f(x_i)}{\hat{p}(x_i)} m(x_i) \sum_{j = 1}^M \frac{\hat{p}(x_{ij})}{q_j(x_{ij})} \right)
$$
只要下式满足就是无偏的。
$$
\sum_{z \in Z(x_i)} m(x_z) = 1
$$
这使得我们可以使用类似启发式 MIS 系数来作为 $m(x_z)$，即
$$
m(x_z) = \frac{q_z(x_z)}{\sum_{j = 1}^M q_j(x_z)}
$$
类似的，多级 RIS 时中间几层的 $\hat{p}$ 也可以不同，其相应的无偏修正方法也类似。

### 加权的 RIS

因为 MC 是可以加权的，所以可以用下式来估计
$$
I \approx \frac{1}{N} \sum_{i = 1}^N \left( \frac{f(x_i)}{\hat{p}_0(x_i)} \frac{1}{\sum_{j = 1}^{M_i} K_{ij}} \sum_{j = 1}^{M_i} \left( K_{ij} \frac{\hat{p}_0(x_{ij})}{\hat{p}_1(x_{ij})} \frac{1}{M_{ij}} \sum_{k = 1}^{M_{ij}}\frac{\hat{p}_1(x_{ijk})}{q(x_{ijk})} \right) \right)
$$
对于最内层 MC，更大的 $M_{ij}$ 会产生更小的方差，调整中层的权值，可以让 $M_{ij}$ 更大的项有更大的权值，从而得到更小的方差。

$K_{ij} = M_{ij}$ 时，就是原论文中的合并水槽，即合并水槽可以理解为一次加权的 RIS。

## PDF 与随机变量

当我们说 PDF 时，一定是在说某个随机变量的 PDF。当随机变量发生改变时，PDF 要随之发生改变。

比如一个面积为 $A$ 的平面光源，均匀采样一点的 PDF 是 $1 / A$，即光源位置的 PDF。当变换到光源方向时，PDF 转为立体角的 PDF，就要乘上距离平方、除以余弦项。

原论文让初始采样 PDF 为一个与着色点无关的光源位置 PDF，并将转换为立体角 PDF 的系数记作 $G$ 项，避开了对 PDF 随机变量改变的说明。

## ReSTIR

### 大致算法流程与多级 RIS 本质

RIS 取 $N = 1$。

对于每个像素：

* 对光源采样，生成 $M$ 个初始样本，PDF 比例于光源亮度（乘以光源面积）
* 使用 RIS，选取一个样本，$\hat{p} = f_r \times L \times G$ 或 $\hat{p} = f_r \times L \times G \times V$
  * $f_r$：BRDF 与余弦项
  * $L$：光源亮度
  * $G$：光的余弦项及距离平方倒数，光的衰减项或 PDF 的变换系数
  * $V$：0-1 的可见性项
* 时间上的复用，合并当前帧与上一帧对应位置的两个水槽
* 可见性复用，若当前光源样本对着色点不可见，“丢弃”样本，即置样本权值为零。
* 空间上的复用，合并自身与附近几个位置的水槽，需要无偏时，进行系数纠正

一帧之中，可以把 ReSTIR 写作如下的三级 RIS：
$$
I \approx \frac{f(x)}{\hat{p}_0(x)} \frac{1}{\sum_{s \in Z(x)} K_{s}} \sum_{s = 1}^{M_s} \left( K_s \frac{\hat{p}_0(x_s)}{\hat{p}_{1s}(x_s)} \frac{1}{K_i + K_h} \left( K_i \frac{\hat{p}_{1s}(x_{si})}{\hat{p}_{2s}(x_{si})} \frac{1}{M_i} \sum_{j = 1}^{M_i} \frac{\hat{p}_{2s}(x_{sij})}{q(x_{sij})}  + K_h \Delta \right) \right)
$$
其中：

* $M_i$ 表示一个像素的初始样本数（$i$ 指 initial），原论文中，$M_i = 32$
* $\Delta$ 表示上一帧相关的内容，是一个与此式类似的东西
* $K_i$ 与 $K_h$ 指时间上复用的权值（$h$ 指 history），原论文中，$K_i = M_i$、$K_h = \min(M_h, 20M_i)$
* $M_s$ 表示空间上复用时的样本数（$s$ 指 spatial），原论文中，$M_s = 4$（无偏）或 $M_s = 6$（有偏）（与原论文比增加了 1，指像素自己）
* $K_s$ 表示空间上复用的权值，原论文中，$K_s = K_i + K_h$
* $q(x)$ 表示光源采用的 PDF，关于光源位置
* $\hat{p}_0(x)$、$\hat{p}_{1s}(x)$、$\hat{p}_{2s}(x)$ 都是 $f_r \times L \times G$（无可见性复用）或 $f_r \times L \times G \times V$（有可见性复用），后二者中的下标 $s$ 表示它们来自空间中不同的着色点

将几个 $K$ 替换掉，可以得到更接近原论文给出的伪代码的样子：
$$
I \approx \frac{f(x)}{\hat{p}_0(x)} \frac{1}{\sum_{s \in Z(x)} K_{s}} \sum_{s = 1}^{M_s} \left( \frac{\hat{p}_0(x_s)}{\hat{p}_{1s}(x_s)} \left( \frac{\hat{p}_{1s}(x_{si})}{\hat{p}_{2s}(x_{si})}\sum_{j = 1}^{M_i} \frac{\hat{p}_{2s}(x_{sij})}{q(x_{sij})}  + \min(M_h, 20M_i) \Delta \right) \right)
$$

### 时间上的复用（temporal reuse）

如之前所说，合并水槽就是加权的一次 RIS。这也可以解释为什么可以随便设置一个样本数的截断上界（如 20 倍）而结果依然正确。

### 可见性复用（visibility reuse）

如之前所说，可见性复用相当于在 RIS 的 $\hat{p}$ 中加入 $V$，但并没有在每次生成样本时都进行可见性测试，而是在最后做，相当于进行拒绝采样。

### 空间上的复用（spatial reuse）

如之前所说，就是一次内层 PDF 不同的加权 RIS。

从之前给出的三级 RIS 的式子中可以看到，要判断样本来源是否在 $Z(x)$ 中时，应当使用的是 $\hat{p}_{1s}$。这解释了原论文中推导部分使用的 $q$ 但伪代码中使用却是 $\hat{p}$。

