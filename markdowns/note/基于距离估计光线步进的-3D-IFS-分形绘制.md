---
title: 基于距离估计光线步进的 3D IFS 分形绘制
date: 2021-03-20 21:54:38
tags:  [CG, 光线追踪, 光线步进, 分形, 学习笔记]
categories: 学习笔记（CG）
---

本文作为本人阅读 [Distance Esitmated 3D Fractals](http://blog.hvidtfeldts.net/index.php/2011/06/distance-estimated-3d-fractals-part-i/) 系列文章前 3 篇的笔记及一个扩展，将描述基于距离估计光线步进的任意 3D IFS（Iterated Function Systems）分形的绘制方法。本文首先将对光线步进（ray marching）和原文中的距离估计器（Distance Estimator）做一些简要的说明。之后将分析并给出 Sierpinski 四面体和 Cantor 立方体两个分形的距离估计器。最后会给出将此方法拓展到任意 3D IFS 绘制的算法，并以 Menger 海绵为例进行说明。后两部分均会给出核心代码和本人的绘制结果。

<!-- more -->

## 光线步进（Ray Marching）与距离估计器（Distance Estimator）

### 光线步进

这里必须先安利一波叶大的知乎系列文章[《用 C 语言画光》](https://zhuanlan.zhihu.com/p/30745861)，其内容就是基于 SDF 的 2D 光线步进进行 2D 绘制。

光线步进，类似于光线追踪，但不同的是，光线追踪是求出与几何体的交点，而光线步进是每次向前前进一段距离，直到达到设定的次数上界，视为没有相交，或单次前进距离小于一定阈值，视为相交。这样来看，光线步进也可以看作是一种求交点的方式，可以方便地放入一个光线追踪器里。

光线步进每一步要前进一段距离，这个距离要怎样确定呢？最简单的方法就是每次走固定很短的一段距离，[Rendering 20 - Parallax (CatLikeCoding)](https://catlikecoding.com/unity/tutorials/rendering/part-20/) 这篇在 Unity 中实现凹凸贴图的教程最后就是使用这样的光线步进方法，但在渲染场景这种场合下就不太合适，毕竟一次只走一点实在是太慢了。

叶大的文章是用 SDF（Signed Distance Field），有符号距离场。该场是一个三维坐标到距离的函数，即空间中一点到物体的最短距离，外侧为正、内侧为负。既然 SDF 描述的是到物体的最短距离，那向前走这么长的距离必然不会走过头，要么运气好正好到了物体表面，要么距物体还有一段距离。这种做法比前一种感觉上就要快不少，也是比较常见的选择。此外，使用 SDF 进行光线步进也可以方便地绘制 CSG（Construct Solid Geometry）或实现物体的碰撞或收缩。缺点是当光线与物体表面接近平行时，效果会比较差。

### 距离估计器

原文中的距离估计器（Distance Estimator，DE），或者可以称其为距离估计函数，是和 SDF 差不多的东西，如以下这个函数就可以作为球心在原点的球的 DE：

```c++
float de_sphere_1(float radius, Vec3f pos) {
    return pos.Length() - radius;
}
```

事实上，这就是球的 SDF。不过，以下两个函数也可以作为这样一个球体的 DE：

```c++
float de_sphere_2(float radius, Vec3f pos) {
    return std::max(pos.Length() - radius, 0.0f);
}

float de_sphere_3(float radius, Vec3f pos) {
    return std::abs(pos.Length() - radius);
}
```

它们在球体外侧和 SDF 相同，但内侧使用了不同的值。像是这样，我们可能还能定义出一些其他的 DE，不过，这些不同的 DE 会对我们的渲染结果产生怎样的影响呢，我们应该选用怎样的 DE 呢？

先抛开这个来看另一个问题，我们的光线步进只能找到光线与物体的交点，但为了应用各种光照模型，法线是必不可少的，我们该如何得到法线呢？根据 SDF 的定义，其梯度方向就代表着法线方向，而梯度可以通过如下方法进行近似计算：

```c++
Vec3f norm(
    SDF(pos.x() + EPS, pos.y(), pos.z()) - SDF(pos.x() - EPS, pos.y(), pos.z()),
    SDF(pos.x(), pos.y() + EPS, pos.z()) - SDF(pos.x(), pos.y() - EPS, pos.z()),
    SDF(pos.x(), pos.y(), pos.z() + EPS) - SDF(pos.x(), pos.y(), pos.z() - EPS));
norm.Normalize();
```

DE，如我刚刚所说，它和 SDF 差不多，以上给出的球体的三个 DE 在球体外侧也都与 SDF 的值相等，那一个很自然的想法就是也用类似的方法计算其近似梯度。`de_sphere_01()` 返回的就是 SDF，那肯定会得到正确的结果；`de_sphere_02()` 和 `de_sphere_03()` 在外侧使用 SDF，内侧使用其他定义，而因为计算法线时，加减 `EPS` 得到的 6 个邻近点一般会有一部分在内侧、一部分在外侧，这可能会使结果变得和我们期望的不同。

对于以上三种 DE，原文给出了相应的渲染结果，验证了这样的想法：

![deifs_01_de_and_normal.jpg](https://i.loli.net/2021/03/20/iLTyeGBuOMfwkzH.jpg)

图中最左侧的是用 `de_sphere_02()` 绘制的结果，可以看到表面有一圈圈圆环的伪像，是因为得到了错误的法线；中间是 `de_sphere_01()` 的绘制结果，是一个正确的结果；右侧使用 `de_sphere_03()` 并且在计算法线时，把点 `pos` 逆着光线方向回退了一小段距离，也得到了合理的结果，否则也会得到错误的结果。回退一段距离可以让 6 个邻近点都在外侧，都使用 SDF 的值。

所以，从法线计算的角度来讲，我们希望 DE 返回的值就是 SDF，不行的话至少外侧得是 SDF。我本人的理解就是，DE 是在 SDF 难以计算的情况下给出的一个更好计算的妥协版本，你要说它们就是同一个东西，我觉得也不是不可以。

### 加入到光线追踪器之中

根据以上的描述，我在自己的一个光线追踪器中派生出 `Fractal` 类（其实叫 `DEObject` 可能更合理一些），提供 `DistanceEstimate()` 纯虚函数，同时用其实现了 `Interset()` 函数，这样只要继承 `Fractal` 并给出相应的 DE 的实现，就可以渲染相应的物体。

```c++
// fractal.h
class Fractal : public Object3D {
public:
    Fractal(Material *material);

    virtual bool Intersect(const Ray &r, Hit &h, float tmin) const override;

    virtual float DistanceEstimate(Vec3f pos) const = 0;
};

// fractal.cpp
bool Fractal::Intersect(const Ray &r, Hit &h, float tmin) const {
    float dist_scale = r.getDirection().Length();
    float dist = tmin / dist_scale;
    float last_step = 0;
    for (int i = 0; i < FRACTAL_ITERATION; i++) {
        Vec3f p = r.pointAtParameter(dist * dist_scale);
        float step = DistanceEstimate(p);
        dist += step;
        float t = dist * dist_scale;
        if (t > tmin && step < FRACTAL_STOP_STEP && step < last_step) {
            if (t < h.getT()) {
                p = r.pointAtParameter(dist * dist_scale);
                float dx = DistanceEstimate(Vec3f(p.x() + FRACTAL_NORMAL_EPS, p.y(), p.z()))
                    - DistanceEstimate(Vec3f(p.x() - FRACTAL_NORMAL_EPS, p.y(), p.z()));
                float dy = DistanceEstimate(Vec3f(p.x(), p.y() + FRACTAL_NORMAL_EPS, p.z()))
                    - DistanceEstimate(Vec3f(p.x(), p.y() - FRACTAL_NORMAL_EPS, p.z()));
                float dz = DistanceEstimate(Vec3f(p.x(), p.y(), p.z() + FRACTAL_NORMAL_EPS))
                    - DistanceEstimate(Vec3f(p.x(), p.y(), p.z() - FRACTAL_NORMAL_EPS));
                Vec3f norm(dx, dy, dz);
                norm.Normalize();
                h.set(t, material, norm, r);
                return true;
            } else {
                break;
            }
        }
        last_step = step;
    }
    return false;
}
```

## Sierpinski 四面体的绘制

### Sierpinski 四面体的 DE

以下是 Wiki 上 Sierpinski 四面体的一个图片，图中有红蓝两个四面体，并且相交了一部分。单看一个四面体，观察其构造方法：我们可以把一个完整的四面体缩小到原来的一半，复制到四份，分别向四个顶点的方向平移一段距离，反复此操作多次可以得到一个这样的形状，接下来，我们就要利用这个构造方法来写出 Sierpinski 四面体的 DE。

<img src="https://upload.wikimedia.org/wikipedia/commons/b/b4/Sierpinski_pyramid.png" style="zoom:20%;" />

方便起见，我们把初始的那个完整四面体的四个顶点分别放在 $(1, 1, 1)$、$(-1, -1, 1)$、$(1, -1, -1)$、$(-1, 1, -1)$ 的位置上，其他位置的 Sierpinski 四面体可以通过旋转平移缩放得到，在光线追踪器中已经实现了，所以这样就够了。

计算这四个顶点与当前询问点的距离，很明显，询问点与哪个顶点更近，它就与一次操作后的四个小四面体中的哪一个更近。

```c++
Vec3f min_point(1, 1, 1);
float min_dist2 = (min_point - pos).LengthSqr();

float dist2_temp = (pos - Vec3f(-1, -1, 1)).LengthSqr();
if (dist2_temp < min_dist2) {
    min_point.Set(-1, -1, 1);
    min_dist2 = dist2_temp;
}

dist2_temp = (pos - Vec3f(1, -1, -1)).LengthSqr();
if (dist2_temp < min_dist2) {
    min_point.Set(1, -1, -1);
    min_dist2 = dist2_temp;
}

dist2_temp = (pos - Vec3f(-1, 1, -1)).LengthSqr();
if (dist2_temp < min_dist2) {
    min_point.Set(-1, 1, -1);
    min_dist2 = dist2_temp;
}
```

这样找到最近的一个小四面体后，我们就需要继续考虑询问点与小四面体的关系，借助分形的递归性质，我们可以重复利用以上代码。具体地说，对小四面体与询问点反向操作一倍，即放大到 2 倍，并平移使得放大后的小四面体与原四面体重合。要注意的是，我们并不需要真的对什么几何体做一次变换，实际要变换的只有询问点，只是下一次迭代重复使用以上代码的化，就相当于对小几何体做了这样的变换。

```c++
pos = 2 * pos - min_point;
```

反复以上操作 $n$ 次，最后得到的 `pos` 的模长除以 $2^n$（因为每次操作都会乘 2），就是原始询问点到与之最近的递归 $n$ 次后的小四面体中心的距离。我们把该距离作为 DE 的值。完整的 DE 如下：

```c++
float SierpinskiTetrahedron::DistanceEstimate(Vec3f pos) const {
    for (int i = 0; i < iteration; i++) {
        Vec3f min_point(1, 1, 1);
        float min_dist2 = (min_point - pos).LengthSqr();
        
        float dist2_temp = (pos - Vec3f(-1, -1, 1)).LengthSqr();
        if (dist2_temp < min_dist2) {
            min_point.Set(-1, -1, 1);
            min_dist2 = dist2_temp;
        }
        
        dist2_temp = (pos - Vec3f(1, -1, -1)).LengthSqr();
        if (dist2_temp < min_dist2) {
            min_point.Set(1, -1, -1);
            min_dist2 = dist2_temp;
        }
        
        dist2_temp = (pos - Vec3f(-1, 1, -1)).LengthSqr();
        if (dist2_temp < min_dist2) {
            min_point.Set(-1, 1, -1);
            min_dist2 = dist2_temp;
        }
        
        pos = 2 * pos - min_point;
    }
    
    return pos.Length() * std::exp2(-iteration);
}
```

因为这样得到是到四面体中心的距离，当重复次数较多时还好，重复次数较少时只会得到一个个孤立的点，看不出一个完整的连续的形状，我对其做了一个小的修改：最后不取到中心的距离，而是取到小四面体的包围球的距离，即把最后一行改为：

```c++
return (pos.Length() - std::sqrt(3.0f)) * std::exp2(-iteration);
```

这样，我们就可以绘制 Sierpinski 四面体了。下图是迭代次数从 0 到 6 的绘制结果。

![deifs_02_sierpinski.png](https://i.loli.net/2021/03/20/7HmYJRL1h4pPfDE.png)

图中也可以看到，就算使用到包围球距离，效果也不是特别的好（但比起不连续的点要好多了），效果最好的应该是直接计算到四面体的距离，但毕竟算到球体的距离要简单很多。

### 利用对称性

在以上的算法中，我们一次迭代需要计算 4 次点积（向量模长的平方），考虑到 Sierpinski 四面体的对称性，我们还可以做得更快一些。下图是原文中的一张图，描绘了 Sierpinski 四面体的对称性。

![](http://blog.hvidtfeldts.net/media/illu.png)

我们的目标是，利用对称性，把各种情况都转化为与 $(1, 1, 1)$ 这个顶点计算，假设上图中上侧的红色顶点就是 $(1, 1, 1)$，如果询问点在白色区域，那无需做任何操作。如果询问点在红色镜子的另一侧（图中的红色部分即青色部分，以及看不见的背面的这两个部分），将其沿红色镜子对称过去，蓝色镜子与绿色镜子同理。这三面镜子的平面方程都比较特殊，分别是 $x + y = 0$、$y + z = 0$、$z + x = 0$，这使得沿平面做对称的操作变得十分简单。于是，利用对称性优化后的 DE 就是：

```c++
float SierpinskiTetrahedron::DistanceEstimate(Vec3f pos) const {
    for (int i = 0; i < iteration; i++) {
        if (pos.x() + pos.y() < 0) {
            pos.Set(-pos.y(), -pos.x(), pos.z());
        }
        if (pos.y() + pos.z() < 0) {
            pos.Set(pos.x(), -pos.z(), -pos.y());
        }
        if (pos.z() + pos.x() < 0) {
            pos.Set(-pos.z(), pos.y(), -pos.x());
        }

        pos = 2.0f * pos - Vec3f(1, 1, 1);
    }

    return (pos.Length() - SQRT_3) * std::exp2(-iteration);
}
```

此处再放上一张本人渲染的尺寸大一些、带上阴影的 Sierpinski 四面体：

![deifs_03_sierpinski.png](https://i.loli.net/2021/03/20/YNDcRKtufWh9JTs.png)

## Cantor 立方体的绘制

现在我们已经知道了 Sierpinski 四面体的构造方法，现在我们再尝试写一个简单的分形的 DE。

下图是网络上一张 Cantor 立方体的图片。观察其构造方法：把一个正方体按 $3 \times 3 \times 3$ 切开，保留 8 个角落处的小正方体，反复操作多次可以得到这样的形状。

![](http://blogs.ams.org/visualinsight/files/2013/11/cantors_cube.jpg)

这次我们直接利用对称性写 DE。假设原始的正方体就是一个中心在原点、边长为 2 的正方体，那么 3 个对称平面就是 $x = 0$、$y = 0$、$z = 0$，我们将 8 个小正方体都对称到 $(1, 1, 1)$ 顶点这边的这个小正方体上，再类比写出反向操作一步的变换式，就可以得出以下的 DE：

```c++
float CantorCube::DistanceEstimate(Vec3f pos) const {
    for (int i = 0; i < iteration; i++) {
        if (pos.x() < 0) {
            pos.Set(-pos.x(), pos.y(), pos.z());
        }
        if (pos.y() < 0) {
            pos.Set(pos.x(), -pos.y(), pos.z());
        }
        if (pos.z() < 0) {
            pos.Set(pos.x(), pos.y(), -pos.z());
        }

        pos = 3.0f * pos - 2.0f * Vec3f(1, 1, 1);
    }

    return (pos.Length() - SQRT_3) * std::pow(3.0f, -iteration);
}
```

下图是迭代次数从 0 到 5 的绘制结果。（相比 Sierpinski 四面体的场景，多打了一个水平向前的定向灯）

![deifs_04_cantor.png](https://i.loli.net/2021/03/20/iDeCBwVjIAz8fQJ.png)

在最后的一张图中，立方体的远处就像是溶解了一般，是因为 `Intersect()` 中步进的次数设置的小了一点。

同样的，我也渲染了一张大一点的、有阴影的 Cantor 立方体

![deifs_05_cantor.png](https://i.loli.net/2021/03/20/wa8D5O4oBdJxlcG.png)

## 任意 3D IFS 的绘制

### IFS

绘制过 Sierpinski 四面体与 Cantor 立方体后，我们已经掌握了 3D 分形绘制的基本方法了，但是，我们显然不可能为每一种可能的分形都创建一个子类出来，更何况有一些分形的 DE 并不是那么容易手推出来。

迭代函数系统（Iterated Function System，IFS）是一种构成分形的方法。简单来说，它用若干变化矩阵来描述分形的构成方式，就像是把我们在介绍 Sierpinski 四面体与 Cantor 立方体时用自然语言描述的构造方法形式化地表示了出来一样。比如说 Sierpinski 四面体，它的构造方法可以用以下这样四个矩阵表示：
$$
\begin{pmatrix}
0.5 & 0 & 0 & 0.5 \\
0 & 0.5 & 0 & 0.5 \\
0 & 0 & 0.5 & 0.5 \\
0 & 0 & 0 & 1 \\
\end{pmatrix}
\begin{pmatrix}
0.5 & 0 & 0 & -0.5 \\
0 & 0.5 & 0 & -0.5 \\
0 & 0 & 0.5 & 0.5 \\
0 & 0 & 0 & 1 \\
\end{pmatrix}
\begin{pmatrix}
0.5 & 0 & 0 & 0.5 \\
0 & 0.5 & 0 & -0.5 \\
0 & 0 & 0.5 & -0.5 \\
0 & 0 & 0 & 1 \\
\end{pmatrix}
\begin{pmatrix}
0.5 & 0 & 0 & -0.5 \\
0 & 0.5 & 0 & 0.5 \\
0 & 0 & 0.5 & -0.5 \\
0 & 0 & 0 & 1 \\
\end{pmatrix}
$$
（我们假设初始几何体的中心在坐标原点）

### DE 的计算

所以我们可以如此定义一个 `Ifs3D` 的子类：

```c++
class Ifs3D : public Fractal {
public:
    Ifs3D(int iteration, float radius, const std::vector<Matrix> &transforms, Material *material);

    virtual float DistanceEstimate(Vec3f pos) const override;

private:
    int iteration;
    float radius;
    std::vector<Matrix> transforms;
    std::vector<Matrix> inv_transforms;
};
```

因为是任意的 3D 分形，所以我们不再能利用对称性，又回到了一开始计算 Sierpinski 四面体 DE 的方法上，每一次迭代，其步骤可以大致概括为：

* 对于一次分形构造产生的几个小几何体，计算出距离，找到最近的一个
* 对找到的小几何体和询问点反向做一次操作

在第一步中，有两点需要注意：

* 在之前两个分形的绘制中，我们计算的是和顶点之间的距离，只是因为它们产生的小几何体正好一个顶点一个。要推广的话，要计算的应该是变换后的小几何体中心（即变换后的原点）之间的距离
* 找最近的小几何体时，我们想找的不一定是在当前空间中最近的一个，而是假设这个小几何体就是最近的，反向做一次操作后，询问点到原点（小几何体中心）的距离最近的一个。区别在于，如果几个变换矩阵的缩放系数不同，比如一个缩小为 0.8 而另一个缩小为 0.2，在这两个变换矩阵对应得到的小几何体中心的连线上，有八成的部分应该属于 0.8 这边的小几何体，只有剩下二成属于 0.2 这边的小几何体。对询问点做一次相应的逆变换可以解决这个问题

在原来的代码中，我们最后使用了最后得到询问点的模长除以了 $2^n$ 或 $3^n$，2 和 3 是逆变换矩阵中的缩放系数，在这里，我们不同的变换矩阵可能缩放系数不同，甚至可能会是不均等缩放，所以我们不能简单地计算最终结果。我们原来的除法，其实相当于把几次迭代下来的逆变换，反向做了一遍正变换，形式化地说，我们原来是 $p' = M^{-1}_n M^{-1}_{n - 1} \cdots M^{-1}_1 p$，得到在最后一个小几何体相对空间下的坐标，反过来就是 $p = M_1 M_2 \cdots M_n p'$ 再回到初始坐标系的的坐标，不过因为我们这一步操作是为了求出距离，相当于我们在变换一个向量而非点，平移操作不会造成影响，所以解出来的结果和一开始的输出不同（虽然刚刚都用 $p$ 来表示了）。

这样我们就可以得到整个 DE 的代码（代码中增加了取到包围球距离而非到中心距离的部分）：

```c++
float Ifs3D::DistanceEstimate(Vec3f pos) const {
    std::vector<int> min_indices(iteration);
    Vec3f original_pos = pos;

    for (int i = 0; i < iteration; i++) {
        int min_index = -1;
        float min_dist2 = std::numeric_limits<float>::max();

        Vec3f next_pos;
        for (int j = 0; j < transforms.size(); j++) {
            Vec3f p = pos;
            inv_transforms[j].Transform(p);
            float dist2 = p.Dot3(p);
            if (min_index == -1 || dist2 < min_dist2) {
                min_dist2 = dist2;
                min_index = j;
                next_pos = p;
            }
        }

        min_indices[i] = min_index;
        pos = next_pos;
    }

    Vec3f temp = pos;
    temp.Normalize();
    bool is_negative = pos.Dot3(pos) <= radius * radius;
    Vec3f p = pos - temp * radius;
    for (int i = iteration - 1; i >= 0; i--) {
        transforms[min_indices[i]].TransformDirection(p);
    }
    float dist = p.Length();
    return is_negative ? -dist : dist;
}
```

要注意的是最后这段符号的判断，如果直接返回不做处理的结果，相当于使用 `abs(pos.Length() - radius)` 作为球的 DE，要么法线计算有问题，要么需要在计算法线时做回退操作。

### 以 Menger 海绵为例

Menger 海绵的单词构造方法是，把一个正方体做 $3 \times 3 \times 3$ 的划分，扔掉体心的一块以及六个面心的小块，保留剩下 20 小块，其变换矩阵可以用如下的代码生成：

```c++
std::vector<Matrix> transforms;
for (int i = -1; i <= 1; i++) {
    for (int j = -1; j <= 1; j++) {
        for (int k = -1; k <= 1; k++) {
            if (std::abs(i) + std::abs(j) + std::abs(k) <= 1) {
                continue;
            }
            Matrix m;
            m.SetToIdentity();
            // scale
            m.Set(0, 0, 1.0f / 3.0/f);
            m.Set(1, 1, 1.0f / 3.0/f);
            m.Set(2, 2, 1.0f / 3.0/f);
            // translate
            m.Set(0, 3, i);
            m.Set(1, 3, j);
            m.Set(2, 3, k);
            transforms.push_back(m);
        }
    }
}
```

依旧是给出迭代次数从 0 开始递增的结果和一个大尺寸带阴影的渲染结果（小尺寸绘制结果也加上了阴影，不然一片白看不清结构）：

![deifs_06_menger.png](https://i.loli.net/2021/03/20/8bh9BkKcGodxJqp.png)

![deifs_07_menger.png](https://i.loli.net/2021/03/20/Ecxa2wkZitI47mq.png)