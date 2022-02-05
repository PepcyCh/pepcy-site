---
title: 用 Unity 学习 PBR（三）几何项
date: 2021-02-15 09:34:48
tags: [Unity, CG, PBR, 学习笔记]
categories: 学习笔记（CG）
---

本文先简单介绍了几何阴影函数（Geometry Shadowing Function，GSF）的基本性质，之后大致介绍了 Smith GSF，包括 Smith 遮蔽函数与四种联合遮蔽阴影函数，描述了两个 GGX Smith 高度相关遮蔽阴影函数的近似，最后给出了代码实现与在 Unity 中 GSF 的可视化结果。

参考

* [基于物理的渲染（PBR）白皮书 - 毛星云](https://github.com/QianMo/PBR-White-Paper)
* [Physically Based Rendering Algorithms: A Comprehensive Study In Unity3D](https://www.jordanstevenstechart.com/physically-based-rendering)
* 《Real Time Rendering, 4th》- Chap. 9

<!-- more -->

## 基本性质

因为微平面对光线的遮挡，导致并不是所有微平面法向为 $h$ 的微平面都会产生贡献，几何阴影函数（Geometric Shadowing Function，GSF）用于描述这一现象，是保证 BRDF 能量守恒的重要一项。微平面对光线的遮挡具体又可以分为对入射光遮挡的阴影（shadowing）与对出射光遮挡的遮蔽（masking）。

几何函数有两种形式，一种是只考虑出入射光一侧的遮挡状况的 $G_1(v, m)$ 或 $G_1(l, m)$，另一种同时考虑了两者的 $G_2(l, v, m)$，在认为出入射光的遮挡状况无关时，有 $G_2(l, v, m) = G_1(l, m) G_1(v, m)$。

在法线分布项中，我们提过以下等式：
$$
\int_{\Omega} D(m) (v \cdot m) \mathrm{d} m = (v \cdot n)
$$
但在我们的代码中，点积通常是对 0 取 max，即我们用的一般是 $(v \cdot n)^+ = \max(0, v \cdot n)$，此时上式不一定成立，因为一个方向上可能穿过了多个微平面，在原来的情况下，背光侧的点积为负，与正相消，就只剩一份的贡献，但点积对 0 取 max 后，就可能有多个正的贡献，而我们的几何函数可以认为就是来消除这些额外的贡献的：
$$
\int_{\Omega} G_1(v, m) D(m) (v \cdot m)^+ \mathrm{d} m = (v \cdot n)
$$
由此可见，几何函数的选取受制于法线分布函数，但只有法线分布函数并不能唯一决定几何函数，因为法线分布函数只提供了百分比信息，但遮挡是受具体轮廓影响的。同样的法线分布，轮廓不同，遮挡状况可能大不相同。接下来提到的一些几何函数都是在轮廓不规律的情况下有更好表现的函数，在布料等轮廓很有规律的材质上，效果可能就不够真实。

几何函数的函数值满足 $0 \leq G_2(l, v, m) \leq 1$ 且一般 $G_2(n, n, m) = 1$。当平面比较光滑时，大部分情况下函数值接近 1，除了掠射的时候；平面更粗糙，相同状况下遮蔽更大。

## Implicit GSF

一个简单粗暴的经验模型：
$$
G_2(l, v, m) = (l \cdot n) (v \cdot n)
$$
其最大的好处是，可以与 Cook-Torrance 公式的分母相消，称这个结果为可见性项（visible term）：
$$
V(l, v) = \frac{G_2(l, v, m)}{4(l \cdot n) (v \cdot n)}
$$

## Smith 遮蔽函数

Smith 遮蔽函数（Smith masking function）基于物理，满足之前提过的积分式，而且对真实的反映更好，所以现在被广泛使用。其形式是：
$$
G_1(v, m) = \frac{\chi^+(v, m)}{1 + Ʌ(v)}
$$
$ɅɅ(v)$ 的定义可以看《Real Time Rendering, 4th》或毛神的白皮书（是的我没搞懂）。当法线分布是形状不变时，该函数有解析形式。对于我们在法线分布项中提过的两个形状不变的分布，其公式为：

Beckmann 分布：
$$
Ʌ(v) = \frac{\mathrm{erf}(a) - 1}{2} + \frac{1}{2a \sqrt{\pi}} \exp(-a^2) \\
a = \frac{1}{\alpha \tan \angle(v, n)} \\
\mathrm{erf}(x) = \frac{1}{\sqrt{\pi}} \int_{-x}^x \exp(-t^2) \mathrm{d}t
$$
$\mathrm{erf}(x)$ 是高斯误差函数，难以计算。其一个近似是：
$$
Ʌ(v) = \begin{cases}
\frac{1 - 1.259a + 0.396a^2}{3.535a + 2.181a^2} & a < 1.6 \\
0 & \mathrm{otherwise}
\end{cases}
$$
GGX/TR 分布：
$$
Ʌ(v) = \frac{-1 + \sqrt{1 + 1 / a^2}}{2}
$$


## Smith 联合遮蔽阴影函数

Smith 联合遮蔽阴影函数（Smith Joint Masking-Shadowing Function）即同时考虑了出入射光的 $G_2$。其有四种形式：

* 分离的遮蔽阴影（separable masking and shadowing）
* 高度相关的遮蔽阴影（height-correlated masking and shadowing）
* 方向相关的遮蔽阴影（direction-correlated masking and shadowing）
* 高度方向相关的遮蔽阴影（height-direction-correlated masking and shadowing）

### 分离的遮蔽阴影（separable masking and shadowing）

认为遮蔽和阴影独立，即：
$$
G_2(l, v, m) = G_1(l, m) G_1(v, m) = \frac{\chi^+(v \cdot m)}{1 + Ʌ(v)}\frac{\chi^+(l \cdot m)}{1 + Ʌ(l)}
$$
是目前流行的分布之一。但事实上遮蔽与阴影总是会有一些相关性。

### 高度相关的遮蔽阴影（height-correlated masking and shadowing）

高度是指，当微平面升高时，会同时增大对出入射光的遮挡。公式是：
$$
G_2(l, v, m) = \frac{\chi^+(v \cdot m)\chi^+(l \cdot m)}{1 + Ʌ(v) + Ʌ(l)}
$$
是目前流行的分布之一。与分离的版本相比，计算量差不多，但这个式子会更精确一些。

### 方向相关的遮蔽阴影（direction-correlated masking and shadowing）

方向是指，考虑了出入射光之间夹角的影响。公式是：
$$
G_2 = \lambda(\phi)G_1(v, m)G_1(l, m) + (1 - \lambda(\phi)) \min(G_1(v, m) G_1(l, m))
$$
其中 $\phi$ 为 $l$ 与 $v$ 的夹角，$\lambda(\phi)$ 的一个经验式子是：
$$
\lambda(\phi) = \frac{4.41 \phi}{4.41 \phi + 1}
$$

### 高度方向相关的遮蔽阴影（height-direction-correlated masking and shadowing）

$$
G_2 = \frac{\chi^+(v \cdot m) \chi^+(l \cdot m)}{1 + \max(Ʌ(v), Ʌ(l)) + \lambda(\phi) \min(Ʌ(v), Ʌ(l))}
$$

## GGX 高度相关的遮蔽阴影

把 Ʌ 函数带入后有：
$$
G_1(v, m) = \frac{2 (n \cdot v)}{(n \cdot v) + \sqrt{\alpha^2 + (n \cdot v)^2 (1 - \alpha^2)}} \\
G_2(l, v, m) = \frac{2(n \cdot v) (n \cdot l)}{(n \cdot l) \sqrt{\alpha^2 + (n \cdot v)^2 (1 - \alpha^2)} + (n \cdot v) \sqrt{\alpha^2 + (n \cdot l)^2 (1 - \alpha^2)}}
$$
可以看出，几何项可以和 Cook-Torrance 公式的分母相消得到方便计算的可见性项（显然 GGX 的分离遮蔽阴影函数也可以）：
$$
V(l, v) = \frac{0.5}{(n \cdot l) \sqrt{\alpha^2 + (n \cdot v)^2 (1 - \alpha^2)} + (n \cdot v) \sqrt{\alpha^2 + (n \cdot l)^2 (1 - \alpha^2)}}
$$
分母中的 $(n \cdot l) \sqrt{\alpha^2 + (n \cdot v)^2 (1 - \alpha^2)}$ 的一个近似是 $(n \cdot l)((n \cdot v)(1 - \alpha) + \alpha)$。

Hammon 在 2017 提出以下高度相关的 GGX 近似公式：
$$
V(l, v) = \frac{0.5}{\mathrm{lerp}(2 (n \cdot l) (n \cdot v), (n \cdot l) + (n \cdot v), \alpha)}
$$

## HLSL 代码实现

实现了隐式 GSF、Beckmann 和 GGX 的分离及高度相关的 GSF、GGX 高度相关 GSF 的两个近似。

```c
// GSF
float ImplicitGSF(float NdotL, float NdotV) {
    return NdotL * NdotV;
}
float BeckmannV(float a) {
    float aSqr = a * a;
    return a < 1.6 ? (1 - 1.259 * a + 0.396 * aSqr) / (3.535 * a + 2.181 * aSqr) : 0;
}
float BeckmannSmithSeparableGSF(float NdotL, float NdotV, float a) {
    float V = BeckmannV(NdotV / a / sqrt(1 - NdotV * NdotV));
    float L = BeckmannV(NdotL / a / sqrt(1 - NdotL * NdotL));
    return (1 / (1 + V)) * (1 / (1 + L));
}
float BeckmannSmithHeightCorrelatedGSF(float NdotL, float NdotV, float a) {
    float V = BeckmannV(NdotV / a / sqrt(1 - NdotV * NdotV));
    float L = BeckmannV(NdotL / a / sqrt(1 - NdotL * NdotL));
    return 1 / (1 + L + V);
}
// Visible
float ImplicitVisible() {
    return 0.25;
}
float GGXSmithSeparableVisible(float NdotL, float NdotV, float a2) {
    float V = NdotV + sqrt(a2 + NdotV * NdotV * (1 - a2));
    float L = NdotL + sqrt(a2 + NdotL * NdotL * (1 - a2));
    return 1 / V / L;
}
float GGXSmithHeightCorrelatedVisible(float NdotL, float NdotV, float a2) {
    float V = NdotL * sqrt(a2 + NdotV * NdotV * (1 - a2));
    float L = NdotV * sqrt(a2 + NdotL * NdotL * (1 - a2));
    return 0.5 / (V + L);
}
float GGXSmithHeightCorrelatedVisibleApprox(float NdotL, float NdotV, float a) {
    float L = NdotV * (NdotL * (1 - a) + a);
    float V = NdotL * (NdotV * (1 - a) + a);
    return 0.5 / (V + L);
}
float HammonGGXSmithVisible(float NdotL, float NdotV, float a) {
    return 0.5 / lerp(2 * NdotL * NdotV, NdotL + NdotV, a);
}
// ...
// GSF
// float GSF = ImplicitGSF(NdotL, NdotV);
// float GSF = BeckmannSmithSeparableGSF(NdotL, NdotV, roughness);
// float GSF = BeckmannSmithHeightCorrelatedGSF(NdotL, NdotV, roughness);
// float GSF = GGXSmithSeparableVisible(NdotL, NdotV, roughnessSqr) * (4 * NdotL * NdotV);
// float GSF = GGXSmithHeightCorrelatedVisible(NdotL, NdotV, roughnessSqr) * (4 * NdotL * NdotV);
float GSF = GGXSmithHeightCorrelatedVisibleApprox(NdotL, NdotV, roughness) * (4 * NdotL * NdotV);
// float GSF = HammonGGXSmithVisible(NdotL, NdotV, roughness) * (4 * NdotL * NdotV);
// Visible
// float Visible = GSF / max(EPS, 4 * NdotL * NdotV)
// float Visible = BeckmannSmithSeparableGSF(NdotL, NdotV, roughness) / max(EPS, 4 * NdotL * NdotV);
// float Visible = BeckmannSmithHeightCorrelatedGSF(NdotL, NdotV, roughness) / max(EPS, 4 * NdotL * NdotV);
// float Visible = GGXSmithSeparableVisible(NdotL, NdotV, roughnessSqr);
// float Visible = GGXSmithHeightCorrelatedVisible(NdotL, NdotV, roughnessSqr);
float Visible = GGXSmithHeightCorrelatedVisibleApprox(NdotL, NdotV, roughness);
// float Visible = HammonGGXSmithVisible(NdotL, NdotV, roughness);
```

## 可视化结果

让片段着色器以灰色直接输出 GSF 的值，结果如下：

![03-gsf.png](https://i.loli.net/2021/02/14/ftwDMvRK8LECPXJ.png)