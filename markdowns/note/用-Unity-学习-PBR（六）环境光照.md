---
title: 用 Unity 学习 PBR（六）环境光照
date: 2021-02-15 09:39:22
tags: [Unity, CG, PBR, 学习笔记]
categories: 学习笔记（CG）
---

本文描述了环境光照中的漫反射项与镜面反射项的计算方法，然后给出了在 Unity ShaderLab 中使用 Unity 提供的全局光照工具得到环境光照结果的代码，最后给出了一路下来实现的各个部分结合起来后的效果并与 Unity 自己的 PBR 结果做了一个大致的对比。

参考

* [基于物理的渲染（PBR）白皮书 - 毛星云](https://github.com/QianMo/PBR-White-Paper)
* [Physically Based Rendering Algorithms: A Comprehensive Study In Unity3D](https://www.jordanstevenstechart.com/physically-based-rendering)
* [Learn OpenGL - PBR/IBL](https://learnopengl.com/PBR/IBL/Diffuse-irradiance)

<!-- more -->

环境光照使用基于图像光照（Image Based Lighing，IBL）的技术，图像即立方体贴图。

环境光照也分为漫反射项与镜面反射项。

## 环境光照的漫反射项

漫反射项使用辉度环境映射（Irradiance Environment Mapping）的技术。

认为漫反射项是 Lambert 的话，这一部分的积分是：
$$
\frac{\rho_{\mathrm{ss}}}{\pi} k_{\mathrm{diffuse}} \int_{\Omega} L(l) (l \cdot n) \mathrm{d}l
$$
积分部分只有法向有关，我们可以提前计算得到这一部分，把计算好的结果存入一个立方体贴图之中。

## 环境光照的镜面反射项

镜面反射项使用分解求和近似，将积分近似拆解为相对方便计算的两项：
$$
\int_{\Omega} f(l, r) L(l) (l \cdot n) \mathrm{d} l \approx \int_{\Omega} L(l) \mathrm{d} l \cdot \int_{\Omega}   f(l, v) (l \cdot n) \mathrm{d} l
$$
第一项是光亮度的平均，取决于表面粗糙度与反射光线。可以这样感性地理解与粗糙度的关系：当平面完全光滑时，只有观察光线的反射光线方向的光有贡献，随着粗糙度的增加，贡献主要来自于观察光线的反射光线方向为中心的一个锥体的范围，且锥体范围越来越大，这个锥体样子的东西讲得专业一点就是镜面波瓣（specular lobe）。

对于不同的粗糙度，我们渲染到不同的 mipmap 级上，使用时根据粗糙度在相应的 mipmap 级上使用观察光线的反射光线进行采样。

在对某一个粗糙度进行处理时，我们根据 NDF 随机微平面法向，假定观察光线与宏平面法向平行，以此得到光源方向并从立方体贴图上采样得到 $L(l)$。处理时可以使用重要性采样与低差序列。

第二项与粗糙度、$F_0$、视角（$v \cdot n$）有关，发现在使用 Schlick Fresnel 近似时，可以把 $F_0$ 提出，得到 $F_0 \times A + B$ 形式的式子，而剩余的两个部分都仅与粗糙度与视角有关，可以用一张二维预处理的贴图（两维坐标分别为 $n \cdot v$ 与粗糙度）来存储和采样这两项的值。具体的推导如下：
$$
\int_{\Omega} f(l, v) (l \cdot n) \mathrm{d} l
$$
$$
= \int_{\Omega} D(m) V(l, v) F(v, m) (l \cdot n) \mathrm{d} l
$$
$$
= \int_{\Omega} D(m) V(l, v) (l \cdot n) (F_0 + (1 - F_0) (1 - (n \cdot v))^5) \mathrm{d} l
$$
$$
= F_0 \int_{\Omega} D(m) V(l, v) (l \cdot n) (1 - (1 - (n \cdot v))^5) \mathrm{d} l + \int_{\Omega} D(m) V(l, v) (l \cdot n) (1 - (n \cdot v))^5 \mathrm{d} l
$$
$$
= F_0 \times A + B
$$
在与计算这张二维材质时，片元的坐标就是 $n \cdot v$ 与粗糙度，我们随机生成 $m$，这样就能得到相应的 $l$，在 NDF 与 GSF 确定的情况下，$A$ 与 $B$ 的值就是确定的。把 $A$ 存在 R 通道中、$B$ 存在 G 通道中，就能得到一张经典的红色为主、左下角有一点绿色的贴图。

## 在 Unity 中获取环境光照信息

Unity 内置了一套获取环境光照信息的工具，Stevens 的教程中给出了使用这套工具的代码：

```c
UnityGI GetUnityGI(
        float3 lightColor,
        float3 lightDirection,
        float3 normalDirection,
        float3 viewDirection,
        float3 viewReflectDirection,
        float attenuation,
        float roughness,
        float3 worldPos
) {
    UnityLight light;
    light.color = lightColor;
    light.dir = lightDirection;
    light.ndotl = max(0.0h, dot(normalDirection, lightDirection));
    UnityGIInput d;
    d.light = light;
    d.worldPos = worldPos;
    d.worldViewDir = viewDirection;
    d.atten = attenuation;
    d.ambient = 0.0h;
    d.boxMax[0] = unity_SpecCube0_BoxMax;
    d.boxMin[0] = unity_SpecCube0_BoxMin;
    d.probePosition[0] = unity_SpecCube0_ProbePosition;
    d.probeHDR[0] = unity_SpecCube0_HDR;
    d.boxMax[1] = unity_SpecCube1_BoxMax;
    d.boxMin[1] = unity_SpecCube1_BoxMin;
    d.probePosition[1] = unity_SpecCube1_ProbePosition;
    d.probeHDR[1] = unity_SpecCube1_HDR;
    Unity_GlossyEnvironmentData ugls_en_data;
    ugls_en_data.roughness = roughness;
    ugls_en_data.reflUVW = viewReflectDirection;
    UnityGI gi = UnityGlobalIllumination(d, 1.0h, normalDirection, ugls_en_data);
    return gi;
}
```

其中 `unity_SpecCube` 是天空盒/反射探针的相关信息。返回的 `UnityGI` 包含了环境光照的漫反射项与镜面反射项：

```c
float3 indirectDiffuse = gi.indirect.diffuse.rgb;
float3 indirectSpecular = gi.indirect.specular.rgb;
```

与直接光照加起来得到最终的结果：

```c
float3 lighting = (diffuse + NDF * Visible * F) * attenColor * NdotL
    + indirectDiffuse + indirectSpecular;
```

## 整合结果与对比

NDF 使用 GGX/TR 分布、GSF 使用 GGX Smith 高度相关遮蔽阴影函数的近似，菲涅尔项使用 Schlick 近似，漫反射项使用 Disney 漫反射，环境光照使用 Unity 提供的，结果如下。其中左侧为 Unity 自己的 PBR（Unity 提供的参数的光泽度而不是粗糙度），右侧是自己实现的 PBR，可以看到区别还是相当的大的。可能原因除了粗糙度的转换关系不同以外，我在调节粗糙度使自己的 PBR 结果看起来和 Unity 的差不多时，发现自己很难达到像它那样那么亮的反光。

![06-gi.png](https://i.loli.net/2021/02/14/bGo1cedTukFltrp.png)