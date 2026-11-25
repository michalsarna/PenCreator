🌐 [English](Layers-and-Pens) · [Polski](Warstwy-i-pisaki) · **简体中文** · [한국어](Layers-and-Pens-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Layers-and-Pens)为准。欢迎提出修改建议。

# 图层与笔

一个绘图有 **64 个图层**。每个形状都位于一个图层上，而 **一个图层就是一支笔**：它的颜色、线条宽度、速度以及其上形状的绘制方式（轮廓、填充线或图片）。各图层依次绘制，当下一个图层需要另一支笔时，作业会停下来让你更换。

![图层标签页：一个图层就是一支笔](https://raw.githubusercontent.com/michalsarna/PenCreator/main/docs/screenshots/05-layers.png)

## 新绘图的图层

| 图层 | 名称 | 绘制方式 |
|---|---|---|
| 1 | 黑笔（Black pen） | **轮廓**（Outline） |
| 2 | 红笔（Red pen） | **轮廓** |
| 3 | 蓝笔（Blue pen） | **轮廓** |
| 4 | 绿笔（Green pen） | **轮廓** |
| 5 | 黑色填充（Black fill） | **用线条填充**（Fill with lines）（黑色，与图层 1 相同） |
| 6 … 64 | 图层 6 … 图层 64（Layer 6 … Layer 64） | **轮廓** |

它们一开始的 **笔宽**（Pen width）都是 0.4 mm，**速度**（Speed）为 3000 mm/min，**填充线间距**（Fill line gap）为 0.4 mm、角度 45°，绘制一遍，且未选用笔库中的笔。从 SVG 或 PDF 打开的绘图，以及 LightCreator 或 MillCreator 的项目，也会得到这些图层。

## 图层标签页

标签页顶部是 **图层列表**。它显示含有形状的图层以及你正在使用的图层；勾选 **显示全部 64 个图层**（Show all 64 layers）可看到所有图层。每一行包括：

* **▲ / ▼**（**在作业中提前**（Earlier in the job）/ **在作业中延后**（Later in the job））：列表按作业顺序从上到下排列；箭头将图层移动一个位置。顺序随项目一起保存。
* 图层的颜色；
* 复选框 **绘制：纳入绘图和 G-code**（Drawn: taken into the plot and the G-code）。未勾选的图层会被排除在预览、刀具路径、G-code 和作业之外，但其形状仍留在工作区上；
* 眼睛图标 **在工作区中显示**（Shown in the work area）。隐藏的图层只是在工作区上隐藏（**全选**（Select all）也会跳过其形状）；只要它被勾选，作业仍会绘制它；
* 图层的名称和绘制方式：单击会使其成为 **当前图层**（active layer），其设置显示在下方，新形状也会放到这里；
* 图层上的形状数量。

列表下方是 **保存预设…**（Save preset…）和 **加载预设**（Load preset）（参见[图层预设](#图层预设)），以及当前图层的设置：

| 设置 | 作用 |
|---|---|
| **名称**（Name） | 图层的名称。它会作为注释 `(Layer: name)` 写入 G-code，也是换笔 G-code 中的变量 `{layer_name}`。 |
| **颜色**（Colour） | 形状在工作区上的颜色。它是笔的一部分：两个颜色不同的图层就是两支笔（参见[换笔](#换笔)）。 |
| **绘制方式**（Drawing） | **轮廓**（Outline）、**用线条填充**（Fill with lines）、**填充并描边**（Fill and outline）或 **图片**（Picture）（见下文）。 |
| **笔**（Pen） | [笔库](#笔库)中的一支笔，或 **未选择笔**（No pen chosen）。**…** 按钮会打开笔库。选择一支笔会把 **笔宽** 设为其笔迹的较大尺寸。 |
| **笔宽**（Pen width） | 笔画出的线条宽度，0.01 … 20 mm。没有选用笔库中的笔时，它就是预览中线条的粗细。 |
| **速度**（Speed） | 笔在纸上的速度，1 … 100000 mm/min（`G1` 移动的 `F`）。 |
| **图片方法**（Picture method） | 仅用于 **图片**：**抖动行**（Dithered rows）、**点**（Dots）或 **按色调排线**（Hatching by tone）。 |
| **抖动**（Dithering） | 仅用于 **抖动行** 和 **点**：图片如何转换成点：**阈值**（Threshold）、**Floyd-Steinberg**、**Jarvis**、**Stucki**、**Atkinson**、**有序**（Ordered）或 **灰度**（Grayscale）（每项都有提示）。 |
| **填充线间距**（Fill line gap） | 用于填充和图片：线条之间的距离，0.02 … 100 mm。 |
| **填充线角度**（Fill line angle） | 用于填充和图片：线条的方向，−360 … 360°。 |
| **交叉线**（Crossed lines） | 用于填充：在第一组填充线上再交叉画第二组。 |
| **绘制次数**（Drawn times） | 每条线绘制的次数，1 … 5：用于颜色太淡的笔。开放的线条会来回绘制。 |

设置下方有一行说明所选的绘制方式，**将选中的形状放到此图层**（Put the selected shapes on this layer）会把选中的形状移到这里（锁定的形状保持不动）。

## 绘制方式

* **轮廓**（Outline）：笔沿着形状的线条移动。开放的线条也可以。
* **用线条填充**（Fill with lines）：封闭形状用平行线填充；孔洞保持空白。
* **填充并描边**（Fill and outline）：先填充，再在其上绘制轮廓，使边缘干净。
* **图片**（Picture）：图层上的图片（位图）用线条绘制：**抖动行**（Dithered rows）（抖动后图片的每一行在暗处绘制）、**点**（Dots）（每个暗单元画一个点）或 **按色调排线**（Hatching by tone）（交叉排线：图片越暗，线条方向越多）。这种图层上的其他形状按轮廓绘制。图片放到某个图层上时，会将该图层切换为 **图片** 一次；你可以再改回来。

在其他绘制方式的图层上，图片不会被绘制：预览和状态行会提示 *图层上的图片无法绘制：请先将其描摹为线条*（picture(s) on the layers cannot be drawn: trace them to lines first）（**排列 → 描摹图像…**（Arrange → Trace image…））。

填充先于轮廓绘制，并且一个图层的线条会按缩短抬笔移动的顺序排列。很小的间距会产生大量线条：一个作业最多生成 200 000 条填充线，其余的填充会被省略，并有警告提示。

## 工作区下方的图层行

64 个图层每个都有一个色块，排成一行，可以横向滚动（两端的箭头或滚轮）。圆点表示该图层有形状，褪色的色块表示隐藏的图层，圆环表示当前图层。色块的提示会显示其编号和名称。

* **单击** 会把选中的形状放到该图层（锁定的形状不动），并使其成为当前图层；没有选中任何内容时，只会使其成为当前图层。
* **双击** 会在该图层上打开 **图层**（Layers）标签页。
* 启用 **设置 → 视图选项… → 显示图层名称**（Settings → View options… → Show layer names）（默认关闭）后，含有形状的图层会在色块旁显示其名称（长名称会被截断；完整名称在提示中）。

## 图层预设

图层设置上方的 **保存预设…**（Save preset…）和 **加载预设**（Load preset）都会打开 **图层预设**（Layer presets）窗口：

* 输入名称并按 **保存预设…**，即可将当前图层的设置以该名称保存（最多 60 个预设）；
* **搜索**（Search）字段可缩小列表范围；单击某个预设会把它的设置应用到当前图层；铅笔图标可重命名，**×** 可删除；
* **导出…**（Export…）将所有预设写入文件，**导入…**（Import…）添加某个文件中的预设。

预设保存除名称、颜色、可见性、是否绘制以及在顺序中的位置之外的所有内容：绘制方式、笔、宽度、速度、图片方法和抖动、填充以及遍数。如果预设中的笔已不在笔库中，图层将没有笔（宽度保持不变），状态行会提示 *预设中的笔不在笔库中：该图层没有笔*（The pen of the preset is not in the library: the layer has no pen）。预设保存在共享数据文件夹中的 `pen_layer_presets.json`（参见[设置与文件](Settings-and-Files-ZH)）。

## 笔库

**绘图仪 → 笔库…**（Plotter → Pen library…）（或图层标签页中 **笔** 旁边的 **…** 按钮）会打开 **笔库**（Pen library）窗口。左侧是 **现成的笔**（Ready-made pens）和 **我的笔**（My pens）；右侧是所选笔的样本（四个方向的线条和一个点，按笔实际画出的样子绘制）及其设置。

### 什么是笔

| 设置 | 作用 |
|---|---|
| **名称**（Name） | 在列表、图层的 **笔** 以及 G-code 注释中显示的名称。 |
| **类型**（Type） | **铅笔**（Pencil）、**钢笔（笔尖）**（Fountain pen (nib)）、**针管笔**（Fineliner）、**毡头记号笔**（Felt-tip marker）、**荧光笔**（Highlighter）或 **软头笔**（Brush pen）。 |
| **笔迹宽度**（Width of the mark） | 笔尖横向的尺寸，0.01 … 50 mm。 |
| **笔迹高度**（Height of the mark） | 笔尖纵向的尺寸，0.01 … 50 mm；圆形笔尖与宽度相同。 |
| **笔尖转角**（Turn of the tip） | 斜头或笔尖宽度的方向，以度为单位。沿宽度方向的线条与宽度一样粗；与之垂直的线条与高度一样细。 |
| **线端**（Ends of a line） | **圆头**（Round ends）或 **平头**（Flat ends）。 |
| **墨水颜色**（Ink colour） | 预览中线条的颜色。 |
| **不透明度**（Opacity） | 5 … 100 %：值越小越透明（如荧光笔），线条交叉处会变深。 |

图层的笔会改变 **预览**（Preview）：每条线都按笔在该方向上画出的粗细、线端、墨水颜色和不透明度来绘制。没有选用笔库中的笔的图层按其自身颜色、圆头、以其 **笔宽** 的粗细绘制。

### 现成的笔

| 笔 | 类型 | 笔迹 | 线端 | 墨水 | 不透明度 |
|---|---|---|---|---|---|
| Fineliner 0.2 mm | 针管笔 | 0.2 mm | 圆头 | 黑色 | 100 % |
| Fineliner 0.4 mm | 针管笔 | 0.4 mm | 圆头 | 黑色 | 100 % |
| Fineliner 0.8 mm | 针管笔 | 0.8 mm | 圆头 | 深蓝色 | 100 % |
| Pencil 0.5 mm | 铅笔 | 0.5 mm | 圆头 | 灰色 | 75 % |
| Fountain pen nib 1.8 x 0.3 mm | 钢笔（笔尖） | 1.8 × 0.3 mm，旋转 45° | 平头 | 深蓝色 | 95 % |
| Felt-tip marker 1 mm | 毡头记号笔 | 1 mm | 圆头 | 红色 | 90 % |
| Chisel marker 3 x 1 mm | 毡头记号笔 | 3 × 1 mm，旋转 45° | 平头 | 黑色 | 95 % |
| Highlighter 4 x 1 mm | 荧光笔 | 4 × 1 mm，旋转 45° | 平头 | 黄色 | 40 % |
| Brush pen 2 mm | 软头笔 | 2 mm | 圆头 | 黑色 | 85 % |

现成的笔不能修改（*现成的笔不能修改。复制一份后再修改。*（A ready-made pen cannot be changed. Make a copy to change it.））。

### 你自己的笔

* **新建笔**（New pen）会在 **我的笔** 中添加一支笔（以一支 0.4 mm 黑色针管笔为起点）；**复制**（Copy）会在你自己的笔中复制所选的笔；**删除**（Delete）会删除你自己的一支笔。字段中的更改会立即保存。
* **导入…**（Import…）从文件（`.json`）读取笔；每支笔都会获得唯一的名称（名称已被占用时会添加编号），无效的笔会被跳过，并在消息中列出。**导出…**（Export…）将你自己的所有笔写入文件；**导出此笔…**（Export this pen…）只写入所选的那一支（现成的笔也可以）。
* 你的笔与程序设置一起保存（最多 200 支）。项目会携带其图层所使用的你的笔：在另一台计算机上打开时，它们会加入那里的 **我的笔**。

## 换笔

图层的笔由其 **颜色**、**笔宽** 和笔库中的 **笔** 共同决定。各图层按列表顺序绘制；当下一个图层在这三者中任何一项上与上一个不同时，就会换笔。笔、颜色和宽度都相同的两个图层会连续绘制，中间不停；仅颜色不同的两个图层会停下来换笔。

* **不设换笔位置**（默认）：笔抬起，作业在原地停止，并发出 `M0 (Change the pen to: …)`。
* **设有换笔位置**（设备设置，**笔**（Pen）页面，**在固定位置换笔**（Change the pen at a fixed place））：笔抬起，绘图仪移动到该位置，发送 **该位置的 G-code**（G-code at the place）（用于换笔机构；其中可以使用变量 `{layer_name}`、`{layer_index}` 和 `{total_layers}`），并在勾选 **在此停止 (M0)**（Stop there (M0)）时以 `M0 (Change the pen to: …)` 停止。不勾选时只写入注释 `(Change the pen to: …)`，作业继续进行。前往该位置的路径会计入预览和时间。

*Change the pen to:* 之后的文本会列出笔库中的笔和图层（`Fineliner 0.4 mm, Black pen`），或者在没有笔库中的笔时，列出图层及其宽度（`Red pen, 0.4 mm`）。当作业从程序发送时，停止会打开 **作业已暂停**（The job is paused）窗口并显示该文本：换好笔后按 **继续作业**（Continue the job）（或 **停止作业**（Stop the job））。参见[机器控制](Machine-Control-ZH)。

使用作业选项 **一次一个元素**（One element at a time）（控制标签页）时，一个元素会连同其所有图层绘制完后再绘制下一个：如果这些图层使用不同的笔，每次切换都会换笔。

## G-code

本程序生成面向 GRBL 的 G-code。使用默认设置（由 Z 轴抬笔，抬笔 3 mm，落笔 0，等待 150 ms，机器零点在左前方）时，它看起来像这样（← 之后的说明不在文件中）：

```
(PenCreator: name of the project)
(Device: name of the device)
(Note: the lines of the Description tab)
G21 G90 G17
G0 Z3            ← 抬笔（使用舵机时：M3 S<up>；使用自定义 G-code 时：其抬笔行）
G4 P0.15         ← 笔移动后的等待，不为 0 时
(Layer: Black pen)
(Pen: Black pen, 0.4 mm)
G0 X… Y…         ← 抬笔移动
G0 Z0            ← 落笔（使用舵机时：M3 S<down>；使用自定义 G-code 时：其落笔行）
G4 P0.15
G1 X… Y… F3000   ← 线条，按图层的速度
…
(Layer: Red pen)
M0 (Change the pen to: Red pen, 0.4 mm)
…
G0 X0 Y0         ← 抬笔，回到工作区的左前角
M5               ← 仅在使用舵机时
M30
```

* 项目的注释（**描述**（Description）标签页）会在名称之后变成 `(Note: …)` 行：括号变为 `<` `>`，`;` 和 `%` 变为逗号，空行被省略，长行被切成每段 100 个字符，最多写入 60 行。
* 数字以毫米为单位，使用机器坐标：你在设备中选作 **机器零点 (0,0)**（Machine zero (0,0)）的角落即 X0 Y0。
* 设备的 **文件最开头**（Very start of the file）、**文件最末尾**（Very end of the file）和 **自定义文件格式**（Own file format）（**G-code** 页面）以及插件的后处理器最后应用。参见[机器控制](Machine-Control-ZH#运行作业)和[插件](Plugins-ZH)。

## G-code 中的注释

G-code 会在注释中标明各部分的开始和结束，这样文件易于阅读，也能在其中找到中断的绘图：`(Job: …)` 带有图层数、线条数和绘制长度；每个 `(Layer: name)` 下是它在作业中的部分、绘制方式、速度和遍数；`(Fill start: 120 lines)` / `(Fill end)`；`(Line 3 of 12 start: closed, 60.0 mm)` / `(Line 3 end)`，每一遍还有 `(Pass 2 of 3)`；使用 **一次一个元素** 时有 `(Element 1 of 4 start)`；`(End of layer name)` 和 `(End of the job)`。一条注释最多 70 个字符（按 GRBL 缓冲区的 128 个字符计数的发送程序绝不会发送更长的行）。设备设置中的 **删除注释**（Remove comments）会将它们全部省略。
