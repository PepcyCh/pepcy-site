---
title: 重学一点编译链接与 CMake——从课程项目到库开发者的笔记
date: 2023-04-02 14:50:48
tags: [CMake, 编译与链接]
---

标题可能起得有点大，其实就是这一段时间在导师公司负责一个库的开发，其开发过程中对自己所学感到不够扎实的一点笔记，内容也会有点杂。

此前只是简单地在各种课程项目中用 CMake，程序的使用情形也不是我一个人开发就是两三个同学在 GitHub 上推拉代码，仅了解一点就足以，但实际工程毕竟还要考虑这个库怎么让别人基于其继续开发，以及最终用户会怎么运行，这些几乎都不是写课程项目时会考虑的东西，故记笔记于此。

<!-- more -->

## CMake 的库目标的类别

库目标（library target），就是通过 `add_library()` 创建的目标。类别可以写在库名称的后面，这里主要会提到的类别有 STATIC、SHARED、MODULE、INTERFACE 和 OBJECT，简单来说（严谨具体的描述还请参考文档）：

* STATIC 顾名思义是静态库（static library）
* SHARED 和 MODULE 算是两种使用形式不同的动态库（shared library / dynamic library）
* INTERFACE 一般用于 header-only 的库，或者配置了一些选项的假目标
* OBJECT 就是只编译到 .o（或者 .obj，Windows 上），一般需要进一步变成静态或动态库

若不写名类别，则会根据变量 `BUILD_SHARED_LIBS` 的值，默认为 STATIC 或 SHARED。

在课程项目时代，经常直接写 `add_libaray(foo foo1.cpp foo2.cpp)` 之类的调用，不在意这个库最终会是什么样，反正最后的可执行能跑就行，所以一般我们是得到了静态库。用 INTERFACE 来处理一些 header-only 库也在课程项目时代相对比较常用，但动态库确实不是课程项目中常用的，或者说，我们很少在课程项目中把自己的代码打成动态库。

静态库与动态库的区别就不说了，SHARED 和 MODULE 都是动态库，区别在哪儿呢？我们知道，载入动态库用启动时载入和运行时载入两种方式。前者一般会在链接时也指明动态库（或者 .lib 的动态库的导入库，Windows 上），之后的使用上与静态库类似，include 相应的头文件，使用里面的东西；后者一般不需要在链接中指明，但在程序中通过 OS 提供的函数手动通过名称或路径载入动态库（或是通过一些封装这些功能的库，如 [dylib](https://github.com/martin-olivier/dylib)），并手动根据符号名载入变量或函数，写过 OpenGL 的人大概都会用过 glad，它是一个类似的例子（但 glad 只有根据符号名载入函数的部分，没有载入动态库本身的部分）。SHARED 与 MODULE 的区别就在于，前者期望被通过 `target_link_libraries()` 之类的函数添加到链接库中以启动时载入，而后者期望被运行时载入。

OBJECT 或许用的不多，但 C/C++ 是先编译到 object 文件再变成库的这一点，一些课程都会讲到所以提一句。我知道的一个用途是，先将源代码通过 `add_library()`  添加到一个 OBJECT 目标中，再使用这一 OBJECT 目标分别构建相同源代码的静态库与动态库而只编译一次，而直接对同样的源代码列表添加一个 STATIC 目标和一个 SHARED 目标会编译两次，因为编译选项会有一些区别，如动态库会需要（并自动）启用 PIC（position independent code）选项而静态库不需要。这种方法需要手动为 OBJECT 目标开启 PIC，代价就是静态库可能会因此有效率的影响，因为 PIC 可能会阻止优化，看到的说法是，64 位机上影响不大，其余可能有较大影响。

## CMake 的输出类别

“找不到 xxx.dll”是一个经典的烦人报错，要写一个更好的处理相关问题的 CMake 脚本，肯定不能依赖输出一定是在 build 目录之类的假设。

与库目标的类型相对应，一个目标会有 3 种输出类型

* RUNTIME
  * `add_executable()` 得到的可执行文件
  * DLL 平台下（如 Windows），SHARED 目标的 .dll
* LIBRARY
  * MODULE 目标的 .dll / .so
  * 非 DLL 平台下（如 Linux），SHARED 目标的 .so
* ARCHIVE
  * STATIC 目标的 .lib / .a
  * DLL 平台下，SHARED 目标的导入库 .lib

对于每一个目标，各有一个相应的 `XXX_OUTPUT_DIRECTORAY` 的 property 在目标上，可以通过 `get_target_property()` 和 `set_target_properties()` 获取和设置。Windows 下的一种可行的设置方式是

* RUNTIME、LIBRARY - `${CMAKE_BINARY_DIR}/bin`
* ARCHIVE - `${CMAKE_BINARY_DIR}/lib`

这样 .exe 和 .dll 一定在一个目录下，.dll 就能找到，.lib 也被分离出去。

而对于 Linux 等平台，有 RPATH 这一设置，它是硬编码在库或可执行文件种的运行时搜索路径，可以设置目标的 `BUILD_RPATH ` property 为 LIBRARY 输出目录，未必要把 .so 和可执行文件放在一起（毕竟动态库很多的时候看着有点难受的）。

自己在课程项目时代，几乎不会去管输出目录怎么样，就用的默认的配置，后来这么设置后感觉看着清新了一些。以前自己写拷外部 dll 也只会写直接拷到 build 之类的操作，当时对目标 property 相关操作也没有接触。

当需要通过 `install()` 写安装规则时，安装的也是这 3 种输出，如

```cmake
install(
    TARGETS foo
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION bin
    ARCHIVE DESTINATION lib
)
```

而对于库来说，其头文件需要 `install(DIRECTORY)` 之类的函数手动另写。

## `target_link_libraries()` 在干什么

课程项目时代，`target_link_libraries()` 就已经是一个很常用的函数了，包括用 PUBLIC 和 PRIVATE 进行设置，PRIVATE 表示构建这个目标需要的依赖，PUBLIC 表示不仅构建需要依赖，而且其他使用这个目标的第三者也要依赖，至少我一直是这么认为的。

直到有一天，我为我的库写了安装规则，并在 `add_subdirectory()` 添加第三方库时加了 ` EXCLUDE_FROM_ALL` 来把它们的安装从默认中屏蔽调，因为它们只是 PRIVATE 依赖，但 CMake 却提示了需要安装 xxx 但不存在安装规则之类的报错。

那么问题是什么？我的库那时是一个静态库，对于静态库目标，CMake 会把 PRIVATE 提升为 PUBLIC，因为 `target_link_libraries()` 不会对静态库真的做链接操作，或者说，不存在把库链接到一个静态库中的操作，它仅仅只是建立了依赖关系，直到一个可执行目标或动态库目标，才会一口气把这一路上的静态库链接到可执行文件或动态库中，所以它实际上只能是 PUBLIC 的，而动态库的 PRIVATE 才是真 PRIVATE。

从以上解释中还能注意到另一点：从使用的角度讲，静态库和动态库都是库，算是同一类，可执行文件使用库，用于执行，算另一类；但从编译链接的角度讲，可执行文件和动态库才是同一类，它们都算是链接的终点，而静态库只是一组 object 文件的打包（所以是 archive）。

这么看来，Windows 之类的 DLL 平台下，一个 SHARED 动态库同时有一个静态的 .lib 导入库用于静态链接，把动态库和静态库在上述分类中分得更清了，倒是 Linux 等非 DLL 平台中 .so 可以出现在链接的参数里，虽然运行时还是要去载动态库。

## 符号与动态库

“无法解析的符号 xxx”和“重复定义的符号 xxx”又是两个经典报错，而事实上，符号（symbol）这个东西，汇编、计组、OS 等课上写的汇编代码中已经解除了一些类，更不要说 oslab 中既要写 RISC-V 汇编又要写 C 的，理论上应该对符号有一定程度的认知了。

一个全局变量或函数，其相应的符号指明了它所在的地址，这样才能找得到它，所以要求使用到的符号存在且唯一。在链接 object 文件为静态库时，各个 object 文件会被打包在一起，所以不能有相同的符号名在不同的 object 中，所以不建议在头文件中直接定义变量或函数，因为可能会导致这个问题；而目前的 `inline` 的语义则是让这个符号只在同一翻译单元（一般是一个源代码文件，对应一个 object 文件）中被引用，所以在头文件中可以加 `inline` 来定义变量或函数。

在 C 中，基本上一个全局变量或函数的符号的名字就是它在代码中的名字。C++ 会对变量名做 mangle 来支持命名空间、成员函数、函数重载等语言特性，如 `void func(int)` 可能会被 mangle 为 `_Z4funci` 或 `?func@@YAXH@Z` 等而 `void func(int, char)` 会变为 `_Z4funcic` 或 `?func@@YAXHD@Z` 等。如何 mangle 符号名是 ABI（abstract binary interface）的一部分，但 C++ 没有规定 ABI，也就是说各编译器可以各自有一套 mangle 的规则，这使得 A 编译器编译出来的库被 B 编译器（甚至不同版本的 A 编译器）编译的代码使用时，有可能会找不到符号。而 `extern "C"` 可以认为是要求编译器不做 mangle，MODULE 动态库可能会更常用它来移除 mangle，从而能方便的通过符号名手动来找函数或变量。只考虑主流平台和主流编译器，或许可以假设在同一个平台下 mangle 结果是一致的，一般编译器开发也会注意尽量不要破坏 ABI 的兼容性，如之前的例子中，`_Z4funci` 是 Linux 下常见的 mangle 结果，`?func@@YAXH@Z` 是 Windows 下常见的 mangle 结果。

此外，符号还有可见性，控制符号是否对外部可见（指动态库外部），使用不可见的符号同样会找不到符号。Windows/MSVC 下符号默认是不可见的，需要 `__declspec(dllexport)` 来使可见，同时有相配套的 `__declspec(dllimport)` 表示这个符号会从外部来。Linux 下符号默认是可见的，但可以通过编译选项设为默认不可见，相应的 CMake 配置是 `CXX_VISIBILITY_PRESET` property 为 `hidden`，一般可能还会同时设置 `VISIBILITY_INLINES_HIDDEN ` 让 inline 的符号不可见，此时就需要 `__attribute__((visibility("default")))` 来让符号可见。因此，在动态库项目中有一组常见的宏定义，来根据 OS 暴露符号：

```c++
#ifdef FOO_STATIC_LIBS
#  define FOO_API
#else
#  ifdef _MSC_VER
#    ifdef FOO_API_EXPORT
#      define FOO_API __declspec(dllexport)
#    else
#      define FOO_API __declspec(dllimport)
#    endif
#  else
#    ifdef FOO_API_EXPORT
#      define FOO_API __attribute__((visibility("default")))
#    else
#      define FOO_API
#    endif
#  endif
#endif
```

关于可见性，本人的一个经历是，我的库使用到了 glad，而使用这个库的项目自己使用了 glad2，二者产生了一大坨符号冲突，后来我的库做成了动态库、关闭默认可见，一切正常了。

理论上，动态库要关注的 ABI 问题不只有 mangle，函数调用的传参方式、虚表布局等也是，但我还没遇到过问题。另外，动态库跨编译器使用也要考虑标准库，尤其是 STL 那些东西的实现，毕竟 C++ 标准也没规定要怎么实现，有可能会两侧实现不一致而挂掉。

## 依赖打包

当我们的开发需要一些安装的第三方 SDK 时，打包不能假设用户也要安装这些 SDK。

还是我的库的例子，它使用了 CUDA 和 OptiX，我本人开发可以从 NVIDIA 网站上下载和安装一下，但打包给使用者的话，显然不能让每一个用户都去下载 CUDA SDK。事实上，CUDA 的库大都同时提供了静态库与动态库版本，CMake 自己提供的 FindCUDAToolkit 也有相应静态与动态两个不同的目标，链接静态库到自己的动态库之中，便不用使用者也下载 CUDA SDK，而要静态使用自己的库的开发者则可以自己下载 CUDA SDK 打包进它的可执行文件或动态库之中。而 OptiX 事实上是 header-only 的库（指 OptiX 7.x），其中运行时载入了 nvoptix 这一由驱动提供的动态库，所以其实本来就不需要使用者去下载，OptiX 的证书好像也是允许把它的头文件拷下来转发的。

从上述内容来看，或许同时提供一个把依赖尽量打包好的动态库和一个供他人打包的静态库会是一个比较好的选择，但静态库的依赖必须沿依赖一路都提供出去，本人目前也不知道是否有什么更好的做法，静态库符号的问题也是同样。
