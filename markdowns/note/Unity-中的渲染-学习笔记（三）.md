---
title: Unity 中的渲染 学习笔记（三）
date: 2021-02-28 14:23:09
tags: [CG, Unity, 学习笔记]
categories: 学习笔记（CG）
---

CatLikeCoding 上的诸多 Unity 系列教程中，关于 Unity 的渲染相关实现的教程的学习笔记，原教程用 20 节讲述了 Unity 自己的渲染相关的一些具体细节。本文是其中 9~12 节的学习笔记，此 4 节的标题依次为：

* 9 - Complex Material
* 10 - More Complexity
* 11 - Transparency
* 12 -Semitransparent Shadows

原教程及其他参考：

* [Unity Rendering Tutorial](https://catlikecoding.com/unity/tutorials/rendering/)
* [Unity-Build-in-Shaders](https://github.com/TwoTailsGames/Unity-Built-in-Shaders)

<!-- more -->

## 9 Complex Material

### 金属度贴图与光滑度贴图

* 金属度 - 要么只使用贴图（R），要么只使用单一数值
* 光滑度 - 要么只使用贴图（A）乘系数，要么只使用单一数值

（以上我觉得只是一种设计方式，就像主颜色是把纹理和数值乘起来而不是要么纯色要么贴图）

光滑度贴图的数据来源

* 与金属度合并，金属度在 R，光滑度在 A（DXT5 压缩时会分别处理 RGB 与 A）
* 与主材质合并，反照率在 RGB，光滑度在 A
* 定值

当使用混合贴图时，使用同一个 uv 采样多次（比如金属度一次，光滑度一次），会被编译优化为单次采样。

### 自发光（emissions）

直接把自发光的数值加到算出的最终颜色上，只在 Forward Base 中加一次。

材质的自发光不会直接影响其他物体，因为毕竟不是光源，但可以影响到 GI。

Unity 标准着色器的自发光项使用了 HDR。

原教程中的 `ColorPickerHDRConfig` 已弃用，函数 `TexturePropertyWithHDRColor` 有无该参数的版本。

### 自定义着色器变体的关键字

以上几个贴图，为了在不使用贴图时不进行无用的采样，可以使用自定义关键字和宏控制编译实现。光滑度来源也可以使用自定义关键字来标明。

### 着色器变体的 `shader_feature`

`shader_feature` 与 `multi_compile` 的区别在于，前者只会生成用到的关键字的组合，后者会生成全部的关键字的组合。当运行时会改变关键字时，使用前者可能会出问题。

## 10 - More Complexity

### 遮蔽（occlusion）

遮蔽有一个纹理（G）和一个控制纹理影响强度的系数。

原教程认为，遮蔽项只应该影响非直接光照（所谓“环境光遮蔽”就应该只影响环境光的理论吗。。。），并指出 Unity 也是这么做的（原教程说老版本的 Unity 是都遮蔽的，意思应该就是现在的 Unity 不是吧。。）。

可以和金属度（R）、光滑度（A）合并为同一张纹理。如果想要在着色器中只采样一次，需要手动实现（因为是两个不同的变量，只是正好赋了相同的值，编译器没法优化；或者像光滑度那样实现，但这样就不能用单独的贴图）

### 细节蒙版（detail mask）

对于细节纹理（细节反照率与细节法线），提供一个纹理来控制是否使用细节以及受细节多少影响，Unity 使用细节蒙版（细节遮罩）纹理的 A 通道控制。

反照率直接在无细节的结果和有细节的结果直接插值即可。

法线的话，原教程是在 (0, 0, 1) 与细节法线之间插值后与无细节法线混合，但实际上，直接插值无细节法线和有细节法线的结果是一样的。

## 11 Transparency

### Alpha Cutoff

当 alpha 低于一定值时舍弃片段。

像素的 alpha 来自反照率纹理的 A（如果光滑度不在这里的话）与主颜色的 A 之乘积。

### 渲染队列

每一个渲染队列对应一个数值，数值更低的更先绘制（就像 DX12 龙书里说的那样）。

默认的渲染队列有（按数值从小到大）：

* Background
* Geometry（不透明物体）
* AlphaTest（使用 alpha cutoff 的物体）
* GeometryLast（最后一个被认为是不透明的队列。队列值在这及之前的物体认为是不透明，排序使得管线状态改变、overdraw 尽可能少；之后的物体认为是透明，从后到前排序）
* Transparency
* Overlay

AlphaTest 在 Geometry 之后的原因是，舍弃片段的代价比较大，所以尽可能不绘制被遮挡的 AlphaTest 的物体。

### 半透明物体与 Fade 模式

半透明物体在 Transparency 渲染队列，使用 alpha 进行混合（Forward Base 中混合为 [SrcAlpha, OneMinusSrcAlpha]，Forward Add 中为 [SrcAlpha, One]），不写入深度。Unity 称这样的渲染方式为 fade。

半透明物体的渲染次序在天空盒之后。按从后到前排序，距离是算物体的中心点到相机的距离。

根据不同的渲染方式控制混合模式与是否写入深度，是通过在着色器中定义属性，在 C# 中用 `SetInt()` 实现的。ShaderLab 中的 `Blend XXX XXX` 和 `ZWrite XXX` 也是能用属性的。

### 半透明物体与 Transparent 模式

与 fade 模式的区别是，源混合模式从 SrcAlpha 变为 One。

其实现的效果是，类似玻璃等物体，在透过光的同时，自己也会有镜面反射光，fade 模式会同时根据 alpha 衰减镜面反射光，我们把 SrcAlpha 设为 One 来取消这个衰减，但相应的，我们要手动衰减漫反射项。具体做法是，把经过光滑度与金属度处理过的反照率乘上 alpha，同时修改 alpha 来满足能量守恒。记 alpha 为 $a$，镜面反射的比例为 $r$，则 $r$ 的比例表现为镜面反射、$a(1 - r)$ 的比例表现为漫反射（我们刚刚已经乘上 alpha 了）、剩余 $(1 - a)(1 - r)$ 的比例表现为穿透，即混合时另一侧的系数。因为另一侧使用 OneMinusSrcAlpha，所以我们把 alpha 修改为 $1 - (1 - a)(1 - r)$：

```c
    albedo = DiffuseAndSpecularFromMetallic(albedo, GetMetallic(fin), specColor, oneMinusReflectivity);
#ifdef _RENDERING_TRANSPARENT
    albedo *= alpha;
    alpha = 1 - oneMinusReflectivity + alpha * oneMinusReflectivity;
#endif
```

## 12 Semitransparent Shadows

### Alpha Cutoff 物体的阴影

在阴影投射的片段着色器中使用 alpha 与 clip。

### 半透明物体的阴影

在半透明的位置，其投射出的阴影会呈现出较淡的阴影，且深度和 alpha 有关，但我们在阴影投射时只能控制写入的深度，在阴影接受时又看不到源物体的 alpha（虽然确实可以再写到另一张纹理上）。Unity 使用抖动（dithering）来控制阴影的深浅，就像是一些报纸或其他印刷品上，使用黑点的密集程度来表现出不同程度的灰色，Unity 使用一张有 16 个等级抖动图来实现（是一张 3D 纹理，z 方向为 alpha 相关的值，0 代表 alpha 为 0 的第一档、0.0625 代表 alpha 的第二档、0.9375 代表 alpha 为 1 的最后一档）。

![](https://catlikecoding.com/unity/tutorials/rendering/part-12/partial-shadows/dither-patterns.png)

（从左到右，4x4 为一档，共 16 档）

在阴影投射中，在抖动图中采样，xy 使用缩放后的屏幕坐标（Unity 选择与 0.25 相乘），z 使用 alpha 乘上 0.9375，得到的结果做 clip。当 alpha 较低时，抖动图中大部分为白，会被 clip 掉；alpha 较高时，抖动图中的白色较少，clip 掉的部分就少，以此实现了与 alpha 相关的半透明物体阴影。

在片段着色器中获取屏幕坐标时，使用 `VPOS` 语义，其值是 0~1 的屏幕坐标。

模糊后的半透明阴影容易出现 shadow swimming 的现象。

因为这样的半透明物体的阴影的代价比较大（因为有大量的 clip 调用），所以可以用关键字控制其开关，当关闭时，使用 alpha cutoff 的阴影。