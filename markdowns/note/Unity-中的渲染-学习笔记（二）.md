---
title: Unity 中的渲染 学习笔记（二）
date: 2021-02-26 21:15:04
tags: [CG, Unity, 学习笔记]
categories: 学习笔记（CG）
---

CatLikeCoding 上的诸多 Unity 系列教程中，关于 Unity 的渲染相关实现的教程的学习笔记，原教程用 20 节讲述了 Unity 自己的渲染相关的一些具体细节。本文是其中 6~8 节的学习笔记，此 3 节的标题依次为：

* 6 - Bumpiness
* 7 - Shadows
* 8 - Reflection

原教程及其他参考：

* [Unity Rendering Tutorial](https://catlikecoding.com/unity/tutorials/rendering/)
* [Unity-Build-in-Shaders](https://github.com/TwoTailsGames/Unity-Built-in-Shaders)

<!-- more -->

## 6 Bumpiness

### 高度贴图（height map）

用灰度表示高度，通过高度得到切线空间下的法向。

`_TexelSize` 的 xy 为两个方向上一个像素对应的 uv 坐标的差，zw 为纹理的大小。即，如果纹理的大小为 $(w, h)$，那么该向量的值为 $(1 / w, 1 / h, w, h)$。

要从高度图得到法向，首先从 x 方向得到 u 的导数，即切线方向：
$$
f'(u) = \lim_{\delta \to 0} \frac{f(x + \delta / 2) - f(x - \delta / 2)}{\delta}
$$
我们可以取 $\delta$ 为 1，即 1 个纹理像素，有：

```c
float2 du = float2(_HeightMap_TexelSize.x * 0.5, 0);
float u1 = tex2D(_HeightMap, i.uv - du);
float u2 = tex2D(_HeightMap, i.uv + du);
float3 tangent = float3(1, 0, u2 - u1);
```

同理可以得到副切线：

```c
float2 dv = float2(0, _HeightMap_TexelSize.y * 0.5);
float v1 = tex2D(_HeightMap, i.uv - dv);
float v2 = tex2D(_HeightMap, i.uv + dv);
float3 bitangent = float3(0, 1, v2 - v1);
```

则切线空间下的法向可以通过叉积得到：

```c
float3 normal = normalize(cross(tangent, bitangent));
```

不过，此处的叉积结果可以直接写为 `float3(u1 - u2, v1 - v2, 1)`，以减少运算。

### 法线贴图（normal map）

高度贴图的缺点是一个像素要采样四次。

因为插值等会破坏纹理中法线的模长，因此法线贴图的 mipmap 生成不能简单的 downsample。

当使用 DXT5nm 压缩格式时，纹理只存储 x 与 y 坐标，且 x 存于 a 通道中。因为 DXT5nm/DXT5 压缩格式中

* r、b 有 5 位、g 有 6 位、a 有 8 位
* rgb 与 a 是独立的

在此时，不能简单地从纹理中采样作为结果，而是要手动计算出 z：

```c
float3 normal;
normal.xy = tex2D(_NormalMap, fin.uv).ga * 2 - 1;
normal.z = sqrt(1 - saturate(dot(normal.xy, normal.xy)));
```

### Unity 提供的 Unpack Normal 系列函数

（此段与原教程存在差异，原教程只说了 `UnpackScaleNormal` 其给出的内容与开源项目有所区别，应该是版本差别所致，此处以开源项目为准）

Unity 使用宏 `UNITY_NO_DXT5nm` 来判断压缩格式是否为 DXT5nm，并提供了一系列函数来进行法线的提取：

* `fixed3 UnpackNormal(fixed4 packednormal)` in `UnityCG.cginc`
* `fixed3 UnpackNormalWithScale(fixed4 packednormal, float scale)` in `UnityCG.cginc`
* `half3 UnpackScaleNormal(half4 packednormal, half bumpScale)` in `UnityStandardUtils.cginc`

其中，后两者亦提供了凹凸缩放的参数 `bumpScale`，作用于 xy 分量，以此来缩减或放大（甚至反向）法线贴图的效果。

`UnpackNormal()` 的代码如下。其中，“do the trick”是指，在压缩格式为 BC5 时，亦需要特殊处理，DXT5nm 是（1，y，1，x）而 BC5 是（x，y，0，1），这一句 `packednormal.x *= packednormal.w` 统一了二者的处理（所以函数名才叫 rg or ag）。

```c
fixed3 UnpackNormalmapRGorAG(fixed4 packednormal)
{
    // This do the trick
    packednormal.x *= packednormal.w;

    fixed3 normal;
    normal.xy = packednormal.xy * 2 - 1;
    normal.z = sqrt(1 - saturate(dot(normal.xy, normal.xy)));
    return normal;
}
inline fixed3 UnpackNormal(fixed4 packednormal)
{
#if defined(UNITY_NO_DXT5nm)
    return packednormal.xyz * 2 - 1;
#else
    return UnpackNormalmapRGorAG(packednormal);
#endif
}
```

而 `UnpackNormalWithScale()` 只是增加了缩放的处理：

```c
fixed3 UnpackNormalWithScale(fixed4 packednormal, float scale)
{
#ifndef UNITY_NO_DXT5nm
    packednormal.x *= packednormal.w;
#endif
    fixed3 normal;
    normal.xy = (packednormal.xy * 2 - 1) * scale;
    normal.z = sqrt(1 - saturate(dot(normal.xy, normal.xy)));
    return normal;
}
```

（我也不知道为什么这里就不拆分函数了。。。）

`UnpackScaleNormal()` 在非 DXT5nm 下，缩放会得到非单位向量（它是直接在取出来的结果的 xy 上做缩放），而原教程给出的版本甚至不会在非 DXT5nm 时处理缩放，猜测可能的原因是非 DXT5nm 主要在移动设备上（Unity 说 DXT5nm 是非移动设备上的默认压缩格式），考虑到其算力较弱，移除了该功能或去除单位化的代价。另一个区别是 `fixed` 与 `half`。

### 法线贴图的细节贴图

细节贴图是另一张（或生成自同一张的）贴图以经过较大的缩放系数处理后的另一组 uv 采样后，与原贴图进行混合的增加细节的方式。

在主颜色中，细节贴图是主题颜色为 0.5 灰度的灰色图片，与原来的值相乘并再乘 2（`unity_ColorSpaceDouble`）得到。

对法线贴图使用细节贴图的话，视作是两个表示凹凸表面的二元函数的加和，其导数也是加和的关系。法线可以表示为 $(-f'_u, -f'_v, 1)$，那么加和的结果就是 $(-f'_u - g'_u, -f'_v - g'_v, 1)$，对应的代码如下：

```c
float3 mainNorm = UnpackNormalWithScale(tex2D(_NormalMap, fin.uv.xy), _BumpScale);
float3 detailNorm = UnpackNormalWithScale(tex2D(_DetailNormalMap, fin.uv.zw), _DetailBumpScale);
float3 normal = float3(mainNorm.xy / mainNorm.z + detailNorm.xy / detailNorm.z, 1);
normal = normalize(normal);
```

另一种被称作 whiteout 混合的方式是，对法线的每一分量同时乘上两个位于分母上的 z 后，删去 xy 分量中的乘 z 分量的部分，即：

```c
float3 mainNorm = UnpackNormalWithScale(tex2D(_NormalMap, fin.uv.xy), _BumpScale);
float3 detailNorm = UnpackNormalWithScale(tex2D(_DetailNormalMap, fin.uv.zw), _DetailBumpScale);
float3 normal = float3(mainNorm.xy + detailNorm.xy, mainNorm.z * detailNorm.z);
normal = normalize(normal);
```

在凹凸度更大的表面上，此种方法有着更好的表现。

Unity 内置着色器中的 `BlendNormals(half3 n1, half3 n2)` 便是做了这样的事。

### 切线空间到世界空间

副切线（bitangent）亦称副法线（binormal），原教程用的后者的称呼，我则比较习惯于前者。

根据基底变换的公式，转化的方式如下：

```c
// in VS
vout.worldTan = UnityObjectToWorldDir(vin.tangent);
vout.worldBitan = cross(vin.norm, vin.tangent)
    * vin.tangent.w * unity_WorldTransformParams.w;

// in FS
fin.worldNorm = normalize(
    fin.worldTan * normal.x +
    fin.worldBitan * normal.y +
    fin.worldNorm * normal.z
);
```

其中：

* 切线的 w 为 -1 或 1，用于修正因建模时镜像操作带来的副切线方向相反的问题。-1 表示切线空间与世界空间手性相反，1 表示相同。Unity 使用左手的世界空间和右手的切线空间，所以一般是 -1
* `unity_WorldTransformParams.w` 为 -1 或 1，用于修正因负缩放带来的副切线方向相反的问题

Unity 的副切线是在顶点着色器中计算出来的。

### Mikktspace

mikktspace 是 Mikkelsen's tangent space 的缩写，是一个切线空间生成的标准。

mikktspace 给出了一个计算切线的算法，从使用这种算法得到的切线构建对应的切线空间就需要用上述我们做过的做法，即：

* 顶点着色器输入单位化的法线与切线
* 副切线按 `cross(normal, tangent.xyz) * tangent.w` 计算
* 片段着色器输入的法线、切线（与副切线）不做单位化处理

mikktspace 得到的切线空间不一定是正交的。

## 7 Shadows

### Unity 定向光的阴影绘制流程

Unity 阴影的绘制流程是：

* 对于每一个投射阴影的光源
  * 写入深度
  * 绘制屏幕空间阴影（摄像机空间的深度好像是提前绘制的）
* 正常绘制物体

### Unity 的级联阴影

Unity 使用级联阴影贴图，默认为 4 级。

阴影级的选择

* Stable Fit - 根据到摄像机的距离决定，得到圆形的边界
* Close Fit - 根据摄像机深度决定，得到直的边界

Close Fit 有着更高的效率，但有着更明显的 shadow edge swimming 的现象（随着摄像机的转动或移动，阴影级边界改变造成的阴影边缘的绘制结果改变，其看起来就像是边缘在游动一样）。

### 阴影贴图与抗锯齿

绘制屏幕空间阴影时，因为是绘制屏幕四边形，所以 MSAA 不会在内部起作用。

FXAA 这样的后处理 AA 则可以处理。

### 阴影粉刺与偏移

为了消除 shadow acne（阴影粉刺，因为阴影贴图像素和物体表面不对齐造成的一道道的阴影伪像），通常的做法就是给深度加上偏移，Unity 有两个偏移参数：偏移与法线偏移，均是光源组件的参数。

Unity 内置着色器函数 `UnityApplyLinearShadowBias(pos)` 处理偏移。代码中的 `unity_LightShadowBias.y`，我看到的说法是，定向光是 1，点光与聚光是 0。`UNITY_REVERSED_Z` 是在某些目标上（如 DX11），z 的范围是反过来的。其具体算法是：

* 定向光：固定的偏移量。clamp 是把位于正交摄像机近平面背后的顶点扔到近平面上，因为定向光光源虽然有它自己的位置，但实际上它背后的物体也应当投射阴影。
* 点光/聚光：与 w（z）相关的偏移量。我看到的说法是，偏移量要和非线性 z 成比例（https://forum.unity.com/threads/how-to-unpack-unity_lightshadowbias-and-_worldspacelightpos0-values.450382/），所以是这个样子。

```c
float4 UnityApplyLinearShadowBias(float4 clipPos)
{
    // For point lights that support depth cube map, the bias is applied in the fragment shader sampling the shadow map.
    // This is because the legacy behaviour for point light shadow map cannot be implemented by offseting the vertex position
    // in the vertex shader generating the shadow map.
#if !(defined(SHADOWS_CUBE) && defined(SHADOWS_CUBE_IN_DEPTH_TEX))
    #if defined(UNITY_REVERSED_Z)
        // We use max/min instead of clamp to ensure proper handling of the rare case
        // where both numerator and denominator are zero and the fraction becomes NaN.
        clipPos.z += max(-1, min(unity_LightShadowBias.x / clipPos.w, 0));
    #else
        clipPos.z += saturate(unity_LightShadowBias.x/clipPos.w);
    #endif
#endif

#if defined(UNITY_REVERSED_Z)
    float clamped = min(clipPos.z, clipPos.w*UNITY_NEAR_CLIP_VALUE);
#else
    float clamped = max(clipPos.z, clipPos.w*UNITY_NEAR_CLIP_VALUE);
#endif
    clipPos.z = lerp(clipPos.z, clamped, unity_LightShadowBias.y);
    return clipPos;
}
```

`UnityClipSpaceShadowCasterPos(pos, norm)` 处理法线偏移，把顶点向表面内部推一定距离，法线与光源向量偏差越大，移动的距离越大。感性分析了一下，造成的效果应该是阴影向内收缩了一段距离。

```c
float4 UnityClipSpaceShadowCasterPos(float4 vertex, float3 normal)
{
    float4 wPos = mul(unity_ObjectToWorld, vertex);

    if (unity_LightShadowBias.z != 0.0)
    {
        float3 wNormal = UnityObjectToWorldNormal(normal);
        float3 wLight = normalize(UnityWorldSpaceLightDir(wPos.xyz));

        // apply normal offset bias (inset position along the normal)
        // bias needs to be scaled by sine between normal and light direction
        // (http://the-witness.net/news/2013/09/shadow-mapping-summary-part-1/)
        //
        // unity_LightShadowBias.z contains user-specified normal offset amount
        // scaled by world space texel size.

        float shadowCos = dot(wNormal, wLight);
        float shadowSine = sqrt(1-shadowCos*shadowCos);
        float normalBias = unity_LightShadowBias.z * shadowSine;

        wPos.xyz -= wNormal * normalBias;
    }

    return mul(UNITY_MATRIX_VP, wPos);
}
```

其他要说明的是，我在 Unity 的帧调试器中发现 `unity_LightShadowBias.x` 是负值，不知道是不是跟 `UNITY_REVERSED_Z` 有关（我这里确实是反 z）。另一点，`unity_LightShadowBias.x` 和 `unity_LightShadowBias.z` 都不等于设置的值，且在四个阴影级中不同。前者随着阴影级的增大（指视角更大的阴影级）值在减小，后者则在增大。应该是什么跟设置输入和阴影级相关参数有关的算式。

（PS：看着一段时看了一些 Unity 的文档和一些相关博客，总感觉自己是不是学了什么假投影矩阵，怎么自己写的时候就没见过反 z 这种东西，明明 DX12 也写过一点。。。）

### 接受阴影

在 Forward Base 中开启 `SHADOWS_SCREEN` 关键字编译器变体。

顶点着色器输出的阴影坐标即屏幕四边形的 uv，从裁剪空间的坐标变换而来（一个是透视除法的处理，一个是 [-1, 1] 到 [0, 1] 的处理）。可以先透视除法再扔去插值，也可以先扔去插值再在片段着色器做除法。Unity 是在片段着色器中做除法，借助 `tex2DProj()`。

此处会遇见因图形 API 不同造成的问题，DX 的 v 是从上到下增长，OpenGL 是从下到上增长，但是 NDC 的 y 都是从下到上增长，因此需要对 DX 做处理。Unity 提供了函数来处理这点：

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

其中 `_ProjectionParams.x` 就是用来处理这个翻转问题的。

Unity 提供的一系列处理以上内容的宏，`SHADOW_COORDS(idx)`、`TRANSFER_SHADOW(a)`、`SHADOW_ATTENUATION(a)` 都做了一些规定：

* 顶点着色器的输入变量名为 `v`（定向光不会有问题，但点光和聚光需要满足这个）
* 顶点着色器输入的 `POSITION` 的变量名为 `vertex`
* 片段着色器输入的 `SV_POSITION` 的变量名为 `pos`

此外，另有宏 `UNITY_SHADOW_COORDS(idx)` 和 `UNITY_TRANSFER_SHADOW(a, coord)`。前者和无前缀版本的区别没有看出来，最终都是定义一个 `float4 _ShadowCoord` 或不定义；后者在有 lightmap 时会和无前缀版本不一样。

最后和光源坐标的 `_LightCoord` 合并起来，有带前缀版本的一对宏和不带前缀版本的一对宏。（不带的那一对总是能在查资料的时候看到，但感觉对比 `UNITY_LIGHT_ATTENUATION` 与 `LIGHT_ATTENUATION`，应该带前缀的是后来的吧应该）

```c
#define UNITY_LIGHTING_COORDS(idx1, idx2) DECLARE_LIGHT_COORDS(idx1) UNITY_SHADOW_COORDS(idx2)
#define LIGHTING_COORDS(idx1, idx2) DECLARE_LIGHT_COORDS(idx1) SHADOW_COORDS(idx2)
#define UNITY_TRANSFER_LIGHTING(a, coord) COMPUTE_LIGHT_COORDS(a) UNITY_TRANSFER_SHADOW(a, coord)
#define TRANSFER_VERTEX_TO_FRAGMENT(a) COMPUTE_LIGHT_COORDS(a) TRANSFER_SHADOW(a)
```

### 更多阴影

Forward Add 里也支持阴影贴图，不同种类光源对应的着色器变体关键字（宏）是：

* 定向光 - `SHADOWS_SCREEN`
* 点光 - `SHADOWS_CUBE`
* 聚光 - `SHADOWS_DEPTH`

此外，还有一个宏是 `SHADOWS_SOFT`，在点光和聚光时会有（定向光的软影是在屏幕空间阴影是处理的），所以 `multi_compile_fwdadd_fullshadow` 一共会产生 5 + 5 + 3 个着色器变体。

点光和聚光不使用屏幕空间阴影，而是调用函数 `UnitySampleShadowmap` 去采样深度和作比较，以及根据 `SHADOWS_SOFT` 判断要不要做 PCF，阴影坐标（`TRANSFER_SHADOW` 做的事）也会处理为相应需要的东西。

点光阴影要使用不同的 shadow caster（因为点光阴影存的是距离，其他两个存的就是 z）：

```c
VertexOutput vert(VertexInput vin) {
    VertexOutput vout;
    vout.pos = UnityObjectToClipPos(vin.pos);
    vout.lightVec = mul(unity_ObjectToWorld, vin.pos).xyz - _LightPositionRange.xyz;
    return vout;
}

float4 frag(VertexOutput fin) : SV_TARGET {
    float depth = length(fin.lightVec) + unity_LightShadowBias.x;
    depth *= _LightPositionRange.w;
    return UnityEncodeCubeShadowDepth(depth);
}
```

其中，`_LightPositionRange` 的 xyz 为光源位置，w 为半径倒数。可以看到，偏移是常数，而且就是设置的值，不使用法线偏移。

`UnityEncodeCubeShadowDepth` 是在不支持浮点输出的目标上，用 R8G8B8A8 这样的目标来得到一个 32 位的 float。采样时需要做相反的处理。

为了把不同光源的 shadow caster 合并在一起，Unity 提供了宏 `TRANSFER_SHADOW_CASTER_NOPOS(o, opos)` 和 `SHADOW_CASTER_FRAGMENT(i)` 分别作为顶点着色器和片段着色器的内容。同样地，规定了一些输入的变量名，具体规定和之前一样。

### PCF

在代码里看了一圈，应该是以下这样：

* 定向光 - 移动设备 5x5，非移动设备 7x7
* 聚光 - 移动设备 4，非移动设备 3x3
* 点光 - 移动设备 4，非移动设备 4

## 8 Reflection

### 反射探针

物体可以设置为“对反射探针静态”，这样物体会被烘焙到反射探针的立方体贴图上的同时允许物体在运行时运动（当然，贴图不会更新）。

反射探针也可以设置为实时，或一些自定义设置。

### 采样天空盒

之前一直没有处理 `UnityIndirect::specular`，用反射光线采样天空盒作为非直接光的镜面反射部分。

```c
float4 env = UNITY_SAMPLE_TEXCUBE(unity_SpecCube0, reflectDir);
indirect.specular = DecodeHDR(env, unity_SpecCube0_HDR);
```

天空盒是 HDR 的，所以需要处理为 LDR，使用 Unity 内置的函数 `DecodeHDR()` 处理。HDR 的采样结果的 4 个通道为 RGBM，结合 `decodeInstruction.xy` 进行转换：$ldr = hdr \times x M^y$。

```c
inline half3 DecodeHDR (half4 data, half4 decodeInstructions) {
	// If Linear mode is not supported we can skip exponent part
	#if defined(UNITY_NO_LINEAR_COLORSPACE)
		return (decodeInstructions.x * data.a) * data.rgb;
	#else
		return (decodeInstructions.x * pow(data.a, decodeInstructions.y)) *
			data.rgb;
	#endif
}
```

要考虑粗糙度的影响，根据粗糙度选择 mipmaps 级采样，Unity 使用 $1.7r - 0.7 r^2$ 来找 mipmaps 级。其中，`UNITY_SEPCUDE_LOD_STEPS` 被定义为常量 6。（在开源项目中找到的位置是在 `UnityStandardConfig.cginc`，与原教程说的不同）

```c
float roughness = 1 - _Smoothness;
roughness *= 1.7 - 0.7 * roughness;
float4 env = UNITY_SAMPLE_TEXCUBE_LOD(unity_SpecCube0, reflectDir,
    roughness * UNITY_SPECCUBE_LOD_STEPS);
indirect.specular = DecodeHDR(env, unity_SpecCube0_HDR);
```

Unity 提供了函数 `Unity_GlossyEnvironment()` 来处理以上的内容。在开源项目中找到的函数体比原教程给出的还要简洁一些，而且所属文件又不一样了，是在 `UnityImageBasedLighting.cginc`。（话说原来纹理也能做参数的啊）

### 更精确地采样（box projection）

因为正在绘制的像素位置和反射探针的位置一般不同，从同一个方向上看去的内容会不太一样，当反射面是平面，或者一个反射探针周围有多个类似物体的时候，就能明显地感受到这个现象，因为我们之前没有处理，绘制出来的表现会有很大的违和感。

考虑修正这一点，反射探针有一个盒型（轴对齐盒）的边界，我们认为我们反射光线打到了盒上，求出交点，然后用探针位置到交点的方向采样立方体贴图。

```c
float3 BoxProjection(float3 direction, float3 position, float3 cubemapPosition, float3 boxMin, float3 boxMax) {
    float3 factors = ((direction > 0 ? boxMax : boxMin) - position) / direction;
    float scale = min(factors.x, min(factors.y, factors.z));
    return position + direction * scale - cubemapPosition;
}
```

Unity 内置函数 `BoxProjectedCubemapDirection` 做了以上的事（且不像原教程所说写了不必要的单位化），有两点：

* `unity_SpecCube0_ProbeProjection.w` 存储了是否需要使用 box projection（当大于 0 时），在其组件中设置
* 其中使用了宏 `UNITY_RBANCH` 来强制编译输出**使用**分支指令的代码，因为分支条件是 uniform 的（对每一个像素一致的）

### 环境贴图的混合

Unity 选取最近的 2 个探针（或天空盒，根据物体的设置），混合系数存在 `unity_SpecCube0_BoxMin.w` 中，使用线性插值，1 表示第一个立方体贴图（`unity_SpecCube0`），0 表示第二个立方体贴图（`unity_SpecCube1`）。

为了减少不必要的贴图采用，仅当混合系数小于一个值（比如 0.999）时才做混合，这个分支也是 uniform 的（因为混合系数是对物体而言的）。

第二个立方体贴图没有自己的采样器（`UNITY_PASS_TEXCUBE` 实际上是传一个纹理加一个采样器），因此要使用 `UNITY_PASS_TEXCUBE_SAMPLER(unity_SpecCube1,unity_SpecCube0)`，否则会编译错误 。我没有改，被 Unity 直接改了代码。。。

```c
    float3 reflectDir = reflect(-viewDir, fin.worldNorm);
    envData.reflUVW = BoxProjectedCubemapDirection(reflectDir, fin.worldPos,
        unity_SpecCube0_ProbePosition, unity_SpecCube0_BoxMin, unity_SpecCube0_BoxMax);
    float3 probe0 = Unity_GlossyEnvironment(UNITY_PASS_TEXCUBE(unity_SpecCube0),
        unity_SpecCube0_HDR, envData);
#if UNITY_SPECCUBE_BLENDING
    UNITY_BRANCH
    if (unity_SpecCube0_BoxMin.w < 0.99999) {
        envData.reflUVW = BoxProjectedCubemapDirection(reflectDir, fin.worldPos,
            unity_SpecCube1_ProbePosition, unity_SpecCube1_BoxMin, unity_SpecCube1_BoxMax);
        float3 probe1 = Unity_GlossyEnvironment(
            UNITY_PASS_TEXCUBE_SAMPLER(unity_SpecCube1, unity_SpecCube0),
            unity_SpecCube1_HDR, envData);
        indirect.specular = lerp(probe1, probe0, unity_SpecCube0_BoxMin.w);
    } else {
        indirect.specular = probe0;
    }
#else
    indirect.specular = probe0;
#endif
```