---
title: Unity 中的渲染 学习笔记（一）
date: 2021-02-22 21:01:25
tags: [CG, Unity, 学习笔记]
categories: 学习笔记（CG）
---

CatLikeCoding 上的诸多 Unity 系列教程中，关于 Unity 的渲染相关实现的教程的学习笔记，原教程用 20 节讲述了 Unity 自己的渲染相关的一些具体细节。本文是其中 1~5 节的学习笔记，此 5 节的标题依次为：

* 1 - Transform
* 2 - Shader
* 3 - Combining Textures
* 4 - The First Light
* 5 - Multiple Lights

原教程及其他参考：

* [Unity Rendering Tutorial - CatLikeCoding](https://catlikecoding.com/unity/tutorials/rendering/)
* [Unity-Build-in-Shaders](https://github.com/TwoTailsGames/Unity-Built-in-Shaders)

<!-- more -->

## 1 Transform

Unity 的旋转顺序是 Y、X、Z。

（原文说矩阵是 ZXY 的顺序乘起来）

## 2 Shader

`_ST` 中，xy 是 tiling，zw 是 offset，即：

```c
#define TRANSFORM(texture, name) texture.uv * name##_ST.xy + name##_ST.zw;
```

## 3 Combining Textures

## 4 The First Light

### 法线的变换

上者是原教程中给出的版本，下者是 Unity 内置着色器开源项目中的版本。不管是哪一个，都避免了在着色器里算逆转置，而且对转置的处理比较巧妙。

```c
inline float3 UnityObjectToWorldNormal(in float3 norm) {
    return normalize(
        unity_WorldToObject[0].xyz * norm.x +
        unity_WorldToObject[1].xyz * norm.y +
        unity_WorldToObject[2].xyz * norm.z
    );
}

inline float3 UnityObjectToWorldNormal( in float3 norm )
{
#ifdef UNITY_ASSUME_UNIFORM_SCALING
    return UnityObjectToWorldDir(norm);
#else
    // mul(IT_M, norm) => mul(norm, I_M) => {dot(norm, I_M.col0), dot(norm, I_M.col1), dot(norm, I_M.col2)}
    return normalize(mul(norm, (float3x3)unity_WorldToObject));
#endif
}
```

### Clamped 点积

clamped 的单位向量点积（`DotClamped`），根据构建目标选择使用 `saturate` 还是 `max`。（不过这个函数被弃用了，我在现在的开源项目中看到都在使用 `saturate` 的样子）

### Forward Base

前向渲染

只是用单个光源，即贡献最大的光源，一般为场景的定向光（太阳光）

### 光源信息

`_WorldSpaceLightPos0`

* 点光/聚光 - xyz 为位置，w 为 1
* 定向光 - xyz 为方向（的反方向），w 为 0
* `float3 lightDir = _WorldSpaceLightPos.xyz - _worldSpaceLightPos.w * worldPos`

`_LightColor0` - 光源颜色

### 浮点数精度（`half` 与 `fixed`）

PC 上 `half`、`fixed` 都是 `float`，在移动设备上才表现出区别

### Unity 内置的光照相关函数

`EnergyConservationBetweenDiffuseAndSpecular(albedo, specColor, out oneMinusReflectivity)` - 在给定镜面反射颜色下削弱漫反射使结果差不多满足能量守恒，有无衰减、单色衰减（RGB 衰减度相同，为镜面反射颜色 RGB 最大项）、彩色衰减。

`DiffuseAndSpecularFromMetallic(albedo, metallic, out specColor, out oneMinusReflectivity)` - 利用金属度在电介质镜面反射率（`unity_ColorSpaceDielectricSpec`）与反照率之间插值出 `specColor`，1 减反射率亦是用金属度在电介质的 1 减反射率与 0 之间插值的。

Unity 的金属度在 Gamma 空间

`UNITY_BRDF_PBS(albedo, specularTint, oneMinusReflectivity, smoothness, normalDir, viewDir, light, indirect)`，是个宏，根构建目标和选项来选择具体函数（具体做了什么不在原教程的范围内，感觉值得单独拿出来看一看）

## 5 Multiple Lights

### Forward Add

前向渲染

只是用单个光源

附加到主光源上，需要手动设置 `Blend One One`，可以不写深度（`ZWrite Off`）

多个定向光会影响 Unity 的动态批次合并

### 点光源的衰减

平方反比 $1 / d^2$

分母加 1 的平方反比（$1 / (1 + d^2)$），使最近时达到 1

### 光源范围

超过范围的物体不会跑 Forward Add

为了更好的边界效果，衰减函数和光源范围应当有关

### 光源 Cookie

光源的蒙版，定向光和聚光是 2D 纹理，点光是立方体纹理

### 光源与 `multiple_compile`

`#pragma multi_compile_fwdadd` 相当于 `#pragma multi_compile POINT DIRECTIONAL SPOT POINT_COOKIE DIRECTIONAL_COOKIE `，每一个会生成一个编译后的着色器程序，以宏的方式控制

在 `AutoLight.cginc` 中，什么宏定义都都没的话，会默认为 `DIRECTIONAL`

Forward Base 不需要是因为主光一定是定向光

### `AutoLight.cginc` 中的衰减计算

代码均取自 Unity 内置着色器开源项目

#### 点光（POINT）

```c
#ifdef POINT
sampler2D_float _LightTexture0;
unityShadowCoord4x4 unity_WorldToLight;
#   define UNITY_LIGHT_ATTENUATION(destName, input, worldPos) \
        unityShadowCoord3 lightCoord = mul(unity_WorldToLight, unityShadowCoord4(worldPos, 1)).xyz; \
        fixed shadow = UNITY_SHADOW_ATTENUATION(input, worldPos); \
        fixed destName = tex2D(_LightTexture0, dot(lightCoord, lightCoord).rr).r * shadow;
#endif
```

衰减利用距离平方在 2D 纹理的主对角线上采样。

#### 带 Cookie 的点光（POINT_COOKIE）

```c
#ifdef POINT_COOKIE
samplerCUBE_float _LightTexture0;
unityShadowCoord4x4 unity_WorldToLight;
sampler2D_float _LightTextureB0;
#   if !defined(UNITY_HALF_PRECISION_FRAGMENT_SHADER_REGISTERS)
#       define DECLARE_LIGHT_COORD(input, worldPos) unityShadowCoord3 lightCoord = mul(unity_WorldToLight, unityShadowCoord4(worldPos, 1)).xyz
#   else
#       define DECLARE_LIGHT_COORD(input, worldPos) unityShadowCoord3 lightCoord = input._LightCoord
#   endif
#   define UNITY_LIGHT_ATTENUATION(destName, input, worldPos) \
        DECLARE_LIGHT_COORD(input, worldPos); \
        fixed shadow = UNITY_SHADOW_ATTENUATION(input, worldPos); \
        fixed destName = tex2D(_LightTextureB0, dot(lightCoord, lightCoord).rr).r * texCUBE(_LightTexture0, lightCoord).w * shadow;
#endif
```

额外有一个立方体纹理，采样取 w 并乘在结果里。

#### 定向光（DIRECTIONAL）

```c
#ifdef DIRECTIONAL
#   define UNITY_LIGHT_ATTENUATION(destName, input, worldPos) fixed destName = UNITY_SHADOW_ATTENUATION(input, worldPos);
#endif
```

定向光只考虑阴影，自身无衰减。

#### 带 Cookie 的定向光（DIRECTIONAL_COOKIE）

```c
#ifdef DIRECTIONAL_COOKIE
sampler2D_float _LightTexture0;
unityShadowCoord4x4 unity_WorldToLight;
#   if !defined(UNITY_HALF_PRECISION_FRAGMENT_SHADER_REGISTERS)
#       define DECLARE_LIGHT_COORD(input, worldPos) unityShadowCoord2 lightCoord = mul(unity_WorldToLight, unityShadowCoord4(worldPos, 1)).xy
#   else
#       define DECLARE_LIGHT_COORD(input, worldPos) unityShadowCoord2 lightCoord = input._LightCoord
#   endif
#   define UNITY_LIGHT_ATTENUATION(destName, input, worldPos) \
        DECLARE_LIGHT_COORD(input, worldPos); \
        fixed shadow = UNITY_SHADOW_ATTENUATION(input, worldPos); \
        fixed destName = tex2D(_LightTexture0, lightCoord).w * shadow;
#endif
```

在 2D 的 cookie 纹理中采样，取 w。

#### 聚光（SPOT）

```c
#ifdef SPOT
sampler2D_float _LightTexture0;
unityShadowCoord4x4 unity_WorldToLight;
sampler2D_float _LightTextureB0;
inline fixed UnitySpotCookie(unityShadowCoord4 LightCoord)
{
    return tex2D(_LightTexture0, LightCoord.xy / LightCoord.w + 0.5).w;
}
inline fixed UnitySpotAttenuate(unityShadowCoord3 LightCoord)
{
    return tex2D(_LightTextureB0, dot(LightCoord, LightCoord).xx).r;
}
#if !defined(UNITY_HALF_PRECISION_FRAGMENT_SHADER_REGISTERS)
#define DECLARE_LIGHT_COORD(input, worldPos) unityShadowCoord4 lightCoord = mul(unity_WorldToLight, unityShadowCoord4(worldPos, 1))
#else
#define DECLARE_LIGHT_COORD(input, worldPos) unityShadowCoord4 lightCoord = input._LightCoord
#endif
#   define UNITY_LIGHT_ATTENUATION(destName, input, worldPos) \
        DECLARE_LIGHT_COORD(input, worldPos); \
        fixed shadow = UNITY_SHADOW_ATTENUATION(input, worldPos); \
        fixed destName = (lightCoord.z > 0) * UnitySpotCookie(lightCoord) * UnitySpotAttenuate(lightCoord.xyz) * shadow;
#endif
```

衰减项和点光一样在对角线上采样；cookie 项在 2D 纹理中采样（因为是透视矩阵所以手动除以了 w），取 w。最后合并时还要考虑是否在光源的背面。

#### 其他

Unity 内置着色器中亦有 `LIGHT_ATTENUATION(a)`，其已被弃用，`UNITY_LIGHT_ATTENUATION(destName, input, worldPos)` 是其代替。旧的版本就好像是 `UNITY_HALF_PRECISION_FRAGMENT_SHADER_REGISTERS` 默认被开启了一样，从 v2f 中取 `_LightCoord`。在新版本中，唯独 POINT 不考虑该宏，不知原因。

此外，衰减项在 x/r 通道，cookie 在 w/a 通道，不知原因。

### 顶点光照

主光源使用 Forward Base，一定量的光源使用 Forward Add（默认为 4 个），之后是顶点光照（更确切地说，为了与 Forward Add 平滑过渡，有一个光源同时出现在了 Forward Add 与顶点光中），其着色器变体的宏是 `VERTEXLIGHT_ON`。

最多支持 4 个顶点光，其位置存在 `unity_4LightPosX0`、`unity_4LightPosY0`、`unity_4LightPosZ0` 中，分别存了 4 个 x、4 个 y、4 个 z，方便并行。颜色则存在 `unity_LightColor[]` 中。衰减系数存在 `unity_4LightAtten0` 中（4 个光源的系数相同）。

`Shade4PointLights` 是内置的计算 4 个顶点光的函数，其使用的衰减函数就是 $1 / (1 + kd^2)$，在 Forward Base 中的顶点着色器中被使用。

### 球谐光照

如果还有更多的需要渲染的光源，使用球谐光照（应该会单独拿出来学一次的）。同样的，为了平滑过渡，有一个光源同时出现在了顶点光与球谐光中。环境光也使用球谐光照。

球谐光照是把剩下的光源以及环境光预先烘培，以若干球谐函数（Unity 选取了前 3 条带共 9 个函数作为基函数）为基底表示近似。又因为是 RGB 三个通道，固一共有 27 个系数。拆解 9 个基函数，结果可以表示为：
$$
a + bx + cy + dz + exy + fyz + gz^2 + hxz + i(x^2 - y^2)
$$
计算时，在 Forward Base 的片段着色器中，把法向带进去进行计算（使用球谐光照的部分算是漫反射项，所以带入法向就很合理；球谐函数拟合镜面反射这种高频的东西效果不太好，所以也不会去算）。Unity 把系数存在 `unity_SHAr`、`unity_SHAg`、`unity_SHAb`、`unity_SHBr`、`unity_SHBg`、`unity_SHBb`、`unity_SHC` 这 7 个向量中，`unity_SHA` 记录常数项与一次项的系数，`unity_SHB` 记录三个交叉项与 $z^2$ 的系数，`unity_SHC` 记录 $(x^2 - y^2)$ 的系数。Unity 的内置函数 `ShadeSH9(normal)` 的计算方法大致如下：

```c
half3 ShadeSH9(half4 normal) {
    half3 a;
	a.r = dot(unity_SHAr, normal);
    a.g = dot(unity_SHAg, normal);
    a.b = dot(unity_SHAb, normal);

    half3 b;
    half4 vb = normal.xyzz * normal.yzzx;
	b.r = dot(unity_SHBr, vb);
    b.g = dot(unity_SHBg, vb);
    b.b = dot(unity_SHBb, vb);
    
    half vc = normal.x * normal.x - normal.y * normal.y;
    half c = unity_SHC.rgb * vc;
    
    return a + b + c;
}
```

实际中的 `ShadeSH9(normal)` 并非如此，它把计算拆在了两个函数（不过把两个函数内嵌就是这样了），还会在 Gamma 空间下渲染时进行一次线性空间到 Gamma 空间的变换。