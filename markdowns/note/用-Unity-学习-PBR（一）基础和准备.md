---
title: 用 Unity 学习 PBR（一）基础和准备
date: 2021-02-14 14:57:21
tags: [CG, PBR, Unity, 学习笔记]
categories: 学习笔记（CG）
---

想着学一下 Unity 的 ShaderLab，一是提升引擎的使用能力，二是通过使用实际的引擎来反思自己的玩具渲染器应该怎样改进或重构。找到了用 Unity 搞 PBR 的教程，正好能巩固和深入学习一下以前只在 Learn OpenGL 学过一次的 PBR 的那套东西。

参考

* [基于物理的渲染（PBR）白皮书 - 毛星云](https://github.com/QianMo/PBR-White-Paper)
* [Physically Based Rendering Algorithms: A Comprehensive Study In Unity3D](https://www.jordanstevenstechart.com/physically-based-rendering)

<!-- more -->

## PBR 理论基础

只有反射的渲染方程
$$
L_o = \int_{\Omega} f_r(w_i, w_o) L_i(w_i) (w_i \cdot n) \mathrm{d}w_i
$$
对于间接光照，使用基于图像的光照（IBL）；对于直接光照，积分就是对所有光源计算求和。

BRDF $f_r$ 拆解为漫反射（diffuse）与镜面反射（specular）。

漫反射项最经典的公式是 Lambert，也有一些其他更物理一些的公式。

镜面反射项，由 Cook-Torrance 提出的基于微平面（microfacet）的公式：
$$
f(l, v) = \frac{F(v, h) G(l, v, h) D(h)}{4 (n \cdot l) (n \cdot v)}
$$
其中：

* $n$ 即 normal，宏平面法向
* $l$ 即 light，渲染方程中的 $w_i$，光源方向
* $v$ 即 view，渲染方程中的 $w_o$，观察方向
* $h$ 即 half，光源方向与观察方向的中间方向
* $F$ 为菲涅尔项，描述出射光中镜面反射占比随视角的变化
* $D$ 为法线分布，描述微平面法向 $m$ 中有多少与 $h$ 平行，即宏平面的多少部分能为镜面反射提供贡献
* $G$ 为几何项，描述与 $h$ 平行的微平面法向中，有多少未因遮挡（masking，挡住入光）或阴影（shadowing，挡住出光）导致实际未能提供贡献

$F$、$D$、$G$ 使用一些参数来描述真实世界的各种表面材质，Disney 原则的 BRDF 提出，各个参数应在 0 ~ 1 内并能够线性地插值两端的结果，以便于美术人员对材质的修改。实现最简单的 PBR 需要的参数有：

* 粗糙度（roughness），主要影响 $D$ 与 $G$，即越光滑的表面，微平面法向就越更多地接近宏平面法向，也更少出现微平面的相互遮挡。在公式中一般用 $\alpha$ 表示，$\alpha$ 与暴露给外部的粗糙度通常并不是同一数值。一个常用的映射是 $\alpha = r^2$，Stevens 的教程中则使用了 $\alpha = (1 - (1 - r)^2)^2$
* 金属度（metallic），通过对金属（导体）与非金属（电介质）的插值来更灵活地描述一些材质。金属没有漫反射成分，且会有与非金属较大区别的菲涅尔项
* F0，入射角为 0（正对着表面入射）时镜面反射光的比例，常用的菲涅尔项的公式都是以 F0 为基础算出其他入射角下的镜面反射比例。金属的 F0 基本上就是金属自身的颜色，而非金属的 F0 一般 RGB 相同且不超过 0.1x。不过，直接提供 F0 作为参数并不是那么直观（即使有 F0 数据库的存在）；镜面发射颜色（specular color）可以认为是 F0

## Unity 准备

在 Unity 中新建一个 Unlit Shader，在其中修改我们需要的熟悉，编写我们自己的顶点与片段着色器（其实新建哪个着色器没有关系，只是选 Unlit Shader 会给一个使用顶点与片段着色器的模板）。

Unity 使用 ShaderLab 语言描述一个 Unity 着色器。在其中，我们主要会用到的部分是属性的编辑与 Cg/HLSL 着色器的编写。

属性的话，我们添加如下属性：

```
Properties
{
    _Color ("Color", Color) = (1, 1, 1 ,1)
    _MainTex ("Texture", 2D) = "white" {}
    _SpecularColor ("Specular Color", Color) = (1, 1, 1, 1)
    _Roughness ("Roughness", Range(0, 1)) = 0
    _Metallic ("Metallic", Range(0, 1)) = 0
    _Anisotropic ("Anisotropic", Range(0, 1)) = 1
}
```

并且在着色器代码中添加这些属性对应变量的定义：

```c
float4 _Color;
sampler2D _MainTex;
float4 _MainTex_ST;
float4 _SpecularColor;
float _Roughness;
float _Metallic;
float _Anisotropic;
```

着色器代码包含以下头文件：

```c
#include "UnityCG.cginc"
#include "AutoLight.cginc"
#include "Lighting.cginc"
```

修改顶点着色器的输入和输出如下：

```c
struct appdata {
    float4 vertex : POSITION;
    float3 normal : NORMAL;
    float3 tangent : TANGENT;
    float2 uv : TEXCOORD0;
};

struct v2f {
    float2 uv : TEXCOORD0;
    UNITY_FOG_COORDS(1)
    float4 vertex : SV_POSITION;

    float3 worldPos : TEXCOORD3;
    float3 worldNormal : TEXCOORD4;
    float3 worldTangent : TEXCOORD5;
    float3 worldBitangent : TEXCOORD6;
};
```

并相应修改顶点着色器：

```c
v2f vert(appdata v) {
    v2f o;
    o.vertex = UnityObjectToClipPos(v.vertex);
    o.uv = TRANSFORM_TEX(v.uv, _MainTex);
    UNITY_TRANSFER_FOG(o,o.vertex);

    o.worldPos = mul(unity_ObjectToWorld, v.vertex);
    o.worldNormal = UnityObjectToWorldNormal(v.normal);
    o.worldTangent = normalize(mul(unity_ObjectToWorld, v.tangent));
    o.worldBitangent = normalize(cross(o.worldNormal, o.worldTangent));

    return o;
}
```

片段着色器部分，我们先做一些准备工作，包括纹理采样、得到各个需要的向量、计算各个需要的点积、处理粗糙度转换等：

```c
float4 frag(v2f i) : SV_Target {
    // main color
    float4 mainTexColor = tex2D(_MainTex, i.uv);
    float3 mainColor = mainTexColor.rgb * _Color.rgb * (1 - _Metallic);

    // vectors
    float3 normalDir = normalize(i.worldNormal);
    float3 viewDir = normalize(_WorldSpaceCameraPos.xyz - i.worldPos);
    float3 lightDir = normalize(
        lerp(_WorldSpaceLightPos0.xyz, _WorldSpaceLightPos0.xyz - i.worldPos,
             _WorldSpaceLightPos0.w));
    float3 lightReflectDir = reflect(-lightDir, normalDir);
    float3 viewReflectDir = reflect(-viewDir, normalDir);
    float3 halfDir = normalize((viewDir + lightDir) * 0.5);

    // dot
    float NdotL = max(0, dot(normalDir, lightDir));
    float NdotH = max(0, dot(normalDir, halfDir));
    float NdotV = max(0, dot(normalDir, viewDir));
    float VdotH = max(0, dot(viewDir, halfDir));
    float LdotH = max(0, dot(lightDir, halfDir));
    float LdotV = max(0, dot(lightDir, viewDir));
    float RdotV = max(0, dot(lightReflectDir, viewDir));

    // roughnees
    float roughness = _Roughness * _Roughness;
    float roughnessSqr = roughness * roughness;

    // light attenuation
    float attenuation = LIGHT_ATTENUATION(i);
    float3 attenColor = attenuation * _LightColor0.rgb;
    
    // ...
}
```

同时新建一个材质，让材质绑定到新建的着色器上。

最后在场景中添加一个几何体，比如球什么的，使用刚刚新建的材质。这样我们在材质中修改参数或修改着色器的效果就可以直接在场景中的这个几何体上看到了。