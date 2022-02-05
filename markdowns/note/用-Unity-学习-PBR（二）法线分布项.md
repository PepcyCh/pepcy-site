---
title: 用 Unity 学习 PBR（二）法线分布项
date: 2021-02-14 15:04:31
tags: [CG, PBR, Unity, 学习笔记]
categories: 学习笔记（CG）
---

本文先简单介绍了法线分布函数（Normal Distribution Function，NDF）的基本性质，之后描述了 Blinn-Phong、Beckmann、GGX / TR、GTR 等分布函数，简单介绍了 NDF 的形状不变性与各向异性的 NDF，最后给出了在 Unity 中各个 NDF 的可视化结果。

参考

* [基于物理的渲染（PBR）白皮书 - 毛星云](https://github.com/QianMo/PBR-White-Paper)
* [Physically Based Rendering Algorithms: A Comprehensive Study In Unity3D](https://www.jordanstevenstechart.com/physically-based-rendering)
* 《Real Time Rendering, 4th》- Chap. 9

<!-- more -->

## 基本性质

法线分布函数（Normal Distribution Function，NDF）描述了各个微平面法向方向的占比，可以认为是，这一块宏平面下微平面法向为某个值的微平面的面积，其满足如下归一化条件：
$$
\int_{\Omega} D(m) (m \cdot n) \mathrm{d}m = 1
$$
即所有微平面的投影面积是宏平面面积。更加推广一些的式子是：
$$
\int_{\Omega} D(m) (m \cdot v) \mathrm{d} m = (v \cdot n)
$$
即所有微平面在某个方向的投影面积是宏平面在这个方向上的投影面积。其中 $D(m) (m \cdot v)$ 可能会有负值，但是会有相应的正值与之抵消。

另外，因为 $D(m)$ 是一个分布函数，它也满足 $D(m) \geq 0$。

如果 $D(m)$ 的表达式里只有 $(n \cdot m)$ 而且是各向同性的话，那这个积分会相对好算很多（有时要算归一化系数，得算一下积分）：
$$
\int_{\Omega} g(n \cdot m) (n \cdot m) \mathrm{d}m = \int_{0}^{2\pi} \mathrm{d}\varphi \int_{0}^{\pi / 2} g(\cos \theta) \cos \theta \sin \theta \mathrm{d} \theta = 2 \pi \int_{0}^{1} xg(x) \mathrm{d} x
$$

## Blinn-Phong

Blinn-Phong 光照模型大家应该都知道，其镜面反射部分是 $(n \cdot m)^{\alpha_p}$，但是直接用该式作为 NDF 的话，其积分不是 1，所以需要进行归一化：
$$
D(m) = \chi^+(n \cdot m) \frac{2 + \alpha_p}{2 \pi} (n \cdot m)^{\alpha_p}
$$
其中 $\chi^+(x) = [x > 0]$。写代码的时候我们都会写 `NdotM = max(0, dot(norm, half))`，相当于起到了 $\chi^+(n \cdot m)$ 的作用。该项严谨一点的话在公式中写上可能比较好，不过不写大家也应该知道是这么个意思。

$\alpha_p$ 是描述平面光滑程度的因子，取值为 $0 \sim \infin$，值越大则越光滑。它与我们 $0 \sim 1$ 的粗糙度参数 $\alpha$ 可以用如下的方式对应：

* $\alpha_p = 2 \alpha^{-2} - 2$（我在代码中选择的方式）
* $\alpha_p = K(1 - \alpha)$
* ……

HLSL 实现：

```c
float BlinnPhongNDF(float NdotH, float ap) {
    return pow(NdotH, ap) * (2 + ap) / (2 * PI);
}
// ...
float NDF = BlinnPhongNDF(NdotH, 2 / max(roughnessSqr, EPS) - 2);
```

为了便于调试与观察 NDF，把 `float4(NDF, NDF, NDF, 1)` 直接作为片段着色器的输出。

## Beckmann

Beckmann NDF 的提出其实比 Blinn-Phong 还要早些，其公式为：
$$
D(m) = \frac{\chi^+(n \cdot m)}{\pi \alpha^2 (n \cdot m)^4} \exp(\frac{(n \cdot m)^2 - 1}{\alpha^2 (n \cdot m)^2})
$$
Beckmann NDF 在比较粗糙时，最大值并不在 $(n \cdot m) = 1$ 时取到。

当 Blinn-Phong 中的 $\alpha_p$ 选用 $2 \alpha^{-2} - 2$ 、粗糙度较低时，二者的曲线很接近。

HLSL 实现：

```c
float BeckmannNDF(float NdotH, float a2) {
    float NdotHSqr = NdotH * NdotH;
    return exp((NdotHSqr - 1) / (a2 * NdotHSqr)) / max(EPS, PI * a2 * NdotHSqr * NdotHSqr);
}
// ...
float NDF = BeckmannNDF(NdotH, roughnessSqr);
```

## GGX / Trowbridge-Reitz

GGX NDF 或 TR NDF，目前比较流行的分布，先于上个世纪七十年代由 Trowbridge-Reitz 提出，后在 2007 年由 Walter 等人独立重新发现并命名为GGX（GGX 一词本身好像并没有什么意思），其公式为：
$$
D(m) = \frac{\alpha^2 \chi^+(n \cdot m)}{\pi ((n \cdot m)^2(\alpha^2 -1) + 1)^2}
$$
Walter 等人的论文中使用了不同的形式。不过推一推会发现二者是一样的，而且上面这个式子实现起来计算起来要更方便一些。
$$
D(m) = \frac{\alpha^2 \chi^+(n \cdot m)}{\pi (n \cdot m)^4 (\alpha^2 + \tan^2(\angle(n, m))^2}
$$
HLSL 实现：

```c
float GGXNDF(float NdotH, float a2) {
    return a2 / max(EPS, PI * sqr(NdotH * NdotH * (a2 - 1) + 1));
}
// ...
float NDF = GGXNDF(NdotH, roughnessSqr);
```

需要注意的是，该实现在光的背面也会是非 0 的值（也就是说没有实现 $\chi^+$）。

## GTR（Generalized Trowbridge-Reitz）

从名字上可以看出来，该 NDF 是 GGX/TR 的推广，目前比较流行的分布，其公式为：
$$
D(m) = \frac{c \chi^+(n \cdot m)}{(1 + (n \cdot m)^2 (\alpha^2 - 1))^{\gamma}}
$$
$c$ 是归一化系数（需要积分求解归一化系数，不过这个分布还挺好积的）。对于参数 $\gamma$：

* $\gamma = 1$，是一个在这之前已有的分布 Berry 分布，$c = (\alpha^2 - 1) / (\pi \ln(\alpha^2))$
* $\gamma = 2$，就是 GGX/TR 分布，$c = \alpha^2 / \pi$

随着 $\gamma$ 的增大，高光尾部会变长。

这里给出 GTR1 和 GTR2 的实现：

```c
float GTR2NDF(float NdotH, float a2) {
    float c = a2 / PI;
    return c / max(EPS, sqr(NdotH * NdotH * (a2 - 1) + 1));
}

float GTR1NDF(float NdotH, float a2) {
    float temp = a2 - 1;
    float c = temp / PI / log(a2);
    return c / max(EPS, 1 + NdotH * NdotH * temp);
}
```

## 形状不变性（shape-invariant）与各向异性 NDF

如果一个 NDF 可以被重写为一下形式，则称其为具有形状不变性（shape-invariant）：
$$
D(m) = \frac{1}{\alpha^2 (n \cdot m)^4} g\left( \frac{\sqrt{1 - (n \cdot m)^2}}{\alpha (n \cdot m)} \right)
$$
以上提到的各个分布中，GGX/TR 与 Beckmann 分布具有形状不变性，其余几个则没有。

具有形状不变性的 NDF 在 $\alpha$ 变化时，相当于微平面拉伸或收缩。

具有形状不变性的 NDF 可以方便的得到其各向异性的版本，也方便推导对应的几何项。一个具有形状不变性的 NDF 的相应的各向异性的版本为：
$$
D = \frac{1}{\alpha_x \alpha_y (n \cdot m)^4} g\left( \frac{\sqrt{(t \cdot m)^2 / \alpha_x^2 + (b \cdot m)^2 / \alpha_y^2}}{(n \cdot m)} \right)
$$
其中 $\alpha_x$ 与 $\alpha_y$ 是两个方向上的粗糙度（X 指切线方向，Y 指副切线方向），相当于两个垂直的方向上是同一分布但是是不同的拉伸因子。当 $\alpha_x = \alpha_y$ 时，退化为各向同性（注意到 $(t \cdot m)^2 + (b \cdot m)^2 + (n \cdot m)^2 = 1$）。

各向异性的 Beckmann 分布：
$$
D = \frac{1}{\pi \alpha_x \alpha_y (n \cdot m)^4} \exp(- \frac{(t \cdot m)^2 / \alpha_x^2 + (b \cdot m)^2 / \alpha_y^2}{(n \cdot m)^2})
$$
各向异性的 GGX/TR 分布：
$$
D = \frac{1}{\pi \alpha_x \alpha_y ((t \cdot m)^2 / \alpha_x^2 + (b \cdot m)^2 / \alpha_y^2 + (n \cdot m)^2)^2}
$$
为了描述各向异性平面，可以直接使用两个粗糙度参数，或者使用一个粗糙度参数与一个各向异性参数 $k_{\mathrm{aniso}}$，由其得到两个粗糙度的方法是（Disney）：
$$
k_{\mathrm{aspect}} = \sqrt{1 - 0.9 k_{\mathrm{aniso}}} \\
\alpha_x = \frac{\alpha}{k_{\mathrm{aspect}}} \\
\alpha_y = \alpha k_{\mathrm{aspect}}
$$
两个方向的粗糙度比值范围为 $[1, 10]$。或（Sony Imageworks）：
$$
\alpha_x = \alpha (1 + k_{\mathrm{aniso}}) \\
\alpha_y = \alpha (1 - k_{\mathrm{aniso}}) \\
$$
两个方向的粗糙度比值范围为 $[1, \infin]$。

（《Real Time Rendering, 4th》中（毛神的 PBR 白皮书是基于其的）中给出的式子使用 $r^2$，但我以为写成 $\alpha$ 更能直接地说明退化的状况）

Stevens 的教程中使用上者的公式，但扩大了 $k_{\mathrm{aniso}}$ 的范围为 $[-20, 1]$，但我以为如果要扩大也应该是扩大到 $[-10, 1]$。当范围为 $[-10, 1]$ 时，两个方向的粗糙度比值范围扩大为 $[0.1, 10]$。不知道 Stevens 的教程选择 $-20$ 是何用意。 

GGX/TR 分布与 Beckmann 分布相应的各向异性分布的 HLSL 实现：

```c
float BeckmannAnisoNDF(float NdotH, float HdotX, float HdotY, float ax, float ay) {
    float NdotHSqr = NdotH * NdotH;
    return exp(-(sqr(HdotX / ax) + sqr(HdotY / ay)) / NdotHSqr)
        / max(EPS, PI * ax * ay * NdotHSqr * NdotHSqr);
}
float GGXAnisoNDF(float NdotH, float HdotX, float HdotY, float ax, float ay) {
    return 1.0 / max(EPS, PI * ax * ay * sqr(sqr(HdotX / ax) + sqr(HdotY / ay) + sqr(NdotH)));
}
// ...
float kaspect = sqrt(1 - 0.9 * _Anisotropic);
float roughnessX = max(EPS, roughness / kaspect);
float roughnessY = max(EPS, roughness * kaspect);
float HdotX = dot(halfDir, i.worldTangent);
float HdotY = dot(halfDir, i.worldBitangent);
// float NDF = BeckmannAnisoNDF(NdotH, HdotX, HdotY, roughnessX, roughnessY);
float NDF = GGXAnisoNDF(NdotH, HdotX, HdotY, roughnessX, roughnessY);
```

## 可视化结果

让片段着色器以灰色直接输出 NDF 的值，结果如下：

![02-ndf.png](https://i.loli.net/2021/02/14/NAbdczUR2a6kjwF.png)