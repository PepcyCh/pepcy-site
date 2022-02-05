---
title: simple-JPEG-IO 实现笔记
date: 2019-10-31 12:21:08
tags: [随笔, 实现笔记]
categories: 学习笔记（大学课外）
---

文档：https://www.w3.org/Graphics/JPEG/itu-t81.pdf

实现参考：

* stb_image：https://github.com/nothings/stb/blob/master/stb_image.h
* Game Engine From Scratch：https://github.com/netwarm007/GameEngineFromScratch/blob/master/Framework/Parser/JPEG.hpp

---

前些日子写学校里图像信息处理的作业时想着不用 BMP 而直接用 JPEG 会方便很多，本来也想写这么一个轮子，所以就花了些时间实现出了这么一个玩意儿。本文就算是我写这个东西时的学习笔记了，因为重心放在了解析上，输出只是随便糊了一个最基本的，所以大部分内容都是站在解析的角度上记录的。

simple-JPEG-IO：https://github.com/PepcyCh/simple-JPEG-IO

---

笔记大概分为四个部分：

* 大致压缩流程
* 基本文件结构
* 解析流程
* 输出流程

<!-- more -->

## 大致压缩流程

把图像分成若干 $8 \times 8$ 的块，右侧和下侧超出的部分用边界值填充。

对于每一个块，先转化至 YCbCr 空间，之后分通道进行压缩。

首先对该块进行 DCT。

之后对 DCT 后得到的 $8 \times 8$ 的矩阵中的每一个元素分别除以一个 $8 \times 8$ 的量化矩阵中的相应元素并向下取整。

在每一个 Restart Interval 内（见「基本文件结构-DRI」），除第一个 DC 系数（$8 \times 8$ 的块中左上角的值）外，每一个 DC 系数对前一个块的 DC 系数做差；AC 系数（$8 \times 8$ 的块中除左上角以外的值）使用 Run-length Code 压缩（具体见「基本文件结构-熵编码部分」）。

之后使用 Huffman 编码进行压缩（Huffman 编码的不是 DC 或 AC 系数，具体见「基本文件结构-熵编码部分」）。

---

JPEG 使用的 DCT 公式：
$$
S_{vu} = \frac{1}{4} c_u c_v \sum_{x = 0}^{7} \sum_{y = 0}^{7} s_{yx} \cos \frac{(2x + 1) u \pi}{16} \cos \frac{(2y + 1) v \pi}{16}
$$
IDCT 公式：
$$
s_{yx} = \frac{1}{4} \sum_{u = 0}^{7} \sum_{v = 0}^{7} c_u c_v s_{vu} \cos \frac{(2x + 1) u \pi}{16} \cos \frac{(2y + 1) v \pi}{16}
$$
其中：
$$
c_u, c_v = \begin{cases}
\frac{1}{\sqrt{2}} &, u, v = 0 \\
1 &, \text{otherwise}
\end{cases}
$$

## 基本文件结构

JPEG 文件中使用大端法。

JPEG 文件由若干块组成，每一块的开头是一个 2 字节长的块标识符，第一字节均为 `0xFF`。除图像首尾的块 SOI（Start Of Image）、EOI（End Of Image）和一些块外，大部分块在 2 字节的块标识符后还有一个 2 字节的部分表示块的长度，该长度包括自己的 2 字节，但不包括块标识符。

根据压缩流程中所描述的，要正确解析一个 JPEG 图像，只需要考虑以下块：

* SOI（Start Of Image）
* EOI（End Of Image）
* SOFn（Start Of  Frame）
* DQT（Define Quantization Table）
* DHT（Define Huffman Table）
* SOS（Start Of Scan）
* DRI（Define Restart Interval）
* RST（Restart）

除此之外，还要了解真正存储图像的熵编码部分。

之后将依次介绍以上每一个块。

### SOI（Start Of Image）

块标识符：`0xFFD8`

除标识符外无其他内容，出现在文件最开始。

### EOI（End Of Image）

块标识符：`0xFFD9`

除标识符外无其他内容，出现在文件最末尾。

### SOFn（Start Of Frame）

块标识符：`0xFFCn`（$n = 0, 1, \dots, \text{F}$）

常用的有 SOF0 和 SOF2，分别表示基线模式（Baseline DCT）和连续模式（Progressive DCT）。

该块描述了一些图像的基本信息。

其块内容可表示为：
$$
\text{Lf} \;\; \text{P} \;\; \text{Y} \;\; \text{X} \;\; \text{Nf} \;\; (\text{Ck} \;\; \text{Hk} \;\; \text{Vk} \;\; \text{Tqk})_{1 \sim \text{Nf}}
$$

#### $\text{Lf}$：Frame Header Length - 16bits

块长度，等于 $8 + 3\text{Nf}$

#### $\text{P}$：Sample Precision - 8bits

可取的值为 $8$ 和 $12$，基线模式下只能为 $8$。simple-JPEG-IO 只考虑了为 $8$ 的情况，实践中还未因此受到影响。

#### $\text{Y}$：Number of Lines - 16bits

图像的高度。若为 $0$ 则表示由 DNL（Define Number of Lines）块定义，本文不考虑这种情况。

#### $\text{X}$：Number of Samples per Line - 16bits

图像的宽度。

#### $\text{Nf}$：Number of Image Components in Frame - 8bits

图像的通道数。一般为 $3$，按顺序分别为 Y、Cb、Cr。

#### $\text{Ck}$：Component Identifier - 8bits

该通道的编号。

#### $\text{Hk}$：Horizontal Sampling Factor - 4bits

水平采样因子，具体解释见「解析流程」。

#### $\text{Vk}$：Vertical Sampling Factor - 4bits

竖直采样因子，具体解释见「解析流程」。

#### $\text{Tqk}$：Quantization Table Destination Selector - 8bits

该通道使用的量化表的编号（$0 \sim 3$）。

### DQT（Define Quantization Table）

块标识符：`0xFFDB`

该块定义了一个或多个量化表。

一个定义了 $n$ 个量化表的块内容可表示为：
$$
\text{Lq} \;\; (\text{Pq} \;\; \text{Tq} \;\; (\text{Qk})_{0 \sim 63})_{1 \sim n}
$$

#### $\text{Lq}$：Quantization Table Defination Length - 16bits

块长度。

#### $\text{Pq}$：Quantization Table Element Precision - 4bits

可取的值为 $0$ 或 $1$，若为 $0$，表示该表的每一个 $\text{Qk}$ 占 8bits，否则表示占 16bits。基线模式下只能为 $0$。

#### $\text{Tq}$：Quantization Table Destination Identifier - 4bits

该量化表的编号，取值为 $0 \sim 3$，用于 SOFn 的 $\text{Tqk}$。

#### $\text{Qk}$：Quantization Table Elements - 8bits/16bits

量化表的每一个元素，按 zig-zag 序给出。

### DHT（Define Huffman Table）

块标识符：`0xFFC4`

该块定义了一个或多个 Huffman 表。

一个定义了 $n$ 个 Huffman 表的块内容可表示为：
$$
\text{Lh} \;\; (\text{Tc} \;\; \text{Th} \;\; (\text{Lk})_{1 \sim 16} \;\; (\text{Vi,j})_{i=1 \sim 16, j = 1 \sim \text{Li}})_{1 \sim n}
$$

#### $\text{Lh}$：Huffman Table Defination Length - 16bits

块长度。

#### $\text{Tc}$：Table Class - 4bits

若为 $0$，表示是一个 DC 表；若为 $1$，表示是一个 AC 表。

#### $\text{Th}$：Huffman Table Destination Identifier - 4bits

Huffman 表的编号，取值为 $0 \sim 3$（基线模式下取值为 $0 \sim 1$），编号相同但类别不同的 Huffman 表不是同一个表（也就是一共最多会有 $8$ 个表）。

#### $\text{Lk}$：Number of Huffman Codes of Length $k$ - 8bits

长度为 $k$ 的 Huffman 码的个数。

#### $\text{Vi,j}$：Value Associated with Each Huffman Code - 8bits

长度为 $i$ 的第 $j$ 个 Huffman 码表示的值。

具体的编码方式将在「解析流程」中给出。

### SOS（Start Of Scan）

块标识符：`0xFFDA`

该块标识着一次扫描的开始，若为基线模式，则只会有一个 SOS 块。紧接其后的就是熵编码的扫描内容。

其块内容可表示为：
$$
\text{Ls} \;\; \text{Ns} \;\; (\text{Csk} \;\; \text{Tdk} \;\; \text{Tak})_{1 \sim \text{Ns}} \;\; \text{Ss} \;\; \text{Se} \;\; \text{Ah} \;\; \text{Al}
$$

#### $\text{Ls}$：Scan Header Length - 16bits

块长度，不含熵编码部分。

#### $\text{Ns}$：Number of Image Components in Scan - 8bits

该次扫描内涉及的通道数。

#### $\text{Csk}$：Scan Component Selector - 8bits

扫描内第 $k$ 个通道的编号，该编号就是在 SOFn 中定义的编号。

#### $\text{Tdk}$：DC Entropy Coding Table Destination Selector - 4bits

该通道使用的 DC 表的编号，该编号就是 DHT 中定义的编号。

#### $\text{Tak}$：AC Entropy Coding Table Destination Selector - 4bits

该通道使用的 AC 表的编号，该编号就是 DHT 中定义的编号。

#### $\text{Ss}$：Start of Spectral of Predictor Selection - 8bits

连续模式时，该次扫描涉及的 $8 \times 8$ 块内第一个元素的 zig-zag 序下标，其及剩下的参数将在「解析流程-连续模式」中做进一步解释。

基线模式下一定为 $0$。

#### $\text{Se}$：End of Spectral of Predictor Selection - 8bits

连续模式时，该次扫描涉及的 $8 \times 8$ 块内最后一个元素的 zig-zag 序下标。

基线模式下一定为 $63$。

#### $\text{Ah}$：Successive Approximation Bit Position High - 4bits

连续模式时，为 $0$ 表示是 $\text{Ss} \sim \text{Se}$ 的 DCT 系数的第一次扫描，否则是之后的扫描。

基线模式下一定为 $0$。

#### $\text{Al}$：Successive Approximation Bit Position Low - 4bits

连续模式时，该次扫描涉及的每一个 DCT 系数的位移系数。

基线模式下一定为 $0$。

### DRI（Define Restart Interval）

块标识符：`0xFFDD`

该块定义了 restart interval 的长度，每当扫描后的熵编码部分已经解析或编码了长度的整数倍的 MCU（Minimum Coded Unit，最小编码单元，将在「解析流程」中做进一步解释） 时暂时结束熵编码部分，同时清零已保存的当前的 DC 值，之后遇到 RST（Restart）块时将继续进行接下来的扫描。如果无 DRI 块，则可以认为 restart interval 的长度为 0，之后也不会有 RST 块。

比如，Photoshop 的基线模式会把图像的每一行设成一个 restart interval，那么文件中会是「SOS，第一行数据，RST，第二行数据，……，RST，最后一行数据」。

当熵编码部分结束时不是一个满的字节时，需要填充满。

其块内容可以表示为：
$$
\text{Lr} \;\; \text{Ri}
$$

#### $\text{Lr}$：Define Restart Interval Segment Length - 16bits

块长度，等于 $4$。

#### $\text{Ri}$：Restart Interval - 16bits

定义的长度。

### RST（Restart）

块标识符：`0xFFD7`

无内容，紧接其后的是熵编码的扫描内容。

### 熵编码部分

此外，需要了解熵编码部分的结构，每一个数值的存储如下：

首先是 $\text{RRRRSSSS}$ ，一个 8 位的数，$\text{RRRR}$ 表示在它之前为 $0$ 的元素个数（对于 DC 系数，应当是 $0$），$\text{SSSS}$ 表示存储该数值需要的位数。这个 $\text{RRRRSSSS}$ 便是用 Huffman 表编码的东西，也就是说，存储的是编码后的 Huffman 码。在基线模式下，DC 系数的 $\text{SSSS}$ 的取值范围是 $0 \sim 11$，AC 系数的取值范围是 $1 \sim 10$，所以基线模式下的一个 DC 表会有 $12$ 个元素，一个 AC 表会有 $162$ 个元素（包括了之后提到的 ZRL 和 EOB），一些解析器（如 Photoshop 的）需要 Huffman 表正好有这么多的元素才视作可解析的 JPEG 文件。

之后才是真正的数值，读取 $\text{SSSS}$ 长的二进制数，根据下表可以得到真正的数值：

| 长度 |         所编码的数         |
| :--: | :------------------------: |
|  0   |             0              |
|  1   |           -1, 1            |
|  2   |        -3, -2, 2, 3        |
|  3   | -7, -6, -5, -4, 4, 5, 6, 7 |
|  4   |      -15 ~ -8, 8 ~ 15      |
|  5   |     -31 ~ -16, 16 ~ 31     |
|  6   |     -63 ~ -32, 32 ~ 63     |
|  7   |    -127 ~ -64, 64 ~ 127    |
|  8   |   -255 ~ -128, 128 ~ 255   |
|  9   |   -511 ~ -256, 256 ~ 511   |
|  10  |  -1023 ~ -512, 512 ~ 1023  |
|  11  | -2047 ~ -1024, 1024 ~ 2047 |

$\text{RRRR}$ 指示之前为 $0$ 的数据数，但显然它最多只能表示 $15$ 个 $0$，于是定义 $\text{RRRRSSSS} = \text{0xF0}$ 为 ZRL（Zero Run Length），表示连续的 $16$ 个 $0$（如果不做特殊处理，理解成一个 $0$ 的前面有 $15$ 个 $0$ 也是可以得到正确结果的）；同时，可能会出现从当前到 $8 \times 8$ 块的最后一个数都是 $0$ 的情况，用 $\text{RRRRSSSS} = 0x00$ 表示，该值被称作 EOB（End Of Block）。在连续模式中会有一些变化，将在「解析流程-连续模式」中进一步说明。

很明显，在熵编码部分中可能出现 `0xFF` 这半个块标识符，而熵编码部分又不像 SOS、SOFn 这样的块用 2 字节指示了长度，所以规定，如果出现了 `0xFF`，在其后需要追加一个无意义的 `0x00`，称为「bitstuff」，解析时应先去除这样的 `0x00`。

## 解析流程

### 基线模式

很多东西在基线模式和连续模式下都是相同或相似的，就都在「基线模式」中描述了。

根据 JPEG 文件结构的特性，很容易想到用一个 `while` 循环控制，依次解析各个块，直至遇到 EOI 块结束。

#### 根据 DHT 的信息建立 Huffman 树

按顺序考虑每一个 $\text{Vi,j}$，记录一个当前的编码 $curr$，初始为 $0$，当没有长度为 $1$ 的编码时，在后面补 $0$，直至找到一个最小的长度 $l$ 使 $\text{Ll} \neq 0$，为 $V_{l,1}$ 分配编码 $curr$，之后 $curr$ 自增 $1$，之后如果仍存在相同长度的编码，重复分配和自增的操作；否则重复补零的操作。

#### 采样因子、扫描与 MCU

MCU（Minimum Coded Unit），最小编码单元，即在一次扫描过程中最小的处理单元，在扫描过程中，按先从左到右、后从上到下的顺序依次处理每一个 MCU。

如果一次扫描内只有一个通道，即 $\text{Ns} = 1$，则无论该通道的采样因子取何值，MCU 都被定义成一个 $8 \times 8$ 的块。当图像的宽或高不为 $8$ 的整数倍时，在下层和右层按边界值填充。

否则，需要考虑不同通道的采样因子。

假设一副 JPEG 图像 Y、Cb、Cr 通道的水平和竖直采样因子均为 $1$，那就是最简单的情况，此时 MCU 被定义成一个 $8 \times 8$ 的 Y 通道、一个 $8 \times 8$ 的 Cb 通道和一个 $8 \times 8$ 的 Cr 通道。

而除此之外，还有一种常见的情况是 Y 通道的两个采样因子均为 $2$，Cb 和 Cr 的采样因子为 $1$，这意味着 Cb 和 Cr 被进行了缩小。此时，图像最左上角的 $2 \times 2$ 个像素有各自的 Y 值，Cb 和 Cr 的值却公用同一个。MCU 此时的定义是一个 $2 \times 2$ 的块的 Y 通道（如果称一个 $8 \times 8$ 为一个块）、一个 $1 \times 1$ 的块的 Cb 通道和一个 $1 \times 1$ 的块的 Cr 通道。这同样暗示着 Y 通道的宽高不只要填充至 $8$ 的整数倍，还要填充到 $16 \times 16$ 的整数倍。待全部解析完毕后，需对 Cb 和 Cr 通道进行上采样以得到原始图像每个像素的值。

从而我们可以得到是，一个 MCU 由不同通道的 $\text{Vk} \times \text{Hk}$ 的块组成，采样因子最大的那一通道的相应维是没有经过下采样的。

---

在各个块和熵编码部分都解析完毕后，对每一个 $8 \times 8$ 的块进行 IDCT，对需要上采样的通道上采样，最后转为 RGB 空间。

### 连续模式

连续模式与基线模式最大的区别就是 SOS 块中的 $\text{Ss}, \text{Se}, \text{Ah}, \text{Al}$ 了。

连续模式的目的是，当只解析了部分熵编码后，能得到一个完整但不精确的图像，具体的实现方法就是把基线模式中对一个 MCU 的处理分成几个部分，放在不同的扫描中。

对一个 MCU 内的信息的拆分有两种，一是把一个 $8 \times 8$ 块的 $64$ 个数据分段，二是把每一个数按二进制位分段。

$\text{Ss}$ 和 $\text{Se}$ 对应的是第一种，表示本次扫描只有 $64$ 个数中 zig-zag 序下标 $\text{Ss} \sim \text{Se}$ 的数据。特别的，规定 AC 和 DC 不应混在一起，也就是 $\text{Ss} = 0$ 时必有 $\text{Se} = 0$，$\text{Se} \neq 0$ 则 $\text{Ss} \neq 0$。

$\text{Ah}$ 和 $\text{Al}$ 对应的是第二种。在第一次扫描时，$\text{Al}$ 表示解析出的值最后均作一次左移 $\text{Al}$ 位的操作，之后的扫描将一位一位地细化每一个 DCT 系数，存储的就是每个系数的第 $\text{Al}$ 位是否需要调整。

还有一些其他的规定：仅包含 DC 系数的第一次扫描必须是所有扫描中的第一个扫描，仅包含 DC 系数的扫描涉及所有的通道，包含 AC 系数的扫描仅包含一个通道。

在 DC 系数的第一次扫描时，除了要进行左移操作外与基线模式基本相同。

在 AC 系数的第一次扫描时，除了要进行左移操作外，EOB 的定义也发生了变化。EOB 现在能表示的不仅是自己这一块，还表示后续的若干块均为 $0$，此时，凡是 $\text{SSSS} = 0$ 但 $\text{RRRR} \neq 15$ 的都是一个 EOB 标记，在其后还有长为 $\text{RRRR}$ 的数表示为 $0$ 的块数（含当前所在块），该数按下表的规律解析：

| 长度 | 所编码的数 |
| :--: | :--------: |
|  0   |     1      |
|  1   |    2, 3    |
|  2   | 4, 5, 6, 7 |
|  ……  |     ……     |

DC 系数的后续扫描中，按序每次读取一个二进制位，为 $1$ 则表示相应系数的第 $\text{Al}$ 位需要调整。

AC 系数的后续扫描中，与之前的扫描类似，但此时一定有 $\text{SSSS} = 1$（除非是 ZRL 和 EOB），后一位表示该数是否需要调整，同时， $\text{RRRR}$ 现在表示的是在该数之前有多少个在之前的扫描中为 $0$ 的数这次仍然为 $0$，对于之前不为 $0$ 但这时被跳过去的数，是否需要调整则以类似 DC 系数后续扫描的方式缀在表示该数的那一位的后面。因 EOB 和 ZRL 而被跳过去的块也是类似的道理。

## 输出流程

只要依次输出每一个必要的块的内容即可。

建立 Huffman 表时注意最长的编码长度不能超过 $16$，以及所有可能出现的编码都应出现在表中，对于基线模式就是 $12$ 个 DC 的 $\text{RRRRSSSS}$ 和 $162$ 个 AC 的 $\text{RRRRSSSS}$。可以偷懒用等长编码。

注意在熵编码部分添加 bitstuff 和填充最后一个未满的字节。

量化表随便搞一个就好了，我用的是 Photoshop 输出基线模式时用的量化表。