---
title: Unity 中的渲染 学习笔记（四）
date: 2021-03-04 16:18:32
tags: [CG, Unity, 学习笔记]
categories: 学习笔记（CG）
---

CatLikeCoding 上的诸多 Unity 系列教程中，关于 Unity 的渲染相关实现的教程的学习笔记，原教程用 20 节讲述了 Unity 自己的渲染相关的一些具体细节。本文是其中 13~17 节的学习笔记，此 5 节的标题依次为：

* 13 - Deferred Shading
* 14 - Fog
* 15 - Deferred Lighting
* 16 - Static Lighting
* 17 - Mixed Lighting

原教程及其他参考：

* [Unity Rendering Tutorial](https://catlikecoding.com/unity/tutorials/rendering/)
* [Unity-Build-in-Shaders](https://github.com/TwoTailsGames/Unity-Built-in-Shaders)

<!-- more -->

## 13 Deferred Shading

### 正向渲染与延迟渲染的绘制顺序差异

二者的绘制顺序大致是：

正向渲染

1. 从摄像机绘制深度
2. 阴影
3. 不透明物体的 Forward Base 与 Forward Add
4. 天空盒
5. 透明物体的 Forward Base 与 Forward Add
6. 后处理

延迟渲染

1. G-Buffer
2. 前向不透明物体深度
3. 处理深度
4. 反射
5. 光照
6. 阴影
7. 天空盒
8. 透明物体的 Forward Base 与 Forward Add
9. 后处理

### G-Buffer

使用 4 个输出

* ARGB32，RGB - 反照率，A - 遮蔽
* ARGB32，RGB - 镜面反射颜色，A - 光滑度
* ARGB2101010，RGB - 法线（值域移至 [0, 1] 的），A - 未使用
* ARGB2101010（LDR 摄像机）/ ARGBHalf（HDR 摄像机），RGB - 累积光照，一开始为环境光与自发光

（深度模板的那个输出还是会有的）

在 LDR 摄像机中，Unity 期望结果（G-Buffer 中的最后一项）使用对数编码的 LDR 以得到更好的动态范围，若原本的颜色为 C，则转换为 2^-C^（要手动转换，`UNITY_HDR_ON` 关键字表明了使用 HDR 摄像机，否则是 LDR 摄像机），Unity 会在最后转换回来。（但为什么这样会有更好的动态范围，大概查了查也没查到东西。。。）

### 延迟渲染与反射

在正向渲染中，反射探针相关的内容（使用哪个反射探针，混合系数是多少）是对每个物体计算的，而在延迟渲染中，因为在 G-Buffer 中我们并不知道一个像素来自哪一个物体，我们不可能像正向渲染那样使用反射探针。关于这个问题，有两种方案。

第一种解决方案是根本不使用延迟反射，在渲染 G-Buffer 的那个 pass 中，就直接把反射相关的内容绘制到 G-Buffer 的最后一项中。这样和正向渲染的结果是一样的。如果不使用这种做法而使用延迟反射的话，渲染 G-Buffer 的 pass 中，立方体贴图会是纯黑色。（原教程中在最后把 `indirectLight.specular` 赋为 0，应该是让编译器发现之前的复制都白做了来消除掉多余的采样）

另一种方案是，在渲染 G-Buffer 的那个 pass 中不管非直接光照的反射项，使用单独的 pass 做延迟反射。对于天空盒的反射，绘制屏幕四边形做反射；对于反射探针，绘制反射探针的立方体盒，在盒内的像素使用该探针的数据绘制，后绘制的反射探针盒会覆盖之前绘制的。不过，在延迟反射中，反射探针会有一个叫“混合距离”的参数，会把盒向外扩展一定距离作为淡出混合的部分（该参数也会影响盒投影的结果），落于此范围内的像素可能会混合多于两个反射探针的结果。

（从 G-Buffer 中获取内容计算光照的部分在第 15 节）

## 14 Fog

### 不同种类的雾

`multi_compile_fog` 提供对应关键字的着色器变体（或没有表示没有开启雾）。

* 线性（`FOG_LINEAR`）：$f = (E - c) / (E - S)$（$S$ 和 $E$ 分别为衰减起始和衰减结束的位置，$c$ 为雾坐标（见后））
* 指数（`FOG_EXP`）：$f = 2^{-cd}$（$d$ 为雾密度，越大表现出雾越浓）
* 指数平方（`FOG_EXP2`）：$f = 2^{-(cd)^2}$

Unity 提供了 `UNITY_CALC_FOG_FACTOR_RAW(coord)` 来计算雾的系数。

```c
#if defined(FOG_LINEAR)
    // factor = (end-z)/(end-start) = z * (-1/(end-start)) + (end/(end-start))
    #define UNITY_CALC_FOG_FACTOR_RAW(coord) float unityFogFactor = (coord) * unity_FogParams.z + unity_FogParams.w
#elif defined(FOG_EXP)
    // factor = exp(-density*z)
    #define UNITY_CALC_FOG_FACTOR_RAW(coord) float unityFogFactor = unity_FogParams.y * (coord); unityFogFactor = exp2(-unityFogFactor)
#elif defined(FOG_EXP2)
    // factor = exp(-(density*z)^2)
    #define UNITY_CALC_FOG_FACTOR_RAW(coord) float unityFogFactor = unity_FogParams.x * (coord); unityFogFactor = exp2(-unityFogFactor*unityFogFactor)
#else
    #define UNITY_CALC_FOG_FACTOR_RAW(coord) float unityFogFactor = 0.0
#endif
```

用得到的系数在雾颜色或黑色（为了避免多个光源时雾颜色被重复计算，只在 Forward Base 中使用雾颜色，否则使用黑色）和计算出的光照颜色之间插值（要特殊处理没有开启雾的情况）。

（这个宏定义感觉非常的迷，没有开启雾的值和开启雾的值居然不能统一处理。。。）

### 雾坐标

雾坐标可以使用像素到摄像机的距离或像素的线性深度（？就是透视除法前的那个），Unity 使用后者，并使用宏 `UNITY_Z_0_FAR_FROM_CLIPSPACE` 来处理不同透视投影矩阵下不同的 z 的范围（主要是处理反 z）。

### 延迟渲染中的雾

要在延迟渲染中实现雾的效果，需要一个额外的后处理 pass，通过向摄像机添加一个 C# 脚本组件、实现 `OnRenderImage` 方法实现。同时需要一个单独的后处理着色器。

在延迟雾的后处理着色器中，从延迟渲染结果纹理（`_MainTex` 属性）中采样颜色，从摄像机深度图（`_CameraDepthTexture`，不需要定义在属性中）中采样深度并转化为线性深度，就可以像正向渲染中那样处理雾了。其中深度的转换，Unity 提供了函数 `Linear01Depth(depth)` 来实现，其中 `_ZBufferParams` 的 x 为 1 - far / near，y 为 far / near。

```c
// Z buffer to linear 0..1 depth
inline float Linear01Depth( float z )
{
    return 1.0 / (_ZBufferParams.x * z + _ZBufferParams.y);
}
```

把 01 线性深度转为正向渲染的那个线性深度时，原教程先后使用了直接乘上 far 和乘上 far 再减去 near，我个人觉得前者更有道理一些，把范围搞到了希望的 [0, far] 上，原教程之所以会有区别，猜测是其正向渲染的目标得到的深度范围是 [-near, far]，`UNITY_Z_0_FAR_FROM_CLIPSPACE` 认为此种情况应当修正但为了效率没有修。

按以上方式得到的雾会有一些问题，这些问题及其解决如下：

* 因为透明物体不写入深度，它们可能会被雾遮住 -> 为 `OnRenderImage` 添加 `ImageEffectOpaque` 属性，让其在绘制透明物体前绘制雾（不过这样透明物体上就没有雾了）
* 天空盒也被影响了 -> 当 01 深度足够大时，设置 `unityFogFactor` 为 1
* 关闭雾时显示为全雾 -> 用宏判断，在无雾时设置 `unityFogFactor` 为 1（所以说这个没有开启雾的值和开启雾的值居然不能统一处理的设定就好奇怪啊。。。）

## 15 Deferred Lights

### LDR 颜色后处理

如第 13 节中所说，LDR 摄像机会使用对数编码，我们需要一个额外的 pass 来转换回来。而 Unity 要求的延迟着色着色器需要有 2 个 pass，其中第二个 pass 就是处理这个用的。

为了减少运算，我们不需要在天空盒的位置转换，所以 Unity 在这之前对所有的物体又进行了一次绘制，只写模板，在做转换时只在模板匹配上的位置做。

可能需要在 ShaderLab 的 pass 中写入一些内容来搞模板相关的设置：

```
Pass
{
    Cull Off
    ZTest Always
    ZWrite Off
    Stencil
    {
        Ref [_StencilNonBackground]
        ReadMask [_StencilNonBackground]
        CompBack Equal
        CompFront Equal
    }
    
    ...
}
```

在转换时，需要用到已经绘制好的内容，在 `_LightBuffer` 2D 纹理中。

### 定向光

在定向光的延迟光照中，Unity 会输入给顶点着色器的值是 `POSITION` 与 `NORMAL`，前者就是屏幕四边形的四个点（xy 坐标为 0 或 1），后者是摄像机到四边形顶点的向量。

因为这样，我们需要自己搞出屏幕坐标，Unity 提供了 `ComputeScreenPos` 来从裁剪空间的坐标（xy 为 -1 或 1）转换到屏幕坐标，其中 `_ProjectionParams.x` 是 1 或 -1，来解决 uv 起点在左上角或左下角的区别。（原教程说这样得到的是齐次坐标，不过这个摄像机的投影矩阵（？）得到的 w 看起来是 1 的样子）

```c
inline float4 ComputeNonStereoScreenPos(float4 pos) {
    float4 o = pos * 0.5f;
    o.xy = float2(o.x, o.y*_ProjectionParams.x) + o.w;
    o.zw = pos.zw;
    return o;
}

inline float4 ComputeScreenPos(float4 pos) {
    float4 o = ComputeNonStereoScreenPos(pos);
#if defined(UNITY_SINGLE_PASS_STEREO)
    o.xy = TransformStereoScreenSpaceTex(o.xy, pos.w);
#endif
    return o;
}
```

之后就可以用得到的屏幕坐标在 G-Buffer（`_CameraGBufferTexture0` 到 `_CameraGBufferTexture2`，最后一张应该不用采样，而是作为了后续的渲染的目标的样子）中采样了。 

为了计算光照，还需要从深度得到世界空间坐标，输入的 `NORMAL` 就是用来做这个的。我们在片段着色器的输入中得到了摄像机到屏幕四边形上正要绘制的像素的向量，将其延长到摄像机远平面，再乘上 01 线性深度，就得到了观察坐标下的位置，之后乘上观察矩阵的逆矩阵就得到世界坐标的位置。

```c
float depth = SAMPLE_DEPTH_TEXTURE(_CameraDepthTexture, uv);
depth = Linear01Depth(depth);
float3 rayToFarPlane = fin.ray * _ProjectionParams.z / fin.ray.z;
float3 viewPos = rayToFarPlane * depth;
float3 worldPos = mul(unity_CameraToWorld, float4(viewPos, 1)).xyz;
float3 viewDir = normalize(_WorldSpaceCameraPos.xyz - worldPos);
```

`UNITY_BRDF_PBS` 的参数中，我们需要的反照率、反射光着色、光滑度、法向已经从 G-Buffer 中读出来了，视线方向刚刚也算了出来，还需要 1 减反射率、直接光信息和间接光信息。

1 减反射率，我们之前是通过金属度和反照率得到的，但金属度并没有存在 G-Buffer 里。Unity 使用 `SpecularStrength(specular)` 来得到反射率，其返回反射光着色 RGB 中的最大值。这样得到的结果与正向渲染并不相同（比如纯金属本来反射率为 1，延迟渲染中却可能有一些漫反射项），不过可能是一个可以接受的取舍（相比于存储在 G-Buffer 里）。

直接光信息在变量 `_LightColor` 和 `_LightDir` 中，与正向渲染不同，`_LightDir` 这里真的是光的方向（`_WorldSpaceLightPos0.xyz` 是反向的光的方向，因为我们计算光照时需要的就是反向的光源方向），不知道为什么这样设计。

间接光在绘制 G-Buffer 的 pass 和延迟反射中绘制完了，这里不再需要。

定向光阴影比较好搞，因为是使用屏幕空间阴影贴图，直接从其中（`_ShadowMapTexture`）采样并作为衰减系数乘在直接光的颜色中即可。不过 Unity 做了一些额外的工作：在阴影范围的边界处，为了避免出现生硬的分界线，Unity 在边界处对阴影做了淡出的效果，如下（`unity_ShadowFadeCenterAndType.w` 是 0 或 1）：

```c
// in custom shader, CreateLight
shadowAtten = tex2D(_ShadowMapTexture, uv).r;
float shadowFadeDistance = UnityComputeShadowFadeDistance(worldPos, viewZ);
float shadowFade = UnityComputeShadowFade(shadowFadeDistance);
shadowAtten = saturate(shadowAtten + shadowFade);

// in Unity built-in shader
float UnityComputeShadowFadeDistance(float3 wpos, float z)
{
    float sphereDist = distance(wpos, unity_ShadowFadeCenterAndType.xyz);
    return lerp(z, sphereDist, unity_ShadowFadeCenterAndType.w);
}

half UnityComputeShadowFade(float fadeDist)
{
    return saturate(fadeDist * _LightShadowData.z + _LightShadowData.w);
}
```

为了支持光照蒙版（cookie），我们需要把点转换到光照坐标，采样 cookie 纹理（`_LightTexture0`）的 A 通道乘在得到的衰减项中。有人发现，当相邻像素的实际坐标相差太大时，会选一个较低的 mipmap 级，Unity 为了解决这个问题，使用了如下的技巧。

```c
#ifdef DIRECTIONAL_COOKIE
    float2 uvCookie = mul(unity_WorldToLight, float4(worldPos, 1)).xy;
    atten *= tex2Dbias(_LightTexture0, float4(uvCookie, 0, -8)).a;
#endif
```

为了支持 Unity 在 LDR 摄像机下的对数编码，我们首先修改输出为 $2^{-C}$，再把混合模式设为 `Blend [_SrcBlend] [_DstBlend]`，因为 LDR 下会使用与 HDR 不同的混合模式 `Blend DstColor Zero`，相当于把新旧结果相乘写入到目标里，因为 $2^{-(C_1 + C_2)} = 2^{-C_1} \times 2^{-C_2}$。

### 聚光

因为聚光影响的范围一般远没有整个屏幕那么大，Unity 绘制一个正四棱锥作为聚光的范围，开启小于等于的深度测试与背面剔除，在绘制到的像素上计算延迟光照。不过，这种做法存在一个问题，因为是剔除背面，当该正四棱锥与摄像机近平面相交时，会少绘制一些像素，解决方法是在此时做正面剔除和大于的深度测试。

聚光的衰减项包含两项，根据距离的衰减与光照 cookie，前者在 `_LightTextureB0` 中采样，后者在 `_LightTexture0` 中采样，就像在第 5 节说的那样。因为此时转换到光源空间的矩阵是透视的，所以要手动做透视除法。

聚光的阴影项直接使用会处理软硬阴影的 `UnitySampleShadowmap` 函数，应该在第 7 节中说过了。

### 点光

与聚光类似，Unity 绘制一个几何球作为点光的范围进行延迟光照。

衰减与阴影也依旧和第 5 节、第 7 节说的差不多。不过有一点是，第 5 节时给出的 Unity 着色器代码中，无 cookie 的点光的衰减贴图使用 `_LightTexture0`，有 cookie 的点光和聚光则使用 `_LightTextureB0`；但是在这里，发现二者都在使用 `_LightTextureB0`。

### 避免不必要的阴影采样

原教程的最后给出了这样的代码：

```c
if (shadowed) {
    float shadowFadeDistance = UnityComputeShadowFadeDistance(worldPos, viewZ);
    float shadowFade = UnityComputeShadowFade(shadowFadeDistance);
    shadowAtten = saturate(shadowAtten + shadowFade);
#if defined(UNITY_FAST_COHERENT_DYNAMIC_BRANCHING) && defined(SHADOWS_SOFT)
    UNITY_BRANCH
    if (shadowFade > 0.99) {
        shadowAtten = 1;
    }
#endif
}
```

也就是说，当因为阴影淡出的缘故，像素已经基本不表现为有阴影时，直接对阴影项置 1，让编译器优化掉阴影贴图的采样。原教程只对聚光和点光的软阴影做了处理，Unity 中还额外有一个判断 `!defined(LIGHTMAP_SHADOW_MIXING)`。

其中 `UNITY_FAST_COHERENT_DYNAMIC_BRANCHING` 的“coherent dynamic branching”是指，虽然分支条件的结果不是 uniform 的，但在相邻的像素之间存在相关性，这样的分支在一些硬件上也不会产生过大的效率损失。一般来说，如果一定要在着色器里出现分支，那代价从低到高分别是 uniform(static)、coherent(invariant) dynamic、incoherent(variant) dynamic。（[Shader中的 if 和分支 - 知乎](https://zhuanlan.zhihu.com/p/122467342)）

## 16 Static Lighting

### 光照贴图（Light Map）

当光源与物体都是静态时，可以使用预烘培的光照贴图进行光照。使用实时计算还是光照贴图的设置在光源上。

我在写这篇笔记时，使用的 Unity 2019.4 有三种 lightmapper：enlighten、progressive（CPU）和 progressive（GPU），原教程使用的 enlighten lightmapper 是基于预计算的实时 GI 信息来计算光照贴图的，已经被标注为弃用了，progressive lightmapper 则使用路径追踪的方式来计算光照贴图。

光照贴图有不错的间接光效果（物体的颜色会在相邻物体有一点表现），材质的自发光项也会如同光源般对其他物体着色，但没有镜面反射光（因为镜面反射光是与摄像机位置有关的，不能提前计算）。

光照贴图在正向渲染与延迟渲染中都可以使用。

### 在着色器中使用光照贴图

Unity 在 Forward Base 或 Deferred 这两个 pass 使用光照贴图，此时就不使用顶点光照，也不计算球谐光照。

采样光照贴图的 uv 由 Unity 以 `TEXCOORD1` 传给顶点着色器。

光照贴图存储在 2D 纹理 `unity_Lightmap` 中，以 RGBM（之前提过的指数编码方式）或 dLDR（Double LDR，就是把 HDR 除以 2 后当 LDR 存，解码就再把 2 乘回去）编码 HDR，因此采样结果需要用 `DecodeLightmap()` 解码。

### 适配 Unity 的 lightmapper

Unity 绘制光照贴图的代码中，假定了颜色属性的名称应当是 `_Color`，alpha cutoff 属性的名称应当是 `_Cutoff`，只有这样，半透明物体和截断物体才能在使用光照贴图时正确绘制。

### Meta Pass

在绘制光照贴图时，Unity 使用 Meta pass 来处理物体的颜色、自发光等的影响，这就是使用光照贴图时能从一个物体上看到相邻物体颜色影响的原因。在其中，我们要得到物体的反照率、自发光颜色和镜面反射颜色，该部分就有点像是 Forward Base pass 的一个子集，之后交给 Unity 的函数 `UnityMetaFragment()` 处理。但这个函数内部其实不管反射颜色，所以当表面是不发光的粗糙金属时，因为纯金属也没有漫反射，就几乎不表现出对相邻物体的着色，但实际上应该会有的。Unity 的 trick 是，把镜面反射颜色按粗糙度加在反照率上：

```c
float oneMinusReflectivity;
surfaceData.Albedo = DiffuseAndSpecularFromMetallic(GetAlbedo(fin), GetMetallic(fin),
    surfaceData.SpecularColor, oneMinusReflectivity);
float roughness = SmoothnessToRoughness(GetSmoothness(fin)) * 0.5;
surfaceData.Albedo += surfaceData.SpecularColor * roughness;
```

其中的 `SmoothnessToRoughness` 返回的是 1 减平滑度的平方，就像是我在 PBR 学习笔记中记录的那样，在着色器内部使用的粗糙度经常是外部输入粗糙度的平方，而不平方的粗糙度在 Unity 的着色器内被称作“感性的粗糙度（perceptual roughness）”。

### 方向性光照贴图（Directionality Light Map）

方向性光照贴图是另一张光照贴图（`unity_LightmapInd`），其存储的内容是接收光最大的一个方向，向量长度是方向性的大小，可以设置是否绘制该贴图。

无方向的光照贴图，相当于这么大的光从四面八方均匀地打在表面上，在使用法线贴图时，凹凸感会表现得非常弱。使用方向性光照贴图，相当于这么大的光从某一个方向打来，我们可以用和法向的点积来处理光照，法线贴图的效果会表现得正常一些。Unity 使用 `DecodeDirectionalLightmap()` 来处理，其内部计算半朗泊（$0.5 + 0.5 \mathrm{dot}(n, l)$）作为结果。

```c
inline half3 DecodeDirectionalLightmap (half3 color, fixed4 dirTex, half3 normalWorld)
{
    // In directional (non-specular) mode Enlighten bakes dominant light direction
    // in a way, that using it for half Lambert and then dividing by a "rebalancing coefficient"
    // gives a result close to plain diffuse response lightmaps, but normalmapped.

    // Note that dir is not unit length on purpose. Its length is "directionality", like
    // for the directional specular lightmaps.

    half halfLambert = dot(normalWorld, dirTex.xyz - 0.5) + 0.5;

    return color * halfLambert / max(1e-4h, dirTex.w);
}
```

### 光照探针

动态物体不会接收设置为使用光照贴图的光源，为了将其正常地融入到使用光照贴图地场景中，需要使用光照探针。

光照探针以球谐函数的形式存储该点的光照信息，一组光照探针把其内部的空间划分为若干个四面体，对于每个动态物体，插值其中心点所在的那个四面体的四个顶点的球谐函数信息（四面体也能重心坐标，应该就是用这个插值的；探针组凸包外的就是三角形（？），至少显示的是这样；因为球谐函数信息是对整个物体固定的，当动态物体较大时效果会不太好）传入着色器，使用球谐光照完成着色（已经在第 5 节中介绍过了/加过这个功能了，所以不用增加任何代码）。

不过这样动态物体依然不会产生阴影（不会遮蔽静态物体），也不会接收阴影（静态物体不投射实时阴影，如果能看到一点阴影的效果，是插值光照探针出来的）。

## 17 Mixed Lighting

### 混合光照

光源除了纯实时与使用光照贴图以外，还可以设置为混合。在此模式下，光照贴图依然会被生成，但不包含直接光。在渲染时，光源会像实时光源一样被计算。这么一来，相较于纯实时光照，添加了物体之间反射的间接光；相比纯光照贴图，添加了镜面反射光，允许动态物体，且动态物体与静态物体相互的阴影也能正常。不过付出了额外的纹理与采样、光照探针等代价。

需要注意的是，Unity 为了性能考虑，在混合模式下，`UNITY_LIGHT_ATTENUATION()` 并不计算阴影淡出，`HANDLE_SHADOWS_BLENDING_IN_GI` 宏的定义与否标识了这一点，我们需要在自己的着色器里手动处理，就像在延迟光照中那样。

### 阴影遮罩（Shadowmask）

为了减小混合光照昂贵的代价，可以把静态物体的阴影也烘培下来，设置是把混合光照的模式从默认的“baked indirect”切换为“shadowmask”，此时阴影信息会被单独烘培到一张贴图中（`unity_ShadowMask`），使用 R 通道保存，其中 0 表示完全被遮蔽。事实上，另外 3 个通道也能被用来分别保存阴影信息，光照体积不相交的光源可以使用同一通道，所以可以支持许多光源，在着色器中用 `unity_OcclusionMaskSelector` 与采样结果点积取得对应的分量；当光源实在是没有空闲的通道时，会把直接光烘培在光照贴图上，就像是纯光照贴图时那样。

在质量设置中，Unity 提供了 2 种选项：distance shadowmask 与 shadowmask，前者的代价比普通的混合光照还要大，它不仅烘培静态物体的阴影，也绘制静态物体的实时阴影，而后者并不绘制静态物体的实时阴影，动态物体只能通过光照探针得到静态物体的阴影。

要在延迟渲染中使用 shadowmask，需要一个额外的 G-Buffer 来存储从中采样的结果。

`unity_OcclusionMaskSelector` 总是至多有一个分量为 1，因此在延迟渲染中渲染出不太对劲的效果，一个本来接收了多个光源阴影的物体在渲染 shadowmask G-Buffer 时只渲染了一个光源的阴影上去（正向渲染会渲染多次，所以没事）。

### 减性阴影（Subtractive Shadows）

减性阴影/减性光照面向低性能设备，只支持正向渲染，只支持单一定向光。

静态物体使用光照贴图着色，同时根据实时光照的屏幕空间阴影对阴影区域做衰减，使得动态物体可以在静态物体上留下阴影。动态物体使用光照探针和实时光照，不能直接接收静态物体的阴影。

减性光照没有自己的关键字，其开启时，`LIGHTMAP_ON`、`SHADOWS_SCREEN`、`LIGHTMAP_SHADOW_MIXING` 开启而 `SHADOWS_SHADOWMASK` 关闭。

在减性光照下，关闭直接光（因为烘培在光照贴图里了）；衰减间接漫反射光，在通过 `UNITY_LIGHT_ATTENUATION` 和淡出阴影的计算后得到一个衰减度 `atten`，如果光照使用朗泊，那么光照是

```c
atten * ndotl * _LightColor0.rgb
```

我们是减去阴影，那就是减去

```c
(1 - atten) * ndotl * _LightColor0.rgb
```

其他要考虑的是，对环境光（`unity_ShadowColor`，减性阴影设置的阴影颜色）取 max、处理光源的阴影强度（用 `_LightShadowData.x` 插值）、对原来计算出来的间接漫反射光取 min（不能照亮烘培出来的阴影）。