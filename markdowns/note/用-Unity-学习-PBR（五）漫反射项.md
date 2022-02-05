---
title: 用 Unity 学习 PBR（五）漫反射项
date: 2021-02-15 09:37:29
tags: [Unity, CG, PBR, 学习笔记]
categories: 学习笔记（CG）
---

本文大致描述了直接光照中的漫反射项中的光滑表面模型与粗糙表面模型。在光滑表面模型中，描述了如何把菲涅尔现象加入 Lambert 公式之中；在粗糙表面模型中，简要介绍了 Disney 漫反射模型、Oren-Nayar 漫反射模型与 Hammon 满反射模型。最后给出了代码实现与在 Unity 中的结果。

参考

* [基于物理的渲染（PBR）白皮书 - 毛星云](https://github.com/QianMo/PBR-White-Paper)
* [Physically Based Rendering Algorithms: A Comprehensive Study In Unity3D](https://www.jordanstevenstechart.com/physically-based-rendering)
* 《Real Time Rendering, 4th》- Chap. 9
* [A tiny improvement of Oren-Nayar reflectance model](https://mimosa-pudica.net/improved-oren-nayar.html)

<!-- more -->

在物理原理上，漫反射与次表面散射有相似性，所以《Real Time Rendering, 4th》中都是在说次表面模型的里面说漫反射项。

## 光滑表面模型

光滑表面模型不考虑粗糙度带来的影响，最经典的就是 Lambert 模型

$$
f_{\mathrm{diff}}(l, v) = \frac{\rho_{\mathrm{ss}}}{\pi}
$$
其中 $\rho_{\mathrm{ss}}$ 为反照率（albedo），该参数通常以基础颜色（base color）或漫反射颜色（diffuse color）的名称出现。分母的 $\pi$ 是 BRDF 的归一化系数。

HLSL 实现：

```c
float3 LambertDiffuse(float3 albedo) {
    return albedo / PI;
}
```

我们把出射光分为了漫反射与镜面反射，该式相当于认为所有的出射光都是漫反射，但实际上显然不是如此。一个常用的方法是乘上 1 减去菲涅尔项。
$$
f_{\mathrm{diff}}(l, v) = (1 - F(l, h))\frac{\rho_{\mathrm{ss}}}{\pi}
$$
其中 $F(l, h)$ 有时也用 $F(l, n)$，在粗糙面上使用 $h$ 会更有道理，而在完美镜面上则是使用 $n$ 更有道理。

这个公式认为漫反射项与观察方向无关，但实际并非如此（因为光线的可逆性，只与入射光有关的话，逆向光线就是只与出射光有关，这是不符合公式的，所以要是与入射光有关，就与出射光以同样的方式有关）。

一种考虑了二者的公式是：
$$
f_{\mathrm{diff}}(l, v) = \frac{21}{20 \pi} \rho_{\mathrm{ss}} (1 - F_0) (1 - (1 - (n \cdot l)^+)^5)(1 - (1 - (n \cdot v)^+)^5)
$$

## 粗糙表面模型

真实世界中的一些粗糙表面，如混凝土等会呈现出不那么 Lambert 的漫反射表现，具体来说，Lambert 的结果表现出暗部过暗。

### Disney Diffuse

Disney 原则的 BRDF 使用了如下的公式，Stevens 的教程中亦使用如下公式：
$$
f_{\mathrm{diff}} (l, v) = \frac{\rho_{\mathrm{ss}}}{\pi} (1 + (F_{D90} - 1)(1 - (n \cdot l)^+)^5) (1 + (F_{D90} - 1)(1 - (n \cdot v)^+)^5) \\
F_{D90} = 0.5 + 2 \sqrt{\alpha} (l \cdot h)^2
$$

HLSL 实现：

```c
float3 DisneyDiffuse(float3 albedo, float LdotH, float NdotL, float NdotV, float a) {
    float fd90 = 0.5 + 2 * sqrt(a) * LdotH * LdotH;
    return albedo * lerp(1, fd90, pow5(1 - NdotV)) * lerp(1, fd90, pow5(1 - NdotL)) / PI;
}
```

### Oren-Nayar

Oren-Nayar 模型基于微平面模型，选取了高斯分布的法线分布与 V 腔假设（V-cavity，轮廓是一片 V 型）的几何遮蔽阴影，每个微平面认为是符合 Lambert 公式。得到的结果如下：
$$
f_{\mathrm{diff}} (l, v) = \frac{\rho_{\mathrm{ss}}}{\pi} (A + B \frac{s}{t}) \\
A = 1 - 0.5\frac{\sigma^2}{\sigma^2 + 0.33} \\
B = 0.45\frac{\sigma^2}{\sigma^2 + 0.09} \\
s = (l \cdot v) - (n \cdot l) (n \cdot v) \\
t = \begin{cases}
1 & s \leq 0 \\
\max(n \cdot l, n \cdot v) & s > 0
\end{cases}
$$
该结果被称作量化的 Oren-Nayar，原始论文还提出了一个精确的 Oren-Nayar 公式，但计算太复杂了。

一种更常见的形式是（区别在 $s/t$ 一项）：
$$
f_{\mathrm{diff}}(l, r) = \frac{\rho_{\mathrm{ss}}}{\pi} (A + B \max(0, \cos (\phi_i - \phi_r)) \sin \alpha \tan \beta) \\
\alpha = \max(\theta_i, \theta_r) \\
\beta = \min(\theta_i, \theta_r) \\
$$
不过推一推发现基本是一样的（上推下推起来还挺简单的）（但感觉 $s \leq 0$ 的情况，即 $\cos (\phi_i - \phi_r) \leq 0$ 时的结果不太对。。。），但显然上面的式子算起来更快一些。

文章 [A tiny improvement of Oren-Nayar reflectance model](https://mimosa-pudica.net/improved-oren-nayar.html) 对量化的 Oren-Nayar 中的 $A$ 做了一些修改，让结果更加接近精确算式一些：
$$
A = 1 - 0.5\frac{\sigma^2}{\sigma^2 + 0.33} + 0.17 \rho_{ss} \frac{\sigma^2}{\sigma^2 + 0.13}
$$

以上公式中 $\sigma^2$ 取值在 $0 \sim \infin$，与粗糙度有关。

HLSL 实现：

```c
float3 OrenNayar(float3 albedo, float LdotV, float NdotL, float NdotV, float sigma2) {
    float A = 1 - 0.5 * sigma2 / (sigma2 + 0.33);
    float B = 0.45 * sigma2 / (sigma2 + 0.09);
    float s = LdotV - NdotV * NdotL;
    float tInv = s > 0 ? 1 / max(EPS, max(NdotV, NdotL)) : 0;
    return albedo * (A + B * s * tInv) / PI;
}
```

### Hammon’s Diffuse BRDF

Oren-Nayar 的推导中使用的法线分布与几何遮蔽阴影模型并非是目前流行的 GGX NDF 与 Smith G，一种使用 GGX NDF 与 Smith 高度相关遮蔽阴影函数的推导结果是 Hammond 给出的如下公式：
$$
f_{\mathrm{diff}}(l, v) = \frac{\rho_{\mathrm{ss}}}{\pi} ((1 - \alpha) f_{\mathrm{smooth}} + \alpha f_{\mathrm{rough}} + \rho_{\mathrm{ss}} f_{\mathrm{multi}}) \\
f_{\mathrm{smooth}} = \frac{21}{20} (1 - F_0) (1 - (1 - (n \cdot l)^+)^5)(1 - (1 - (n \cdot v)^+)^5) \\
f_{\mathrm{rough}} = k_{\mathrm{facing}} (0.9 - 0.4 k_{\mathrm{facing}}) \frac{0.5 + (n \cdot h)}{n \cdot h} \\
k_{\mathrm{facing}} = 0.5 + 0.5 (l \cdot v) \\
f_{\mathrm{multi}} = 0.3641 \alpha
$$

## Unity 结果

让片段着色器直接输出直接光照的漫反射项，结果如下：

![05-diffuse.png](https://i.loli.net/2021/02/14/lycgCWEFdq2Ufv5.png)