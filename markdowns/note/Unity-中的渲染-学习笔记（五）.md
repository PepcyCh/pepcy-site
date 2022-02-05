---
title: Unity 中的渲染 学习笔记（五）（完）
date: 2021-03-06 10:26:50
tags: [CG, Unity, 学习笔记]
categories: 学习笔记（CG）
---

CatLikeCoding 上的诸多 Unity 系列教程中，关于 Unity 的渲染相关实现的教程的学习笔记，原教程用 20 节讲述了 Unity 自己的渲染相关的一些具体细节。本文是其中 18~20 节，也就是最后 3 节的学习笔记，此 3 节的标题依次为：

* 18 - Realtime GI, Probe Volumes, LOD Groups
* 19 - GPU Instancing
* 20 - Parallax

原教程及其他参考：

* [Unity Rendering Tutorial](https://catlikecoding.com/unity/tutorials/rendering/)
* [Unity-Build-in-Shaders](https://github.com/TwoTailsGames/Unity-Built-in-Shaders)

<!-- more -->

## 18 Realtime GI, Probe Volumes, LOD Groups

### 使用 Enlighten 的实时 GI

Unity 标注了 Enlighten lightmapper 与使用 Enlighten 的实时 GI 为弃用，但去看了一圈一副是因为商业因素而非技术因素弃用的感觉，人儿 Enlighten 自个更新得好好着呢。。。不过我写这篇笔记时的 Unity 2019.4（虽然稳定版有 2020.2 但习惯于用 LTS 版的，所以就是 2019.4 了）并没有其他的实时 GI 方式，所以还是记录一下。关于弃用的博文说是 2021 版会完全移除 Enlighten 同时加上自己的实时 GI，不过目前 2021 版的还没有稳定版，文档里也没看到。。。

实时 GI 的光照贴图（`unity_DynamicLightmap` 与 `unity_DynamicDirectionality`）使用另一套 uv（与非实时的光照贴图的 uv 不同），通过 `TEXCOORD2` 传入。而且不像非实时光照贴图与顶点光照互斥，实时光照贴图是可以与之叠加的。

### 光照探针代理体（LPPV，Light Probe Proxy Volume）

之前提到过，因为动态物体的光照探针插值系数是以其中心决定的，当物体太大太长时效果就会很不好，LPPV 则用来解决这个问题。在 Unity 中为动态物体添加 LPPV 组件，可以看到一些点，可以设置这些点的位置、密度，Unity 会在每个点上预先插值出球谐函数的值，并把这些插值结果存储到一个三维纹理中（这些点分布在包围盒中，三维纹理的坐标应该就是包围盒内的一个位置相关的值），在片段着色器中使用 `SHEvalLinearL0L1_SampleProbeVolume()` 函数采样计算，该函数名中的“L0L1”指的是球谐函数的前两个条带。

要注意的一点是，`UNITY_LIGHT_PROBE_PROXY_VOLUME` 关键字是全局开启或关闭的，需要由 `unity_ProbeVolumeParams.x` 是否为 1 来判断物体是否使用 LPPV（奇怪的设计增加了）。

### LOD 组

在烘培光照贴图时，LOD 组会用 LOD 0 来烘培；在渲染时，非 LOD 0 的静态 LOD 组依然会使用光照探针。

为了避免切换 LOD 时突兀的变换，Unity 提供了两种过渡模式：Cross Fade 与 Speed Tree，原教程只讲了前者。

Cross Fade 就是使用抖动图（在半透明物体的阴影提过的那个）来混合两个相邻的 LOD 级，Unity 提供了 `UnityApplyDitherCrossFade()` 来进行处理。Unity 标准着色器并不支持该功能。

```c
void UnityApplyDitherCrossFade(float2 vpos)
{
    vpos /= 4; // the dither mask texture is 4x4
    float mask = tex2D(unity_DitherMask, vpos).a;
    float sgn = unity_LODFade.x > 0 ? 1.0f : -1.0f;
    clip(unity_LODFade.x - mask * sgn);
}
```

## 19 GPU Instancing

GPU 实例化由关键字 `INSTANCING_ON` 控制，实例 ID 由语义 `SV_InstanceID` 传入。

在开启实例化时，Unity 会用把 `unity_ObjectToWorld` 变量重复定义为一个从数组中取出正确矩阵的宏，以让其他部分能在不修改代码的情况下正确工作。

Unity 不支持在 Forward Add 中实例化（Forward Base、Shadow Caster、Deferred 中是支持的），当在正向渲染中有多个光源时，被额外的光影响到的物体单独绘制，只受主光影响的物体采用实例化。

除了物体模型矩阵及其逆矩阵外，其他的每实例属性要自己按需增加，由 `UNITY_INSTANCING_BUFFER_START()` 与 `UNITY_INSTANCING_BUFFER_END(arr)` 括起（不知道哪个版本后，Unity 把原教程给出的宏的名字里去掉了个“C”，并要求在结尾处给一个名字。。。）。在其中定义属性使用的宏 `UNITY_DEFINE_INSTANCED_PROP(type, name)` 视情况替换为 `type name;` 或 `static type name;`。此外，属性的设置在 C# 中得通过属性块的方式设定（直接设定不会使用实例化，但我把属性块对象的定义放进那个 for 循环依然能实例化，不知道是编译原因还是版本原因）。

一般实例化时一组物体的个数主要受限于每个 buffer 的大小限制（限制是对单个 buffer 算的，不是对总和算的，虽然 buffer 的个数确实也有限制）。

原教程提到的 `#pragma instancing_options lodfade`，我这里无论是否写上这一行，都会使用实例化，可能是版本原因。

## 20 Parallax

### 视差贴图（Parallax Map）

视差贴图能提供比法线贴图更高的凹凸性，其大致的原理是，根据切线空间下的视线方向和纹理采样结果对 uv 进行偏移。教程中在顶点着色器中计算出切向空间视线方向，插值之后在片段着色器中做单位化，这样的结果和全部在片段着色器中计算相同，同时减少了一次片段着色器中矩阵乘法，但增加了一个着色器间的插值变量。

在一个平面上，记高度为 0，视线原来看到的位置是 $(u, v, 0)$（切线空间），切线空间视线方向是单位向量 $(x, y, z)$，当整个平面上升 $h$ 个单位高度后，同样的视线现在与平面的交点则是 $(u + xh/z, v + yh/z, h)$。当平面的高度各处不同时，使用一个纹理来表示高度（Unity 使用其 G 通道），使用 $(u, v)$ 处的高度来近似 $(u + xh/z, v + yh/z)$ 处的高度（毕竟只有偏移后我们才知道真正的高度是多少，而偏移又需要高度来偏移，就只好这样了），以此来做 uv 的偏移。下图是一个高度为 1 的示意图。

![](https://catlikecoding.com/unity/tutorials/rendering/part-20/parallax-mapping/raycasting.png)

我们可以用一个常数来控制视差贴图的强度，乘在 uv 的偏移量上。

Unity 还额外做了几件事：

* 让采样出来的高度 -0.5。原来我们只是升高平面，让中间高度以上的升高，中间高度以下的降低
* 对偏移量中的除法做了一些操作，它除以的是 z 加上 0.42

### 光线步进（Raymarching）

（Unity 不使用这个）

刚刚提到过，我们使用的高度是近似的，现在我们使用光线步进的方式来得到一个更精确的高度。

我们先偏移高度为 1 单位（纹理上可能的最大的高度）对应的偏移量，然后用较小的步长一步步地走回去，直到我们走到了表面下方（一开始的高度 1 必然不低于表面高度）。这样会得到比单纯的一次偏移质量更好、视觉效果更陡的高度差，但代价很大。

此种做法得到的结果，表面就好像变成了若干层，是因为我们把结果的高度约束在了几个定值上，比如设步数为 10，那步长就是 0.1，最终的高度就只会是 0、0.1、0.2、……、0.9、1.0 这样（根据实现，上界 1.0 可能取不到），这样结果就是 10 层。

为了解决分层的问题，除增大步数以外，原教程还提供了两种做法。一是在结果的相邻两步间插值，假设这两不之间的表面是线性的，利用相似三角形计算出插值系数（如下图虚线所示）。这样会得到不分层的逐段线性的起伏。

![](https://catlikecoding.com/unity/tutorials/rendering/part-20/raymarching/line-line-intersection.png)

另一种做法是，在结果的相邻两步间做二分。每增加一次二分，结果分层的可能的层数就会乘 2，相比增大步数大大节省了代价，但比线性插值的代价要大，这样得到的结果会更贴近原纹理，尤其是当起伏是圆滑的形状时。

最后，原教程指出，当 Unity 进行动态批处理打包几何体时，为了效率，不会单位化传给顶点着色器的法线与切线，当批次里存在缩放过的几何体时，就会得到错误的法线与切线。之前我们不在顶点着色器中使用它们，而在片段着色器中都对其进行了单位化，所以没出事。但这里我们在顶点着色器中求切线空间视线方向，就会得到错误的结果。Unity 标准着色器不对其进行修证（毕竟这种情况还挺少见的）。