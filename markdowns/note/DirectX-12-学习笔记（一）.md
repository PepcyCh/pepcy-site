---
title: DirectX 12 学习笔记（一）
date: 2020-08-04 16:14:53
tags: [DirectX12, CG, 学习笔记]
categories: 学习笔记（CG）
---

因为觉着 DirectX 12 和 Vulkan（还有 Metal）作为所谓现代的图形 API，有必要依次学一学，因为之前学的是 OpenGL 3.3。

总之先从 DirectX 12 开始，用的教程是 Frank Luna 的书《Introduction to 3D Game Programming with DirectX12》，没有中文的样子，搞到的电子版有 1200 页。。。有三部分，第一部分（前三章）是数学，跳过了；第二部分（4~14 章）是 DirectX 12 基础，也就是本笔记时看过去的部分；第三部分（15~23 章）算是专题，比如法线贴图、第一人称摄像机、AO 等，之后会继续看下去。

书有一个配套的代码仓库：[d3dcoder/d3d12book](https://github.com/d3dcoder/d3d12book)

我也是跟着书和这个仓库每章也写了，大部分是一样的，但也改了一些部分，比如 DDS 材质的加载使用 DirectXTK12 库，而书中是作者自己魔改的 DirectXTK11。另一点，处于个人喜好，没有用 Visual Studio 的项目，还是用的 VS Code + CMake。代码仓库是：[PepcyCh/learn-directx12](https://github.com/PepcyCh/learn-directx12)

此外，在知乎上看到了一个 ID 为「[卡卡](https://www.zhihu.com/people/qia-qia-80-99-20)」的人也在跟这本书，并且会用心做每一个练习（相比之下我就太囫囵了。。。），安利一下。

（这可能并不是一篇看能就能够入门的笔记，只是我来列出原书第二部分的学习后，对整个部分一个总结概括性质的个人笔记）

<!-- more -->

## 每章代码简介

### chap. 4 - initialization

Windows 窗口及 DirectX 12 必要的初始化，程序运行得到的是一个能显示 FPS 以外只有纯色背景的窗口。

### chap. 6 - drawing I

DirectX 12 绘制最基本的内容，绘制一个彩色立方体。

`ch06_box` 是跟着原书和仓库来的，`ch06_box_extra` 是书上这一章的习题 2，并且从原来的左手系改成了自己更习惯的右手系。

### chap. 7 - drawing II

引入了「帧资源（Frame Resource）」与「渲染物体（Render Item）」进行渲染，同时向 `Common` 内加入了一个 `GeometryGenerator` 来生成立方体、圆柱（圆台）、经纬球、几何球等集合体。

`ch07_shape` 是绘制 Geometry Generator 生成的几何体（把一侧的经纬球改成了几何球），并通过「1」键开启或关闭线框模式。

`ch07_land_wave` 是以平面为基础绘制「山川河流」，大部分后续代码也是由此改来。

### chap. 8 - lighting

使用 Blinn-Phong 光照绘制山川河流的例子。这个 Blinn-Phong 和当时在 Learn OpenGL 学的有所区别，用到的参数分别是漫反射（diffuse）、环境光（ambient）、粗糙度（roughness）和正射时的菲涅尔系数。

### chap. 9 - texturing

在山川河流的例子中加入材质，使用 DDS 材质。

### chap. 10 - blending

在山川河流的例子中，使用混合绘制半透明的水。

### chap. 11 - stenciling

使用模板来模拟镜子的效果，加入了练习中的地板镜像。同时实现了 Learn OpenGL 那边讲过的用模板实现的物体轮廓，通过「1」键来开启或关闭。

### chap. 12 - geometry shader

在山川河流的例子中，使用几何着色器实现的告示板（Billboard）技术来绘制树木。

### chap. 13 - compute shader

`ch13_wave` 是在山川河流的例子中，使用计算着色器更新水面的水波。

`ch13_blur` 是通过计算着色器对山川河流的渲染结果进行高斯模糊。

`ch13_sobel` 是使用 Sobel 算子对山川河流的渲染结果进行边缘检测，并通过一次渲染屏幕四边形来为渲染结果加入描边的效果。我在代码中结合了光照章节中的练习 6（一种卡通渲染方法）。

### chap. 14 - tessellation

`ch14_tessellation_basic` 是通过细分来生成山川河流中的山的顶点，根据到观察点的距离决定细分系数以实现一种 LOD。

`ch14_tessellation_bezier` 通过细分生成贝塞尔曲面。

## Win32 窗口

通过一个 `WNDCLASS` 对象来设定窗口的一些特性（如图标、光标样式、事件处理函数）并通过 `RegisterClass()` 来注册这一类窗口；然后通过 `CreateWindow()` 来创建真正的窗口。

其中，这个事件处理函数的签名大致如下 `LRESULT CALLBACK (HWND win, UINT msg, WPARAM w_param, LPARAM l_param)`，其中 `HWND win` 就是窗口的句柄（指针），虽然我们把它存了起来，但还是需要以参数的形式提供在其中，可能会觉得有些没有必要。其实，在 `CreateWindow()` 的期间就会触发事件处理函数，而我们存的句柄来自 `CreateWindow()` 的返回值，这时还是无效的，所以需要把那个有效的句柄以参数的形式传入。在编写具体函数体的时候也要注意要使用参数中的句柄而不是存储的变量。

## DirectX 12 渲染管线

DirectX 12 一次渲染经过一下步骤：

* Input Assemble（IA）
* Vertex Shader（VS）
* Hull Shader（HS）
* Tessellation
* Domain Shader（DS）
* Geometry Shader（GS）
* Rasterizer（RS）
* Pixel Shader（PS）
* Output Merger（OM）

其中 IA、Tessellation 和 RS 是不可编程的。此外，还有一个可单独使用的 Computer Shader（CS），不在渲染管线之中。

## DirectX 12 的元素

### `ComPtr<T>`

DirectX 12 中的许多对象都是通过一个工厂方法来创建，而这些对象都是 Windows 的 COM（Component Object Model）对象，要通过智能指针的方式管理这些对象就得用 WRL（Windows Runtime Library）的 `ComPtr<T>`，它会在最后调用 COM 对象具体子类实现的一个释放对象的函数。如果把非 COM 的类作为模板参数给 `ComPtr<T>`，就会报错说 `T` 没有那个释放函数（当然，你按着那个签名实现一个也成吧……）；如果把 COM 的类用 C++ 自己的智能指针管理，可能会有问题（[Using std::unique_ptr for managing COM objects](https://stackoverflow.com/questions/21820301/using-stdunique-ptr-for-managing-com-objects)）。

这些 COM 对象的基类可以根据类名的前缀分为 2 种：

* `IDXGIXxx` ：DXGI（DirectX Graphics Interface），一些各个 DirectX 版本、无论 2D、3D 都会用到的东西。
* `ID3D12Xxx` ：DirectX 12 自己的东西。

### D3D12 调试层

对应的接口基类为 `ID3D12Debug`，通过开启调试层，可以在调试模式下（如 `gdb` 中）看到更详细的错误或警告信息，方便调试。

### `IDXGIFactory` & `ID3D12Device`

这两个类是创建 DXGI / DirectX 12 对象的工厂。一般我们在创建完窗口后就会创建这两个对象。

*PS：代码中实际使用的基类是 `IDXGIFactory4`。*

### `D3D12_XXX_DESC`

在使用 `ID3D12Device` 对象的工厂方法创建对象时，经常需要填写一个 `D3D12_XXX_DESC` 对象并作为参数传给工厂方法。这些对象中记录了创建对象时的一些配置。为了方便编程，微软提供了 `d3dx12.h` 的头文件（不自带，需要去 GitHub 之类的地方搞一份），一些比较复杂的类有相应的 `CD3DX12_XXX_DESC` 类，它们继承自相应的 `D3D12_XXX_DESC` 类，没有新增任何成员变量，但新增了一些方便方法，以简便地填写一个对象。

除了以 `_DESC` 结尾地类，还有一些其他的东西，如 `D3D12_RESOURCE_BARRIER` 有着类似的情况，只不过不是用于创建对象。

### Swap Chain

一般我们渲染时为了使画面稳定，显示的缓冲和绘制目标会分开来，等绘制完毕后，让它去显示，而原来显示的缓冲成为下一帧绘制的目标。这样的机制成为交换链（Swap Chain，对应的接口基类为 `IDXGISwapChain`），一般会使用 2 个或 3 个缓冲。

在相应的配置类 `DXGI_SWAP_CHAIN_DESC` 中，有一项 `SwapEffect` 应当设置为 `DXGI_SWAP_EFFECT_FLIP_DISCARD` 或 `DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL`，这个时候，尝试建立一个 MSAA 的交换链会 RE，所以，原书及相应仓库中的 MSAA 代码是不能跑的。

*PS：代码中实际使用的基类是 `IDXGISwapChain1` 和 `DXGI_SWAP_CHAIN_DESC1`。*

### Command Allocator、Queue and List

在 DirectX 12 中，并不是 CPU 提交指令给 GPU 让它直接执行，然后等 GPU 执行完毕后再运行 CPU 侧的下一条指令，而是先把一些命令存起来，然后再一起交给 GPU 让它去执行，并且 CPU 可以继续执行后续的代码。

这些命令由命令列表（Command List，对应的接口基类为 `ID3D12CommandList`）对象的方法添加到列表之中；而列表使用的空间来自命令分配器（Command Allocator，对应的接口基类为 `ID3D12CommandAllocator`）；命令队列（Command Queue，对应的接口基类为 `ID3D12CommandQueue`）则负责把列表里的命令提交给 GPU，也负责一些 CPU 与 GPU 的同步。

使用列表的方法添加命令时，必须先调用一次 `Reset()`，其中还会指明使用的分配器和 PSO（之后会说），等命令添加完了，调用一次 `Close()`，之后才能让队列去交给 GPU 执行。

因为 CPU 与 GPU 是并行的，CPU 在运行后续代码时可能 GPU 还在执行列表里的命令，这里就要保障分配器对象不变（因为命令的数据所占的空间由分配器管着）。

### Fence

CPU 与 GPU 是并行的，因此需要一个机制来让 CPU 知道 GPU 有没有完成某些工作，已让 CPU 部分可以安全地继续执行下去。

Fence 可以认为是插入到命令之间的一个整数，一般我们会让这个整数随插入次数每次加一，我们可以通过最近见到的一个 Fence 的数字是多少来判断它某个 Fence 之前的命令有没有全部完成。

一般，我们会在每一帧的结束，命令队列把命令提交后，使用队列的 `Signal()` 方法将 Fence 插入。根据 Fence 等待命令执行完的操作被称为刷新队列（flush command queue）。

我们需要一个 Fence 对象（对应的接口基类为 `ID3D12Fence`），它会记录最近一个 Fence 的数字。

### 资源、描述子、描述子堆

DirectX 12 中，资源（Resource，相应的接口基类为 `ID3D12Resource`）大致可以分为两大类，缓冲（buffer）和材质（texture）。

资源通过描述子（Descriptor，因历史原因也叫 View）来描述，一个资源上可以有多个描述子。描述有三（五）类，分别是：

* RTV（Render Target View）- 渲染目标
* DSV（Depth Stencil View）- 深度与模板
* CBV（Constant Buffer View）、SRV（Shader Resource View）、UAV（Unordered Access View）
  * CBV - 类似于 OpenGL 的 uniform，在一次着色器程序运行中不变的数据，如光源、变换矩阵等
  * SRV - 对着色器程序来说只读的资源，如漫反射贴图、法线贴图等
  * UAV - 对着色器程序来说可读写的资源，如计算着色器要写入的目标

描述子存储在描述子堆（Descriptor Heap）中，一个堆更类似于一个定长数组，在创建时需要指定大小。一个堆中只能存储同一类的描述子（CBV、SRV、UAV 可以在同一个堆中）。在堆中创建描述子时，需要给出堆的 CPU 句柄（CPU Descriptor Handle）；而 GPU 使用其中的描述子时，需要的是 GPU 句柄（GPU Descriptor Handle）。有时，可以直接把资源的 GPU 虚拟地址（GPU Virtual Address）让 GPU 访问。

在创建资源时，需要传入一个 `D3D12_HEAP_PROPERTIES` 的对象，关于这一块内容的具体描述，可以看 [Walking through the heap properties in DirectX 12](https://zhangdoa.com/posts/walking-through-the-heap-properties-in-directx-12)。

### 资源状态 & Barrier

我们可能会让一个着色器程序先写入一个材质，再让另一个着色器程序读取写入的数据（这是非常常见的场景），在第二个着色器程序开始之前，必须要保证第一个着色器程序已经把材质写好。我们确实可以通过 flush 的方法先等待第一个操作完毕再运行第二个着色器，不过，在更复杂的场景中，这样做一是人自己要 flush 会比较麻烦，二是有些对材质的操作其实是可以并行的，我们 flush 还会减慢运行速度，这时就需要一个新的机制来处理同步的问题，就是 Barrier。

每个资源都会有一个资源状态（Resource State），在资源需要切换状态时，需要插入一个对应的状态转换 Barrier，它会根据资源的前后状态来考虑需要以怎样的方式来同步，而且只会影响使用该资源的命令。

关于 Barrier，有一系列比较好的文章，现在在知乎上有人在翻译：

[【译】拆解D3D12和Vulkan中的Barrier（1）](https://zhuanlan.zhihu.com/p/164491130)（后续几篇的链接就不贴出来了，毕竟挺多的……）

资源状态不会影响命令的执行与否，比如，对只读资源使用有写入的命令也是可以执行的，只是同步上会有一些问题。

一些比较常用的状态有：Common、Generic Read、Unordered Access、Render Target、Copy Source/Destination、Resolve Source/Destination 等。虽然代码中经常有转换到 Generic Read 的代码，但官方文档上似乎并不推荐这样做。

### Vertex Buffer & Index Buffer

DirectX 12 中，顶点与索引的缓冲也是一种资源，它们的描述子类型是 VBV（Vertex Buffer View）与 IBV（Index Buffer View），不过这两个描述子与之前提到的五种不他一样，VBV 与 IBV 不存在描述子堆中，也不是通过 GPU 句柄访问，而是直接把 `D3D12_VERTEX_BUFFER_VIEW` 与 `D3D12_INDEX_BUFFER_VIEW` 对象传给命令列表的 `DrawInstanced()` 或 `DrawIndexedInstanced()` 中。

在 `DrawIndexedInstanced()` 中，还有两个参数 `StartIndexLocation` 和 `BaseVertexLocation`，前者指 Index Buffer 中起始的位置，后者指索引中的每一项要加上的一个数值（比如把多个小的几何体的 Vertex/Index Buffer 拼成一个时，需要设置这个值）。

### 着色器（Shader）

不像 OpenGL，可以把 VS、PS 等全都写在同一个 HLSL 文件内，DirectX 12 提供的函数可以传入各自的入口函数；此外，也可以通过函数传入宏定义。比起 GLSL，HLSL 的感觉更像 C++ 一些，不过矩阵乘法要用 `mul()`，使用 `*` 表示的则是分量乘法，这一点稍微有点难受。

着色器程序的一些输入输出会有一个语义名（Semantic Name），其中一些有特殊意义的语义名均以 `SV_` 开头。当我们把 VS 与 PS 分开写的时候，HLSL 通过语义名来判断哪两个变量是同一个，即使它们的变量名不同，不像 GLSL 中使用变量名来判断。

### Input Layout

VS 需要知道 Vertex Buffer 中每个顶点中每一项的语义名、大小、位置、步长（stride），描述这些东西的叫做输入布局（Input Layout），有点类似于 OpenGL 的 Vertex Attribute。

在输入布局的参数中，还有一个指定输入槽（slot），一个般一个槽对应一个 Vertex Buffer，比如位置一个缓冲、法线另一个缓冲，把它们放在两个槽上。原书代码中基本上只是用一个槽，我也只在其中一个练习中使用了多于一个槽。

### 根签名 & CBV、SRV、UAV、Static Sampler

如在描述子那里说的那样，通过 CBV 向着色器传递诸如光源、变换矩阵之类的东西，通过 SRV 向着色器传递只读材质，通过 UAV 传入可写材质。在着色器程序中，还需要为它们指明使用的寄存器，CBV 使用 `bx`（指 `b0`、`b1`……这样，后同），SRV 使用 `tx`，UAV 使用 `ux`。此外，还有一个称作静态采样器（Static Sampler）的东西，使用寄存器 `sx`，它的作用是采样材质。与 OpenGL 不同，OpenGL 把线性插值还是最邻近采样、越界处理放在对材质的设置中，DirectX 12 则把它们单独出来成为采样器。

使用着色器时，需要有一个东西说明我们使用了几个 CBV/SRV/UAV/Static Sampler，这样的东西称为根签名（Root Signature）。一个根签名通过一个根参数（Root Parameter）的数组和可选的静态采样器的数组创建，根参数数组则依次描述使用的 CBV/SRV/UAV，每一个根参数可以是：

* 描述子表（Descriptor Table）- 连续的一段同一类描述子的寄存器，比如 `b0`-`b2`、`u1`-`u1`（也就是只有一个），通过描述子堆的 GPU 句柄传入，着色器需要经历“句柄-地址-资源”的步骤访问到资源，但只占用 32bit。
* 根描述子（Root Descriptor）- 单个 CBV/SRV/UAV，通过资源的虚拟地址传入，着色器需要经历“地址-资源”的步骤访问到资源，占用 64bit，不过只能以此传入缓冲（buffer），不能传入材质（texture）。
* 根常数（Root Constant）- 一个或多个 32bit 常数，着色器能直接访问到数据，每有一个常数就占用 32bit。

在实际中，要同时考虑着色器访问速度和占用大小来设置。另外，还要考虑更新的频率，一般来说，把更新频率更高的东西要放到根参数数组中更前面的位置上，比如每个物体都不同数据（如模型矩阵、材质（material）等）放在每一帧内都一样的数据（如光源、透视矩阵）之前，把它们分在不同的 Constant Buffer 之中。

着色器中，一个 Constant Buffer 中的数据，以及一个结构体中的数据，在内存排布上遵循以下原则：

* 128bit 一组
* 属于同一个向量的数组不拆开

比如

```hlsl
cbuffer cb : register(b0) {
    float3 v1;
    float3 v2;
    float s1;
    float s2
}
```

在内存中的排布如下：

```plain
| v1 | v1 | v1 | xx |
| v2 | v2 | v2 | xx |
| s1 | s2 | xx | xx |
```

而

```hlsl
cbuffer cb : register(b0) {
    float3 v1;
    float s1;
    float3 v2;
    float s2
}
```

的内存排布是：

```plain
| v1 | v1 | v1 | s1 |
| v2 | v2 | v2 | s2 |
```

这会带来两个问题，一是要考虑使用空间大小，二是要考虑 C++ 代码中要传给这个 Constant Buffer 的结构体的内存布局要一样（比如，不得不插入空隙时，加入一个 `float _padding` 字段）。

### Pipeline State Object（PSO）

在管线状态对象（PSO）中，我们指明绘制要使用的着色器、深度与模板函数、混合函数、背面剔除与正面旋转方向、填充方式（常规还是线框）等。

区分绘制用的 Graphics PSO 和计算着色器的 Compute PSO。

## DirectX 12 相关的其他内容

### 帧资源（Frame Resource）

一帧 flush 一次的做法显然完全没有利用 CPU 与 GPU 的并行性，它还是先 CPU 工作一段，然后 GPU 工作一段，以此交替地工作下去。我们希望这一帧的命令在执行时，下一帧的 CPU 工作也能进行，但由于每一帧的 CPU 工作会修改着色器使用的 Constant Buffer 和命令分配器里的命令，会影响正在执行的 GPU 工作，为了做到并行，我们把这些东西存储多份，每一份就被称作一个帧资源。

如果我们有 $k$ 份帧资源，那么 GPU 就可以与接下来 $k - 1$ 帧的 CPU 并行，如果两者的工作时间差不多，那我们就做到了最优的并行。在原书代码中，选用了 $k = 3$。

为了在数据发生修改时，新数据能够写入每一份帧资源的 Constant Buffer 中，我们在原始数据的结构中加入一个 dirty 字段，当修改时，赋值为 $k$，写入后字段自减一。这样不仅能保证一次修改的数据被写入，在每一帧都修改的情况下也会得到正确的结果。

### 单帧绘制流程

1. 切换到下一个帧资源，并等待该帧资源的上次绘制结束
2. 动画的更新，更新各个 Constant Buffer
3. Reset 当前帧资源的命令分配器，Reset 命令列表（设置 PSO）
4. 设置视口（viewport）和裁剪（scissors rectangle）
5. 清空 RTV & DSV，指明渲染的 RTV & DSV
6. 设置根签名
7. 设置 CBV/SRV/UAV 的描述子堆
8. 设置一帧渲染中不变的 CBV
9. 渲染每个物体
   1. 设置物体的 CBV、物体材质的 CBV、SRV
   2. 绘制
10. 调用命令列表的 Close，用命令队列提交命令让 GPU 开始运行
11. 设置 Fence
12. 交换链切换

### 计算着色器

在计算着色器程序中，需要声明属性 `[numthreads(x, y, z)]`，表示这么大的一块作为一组线程组（Thread Group），同一个线程组的线程并行，且可以共享着色器程序中标有 `groupshared` 的数据，可以在程序中通过 `GroupMemoryBarrierWithGroupSync()` 来同步，不同线程组之间则不行。一般来说，线程组的大小应该是一个数的倍数（根据原书上的描述，NVIDIA 显卡是 32 的倍数，AMD 显卡是 64 的倍数）才能达到更好的效率。

在计算着色器中，越界的部分写入无效，读取则读取到 0。

### 细分

细分包括可编程的 HS 和 DS，以及固定的 Tessellation 步骤。

要使用细分，要绘制的图元的图元类型得是 `D3D_PRIMITIVE_TOPOLOGY_X_CONTROL_POINT_PATCHLIST`（其中 `X` 取 1 - 32，表示控制点的数目），PSO 的图元类型是 `D3D12_PRIMITIVE_TOPOLOGY_TYPE_PATCH`。

HS 包括一个 Constant HS 和一个 Control Points HS，在 C++ 代码中指明的入口函数是 Control Points HS 的函数名，Constant HS 的函数名则通过 Control Points HS 的属性 `[patchconstantfunc()]` 来指明。

Control Points HS 和 DS 都有一个属性 `[domain()]`，其可能的参数是 `isoline`、`triangle` 或 `quad `，关于这三种 patch 如何被 Constant HS 输出的两类细分因子影响，可以看 [Tessellation - OpenGL Wiki](https://www.khronos.org/opengl/wiki/Tessellation)（虽然是 OpenGL 的，但大家都是一样的，而且我觉得这个里面写的挺全的……）。

Control Points HS 的目的是修改传入的控制点，也可以直接不变地输出。

DS 可以看作是细分后顶点的顶点着色器，使用细分的话，透视矩阵在这里乘，DS 的输出作为 PS 的输入。

DS 的输入不是具体的细分后顶点，而是一个跟据 patch 不同的参数（四边形的话就是 uv，三角形就是重心坐标）。需要我们手动根据输入的顶点来算出真正的细分后顶点（可以认为，Tessellation 步骤不知道我们的控制顶点是什么，只是在细分一个标准的线/三角形/正方形，所以也只能给出参数）。

## 代码中的其他内容

### 变换矩阵的左手系 & 右手系

原书代码使用的数学库 DirectX Math 提供的生成透视矩阵、观察矩阵等矩阵的函数末尾有一个 `LH` 或 `RH` 后缀，指示左手矩阵还是右手矩阵。我们在学 OpenGL 使用的 glm 以及我自己写的数学库（在 `toy-renderer` 中）中，都是右手矩阵。使用左右手矩阵绘制的结果是左右镜像对称的。一些交互代码，如通过鼠标移动观察方向，也要对水平方向的处理做相反处理才能得到类似的效果。

（还有一个不同是，DirectX Math 的透视矩阵得到的深度范围是 $[0, 1]$，我自己的数学库得到的是 $[-1, 1]$）

### 变换矩阵的列优先 & 行优先

DirectX Math 是行优先（row-major）矩阵，glm、我自己的数学库、GLSL 以及 HLSL 都是列优先（column-major）。

HLSL 的 `mul()` 函数，既支持 `mul(mat, vec)` 也支持 `mul(vec, mat)`，向量是行向量还是列向量会根据在参数中的位置来决定。

DirectX Math 自己是行优先矩阵右乘行向量，glm 和我自己的数学库是列优先矩阵左乘列向量，这二者其实是一模一样的。同一个矩阵的行优先表示与列有限表示在内存上相当于一次转置，右乘向量矩阵与左乘向量矩阵也是转置关系，所以这二者的矩阵在内存上是相同的。

原书代码在 HLSL 还继续使用右乘行向量的方式，但 HLSL 是列优先矩阵，所以原书代码在将矩阵赋值到 Constant Buffer 的结构中是要做转置。但根据我们刚刚的分析，我们如果在 HLSL 使用左乘列向量，就不需要这一次转置。

### CMake + Win32/DirectX 12

众所周知，Win32 程序的入口函数是 `WinMain` 不是 `main`（当然，还有 `wWinMain`，区别是命令行参数是 ASCII 还是 Unicode），这一点通过

```cmake
set(CMAKE_WIN32_EXECUTABLE TRUE)
```

或

```cmake
add_executable(target_name WIN32
    ...
)
```

来实现。

链接 DirectX 库也很简单，必经这些库和头文件都默认在环境变量里的（Windows 相关的头文件好像要手动加一下来着）。

```cmake
target_link_libraries(target_name
    PUBLIC d3dcompiler d3d12 dxgi
)
```

还有一点，编译 Win32 程序并不一定要用 MSVC 编译器，像我用的就是 Clang。

## 一些与原书不同的地方

### `IDXGIFactory` & `IDXGISwapChain`

原书中使用 `IDXGIFactory::CreateSwapChain()` 来创建交换链，但官网文档上不推荐使用这种方式，所以修改了关于 `IDXGIFactory` 和 `IDXGISwapChain` 的创建部分：

```cpp
// D3DApp.h
Microsoft::WRL::ComPtr<IDXGIFactory4> p_dxgi_factory;
Microsoft::WRL::ComPtr<IDXGISwapChain1> p_swap_chain;

// D3DApp.cpp
ThrowIfFailed(CreateDXGIFactory2(DXGI_CREATE_FACTORY_DEBUG, IID_PPV_ARGS(&p_dxgi_factory)));

DXGI_SWAP_CHAIN_DESC1 sc_desc = {};
sc_desc.Width = client_width;
sc_desc.Height = client_height;
sc_desc.Format = back_buffer_fmt;
sc_desc.SampleDesc.Count = 1;
sc_desc.SampleDesc.Quality = 0;
sc_desc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
sc_desc.BufferCount = kSwapChainBufferCnt;
sc_desc.Scaling = DXGI_SCALING_STRETCH;
sc_desc.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;
sc_desc.Flags = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH;
DXGI_SWAP_CHAIN_FULLSCREEN_DESC sc_fs_desc = {};
sc_fs_desc.Windowed = true;

ThrowIfFailed(p_dxgi_factory->CreateSwapChainForHwnd(p_cmd_queue.Get(), h_win,
    &sc_desc, &sc_fs_desc, nullptr, &p_swap_chain));
```

### DirectXTK12 加载 DDS 材质

按原书中的说法，当时还没有 DirectXTK12，所以作者自己魔改了一个。而我在学习的时候，DirectXTK12 是已经有了的，在 GitHub 上就可以找到，所以使用 DirectXTK12。

只是加载 DDS 材质的话，需要包含以下头文件：

```cpp
#include "ResourceUploadBatch.h"
#include "DDSTextureLoader.h"
```

首先需要初始化资源上传器：

```cpp
ResourceUploadBatch resource_upload(p_device.Get());
resource_upload.Begin();
```

然后调用 `CreateDDSTextureFromFile()` 加载材质：

```cpp
auto grass_tex = std::make_unique<Texture>();
grass_tex->name = "grass";
grass_tex->filename = root_path + L"textures/grass.dds";
ThrowIfFailed(CreateDDSTextureFromFile(p_device.Get(), resource_upload,
    grass_tex->filename.c_str(), &grass_tex->resource));
textures[grass_tex->name] = std::move(grass_tex);
```

不过这个函数只是得到需要干什么事情，我们需要给上传器一个命令列表的对象让它添加命令，最后再 flush 队列：

```cpp
auto upload_finish = resource_upload.End(p_cmd_queue.Get());
FlushCommandQueue();
upload_finish.wait();
```

其中 `upload_finish` 的类型是 `std::future<void>`。

## 一些其他的

DirectXTK12 的例程，以及一些其他官方例程，它们的代码结构都差不太多的感觉，而且感觉看起来挺舒服的。说不定之后会考虑试一试那样子的写法。以及，之后的代码中应该会更多地使用 `d3dx12.h` 中的便捷函数来填充 `XXX_DESC`。