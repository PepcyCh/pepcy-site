---
title: 用 Unity 学习 PBR（四）菲涅尔项
date: 2021-02-15 09:36:07
tags: [Unity, CG, PBR, 学习笔记]
categories: 学习笔记（CG）
---

本文大致介绍了菲涅尔函数（Fresnel Function）的基本性质、金属与非金属在菲涅尔现象上的差异，然后给出了 Schlick 近似公式及其代码实现。

参考

* [基于物理的渲染（PBR）白皮书 - 毛星云](https://github.com/QianMo/PBR-White-Paper)
* [Physically Based Rendering Algorithms: A Comprehensive Study In Unity3D](https://www.jordanstevenstechart.com/physically-based-rendering)

<!-- more -->

---

菲涅尔函数（Fresnel function）用于描述入射光中反射比例随光线与法线夹角的变化。当我们观察湖面时，近处的湖面可以看到湖面下的内容，而远处的湖面则呈现出明显的镜面反射的效果，菲涅尔函数描述的就是这一现象。

最常用的公式是 Schlick 的近似公式：
$$
F(l, m) = F_0 + (1 - F_0) (1 - (v \cdot m))^5
$$
其中 $F_0$ 为出射角（观察光线与法线的夹角）为 0，即正对着平面观察时的反射光比例。随着出射角增大，一开始反射光比例变化不大，而后快速增大到 1。

金属的 $F_0$ 一般 RGB 三个通道的值不相同，表现为金属自身的颜色，一般数值上超过 0.5x；非金属的 $F_0$ 则拥有相同的三个通道，且数值上一般不超过 0.1x，大部分非金属的 $F_0$ 一般为 0.0x，一般是钻石之类的非金属有着 0.1x 的 $F_0$；半导体的数值介于二者之间。

有些地方能看到使用的是 $F(l, n)$（宏平面法向）而不是 $F(l, m)$（微平面法向），我以为使用微平面法向更有道理一些，因为我们在讨论的对象是微平面而不是宏平面。

Schlick 近似的 HLSL 实现：

```c
float pow5(float x) {
    return x * x * x * x * x;
}
float3 SchlickFresnel(float3 f0, float VdotH) {
    return f0 + (1 - f0) * pow5(1 - VdotH);
}
```

