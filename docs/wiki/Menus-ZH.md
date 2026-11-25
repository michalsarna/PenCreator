🌐 [English](Menus) · [Polski](Menu) · **简体中文** · [한국어](Menus-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Menus)为准。欢迎提出修改建议。

# 菜单

菜单栏有九个菜单，顺序如下：**文件 · 编辑 · 插入 · 排列 · 视图 · 机器 · 绘图仪 · 设置 · 帮助**（File · Edit · Insert · Arrange · View · Machine · Plotter · Settings · Help）。LightCreator 和 MillCreator 的布局相同（来自 baseCreator）；只有加工过程的菜单不同（**激光**（Laser）、**铣削**（Milling）、**绘图仪**（Plotter））。

* 在 **Windows 和 Linux** 上，菜单栏位于窗口内、标题栏下方。它在编辑器中显示，不在开始屏幕上显示。
* 在 **macOS** 上，菜单位于屏幕顶部的系统菜单栏中。它多两个菜单：应用菜单 **PenCreator**（**关于 PenCreator**（About PenCreator）、系统的常用菜单项、**退出**（Quit））和 **窗口**（Window）（最小化和最大化）。因此在 Mac 上，**退出** 不在 **文件** 的末尾，**关于** 也不在 **帮助** 的末尾。
* 当前无法使用的菜单项显示为灰色（例如未选中图片时的 **描摹图像…**（Trace image…），开始屏幕上的 **保存**（Save））。开关在开启时带有勾号。
* 快捷键列在菜单项旁边；全部快捷键见[键盘快捷键](Keyboard-Shortcuts-ZH)页面。它们在所有系统上都有效；在 macOS 上，系统菜单栏只显示 **新建**（New）、**打开…**（Open…）、**保存**、**另存为…**（Save as…）和 **退出** 的快捷键。

## 文件

| 菜单项 | 快捷键 | 作用 |
|---|---|---|
| **新建**（New） | Ctrl+N | 在设备的纸张上新建一个空白绘图，位于单独的标签页中（同时最多打开 10 个绘图）。 |
| **打开…**（Open…） | Ctrl+O | 在单独的标签页中打开项目（`.cre`）、SVG、PDF 或 Illustrator（`.ai`）文件；DXF 或 UGS 设计（`.ugsd`）会被导入到绘图中。已打开的项目会被显示，而不会打开两次。参见[快速入门](Quick-Start-ZH)和[项目文件](Project-File-ZH)。 |
| **最近打开**（Open recent） | | 最近的 10 个文件；**清除列表**（Clear list）会清空列表。 |
| **保存**（Save） | Ctrl+S | 将项目写为 `.cre`。尚无项目文件的绘图（新建的，或从 SVG 或 PDF 打开的）会先询问名称。 |
| **另存为…**（Save as…） | Ctrl+Shift+S | 以新名称写入项目。 |
| **项目信息…**（Project information…） | | 文件（名称、文件夹、大小、创建和修改时间）、项目的创建时间、创建者、贡献者和编辑记录。每次手动保存都会记录 **设置 → 程序选项 → 作者**（Settings → Program options → Author）中设置的人员（姓名、电子邮件、其他；在所有程序中相同）、时间以及程序及其版本，最多每分钟一次。 创建者保存全部三项，贡献者只保存姓名和电子邮件（电子邮件与创建者相同时只保存姓名）。 |
| **导入 → 导入图像…**（Import → Import image…） | | 通过导入对话框导入图片（PNG、JPG、BMP、GIF、WebP）。参见[设计工具](Design-Tools-ZH)。 |
| **导入 → 导入 SVG…**（Import → Import SVG…） | Ctrl+I | SVG 中的路径被添加到打开的绘图中，每条路径放到颜色最接近的图层上，其中的图片放到当前图层上。**打开…** 则会为该文件新建一个标签页。 |
| **导入 → 导入 Adobe Illustrator / PDF / EPS / VectorCraft…**（Import → Import Adobe Illustrator / PDF / EPS / VectorCraft…） | | 对 PDF、兼容 PDF 的 `.ai`、旧 PostScript 格式的 `.ai`、EPS 或 VectorCraft 绘图（`.vectorcraft`，含其图片）执行同样的操作。含有 Illustrator 图层的文件会询问按颜色还是按这些图层导入。 |
| **导入 → 导入 DXF…**（Import → Import DXF…） | | DXF 绘图中的直线、圆弧和曲线被添加到绘图中。 |
| **导入 → 导入 LightBurn / MillMage 项目…**（Import → Import LightBurn / MillMage project…） | | LightBurn 项目（`.lbrn2`、`.lbrn`）或 MillMage 项目（`.mage`）中的形状被添加到绘图中。 |
| **导入 → 导入 UGS 设计…**（Import → Import UGS design…） | | UGS Designer 文件（`.ugsd`）中的形状被添加到绘图中。 |
| **导入 → 使用插件导入…**（Import → Import with a plugin…） | | 已安装插件可读取的格式的文件。参见[插件](Plugins-ZH)。 |
| **导入 → 生成形状…**（Import → Generate shape…） | | 已安装插件的生成器。 |
| **导出 → 导出 G-code…**（Export → Export G-code…） | | 与 **绘图仪 → 导出 G-code…**（Plotter → Export G-code…）相同。 |
| **导出 → 导出 SVG…**（Export → Export SVG…） | | 将整个绘图导出为 SVG，尺寸为工作区大小。曲线和图片会被保留。 |
| **导出 → 导出 PLT / DXF…**（Export → Export PLT / DXF…） | | 按你选择的扩展名，将整个绘图的轮廓导出为 HPGL（`.plt`，每个图层一支笔）或 DXF R12。这些是形状，而不是机器的移动。 |
| **导出 → 导出 DXF…**（Export → Export DXF…） | | 将整个绘图导出为当前格式的 DXF（AutoCAD 2007）；曲线展平到 0.01 mm。 |
| **导出 → 导出 DXF R12（legacy）…**（Export → Export DXF R12 (legacy)…） | | 同上，但为 DXF R12，用于较旧的程序和切割机（也称 *legacy*）。 |
| **导出 → 导出 Adobe Illustrator…**（Export → Export Adobe Illustrator…） | | 将轮廓导出为 PostScript 格式的 Illustrator 文件，保留曲线，项目的每个图层对应一个 Illustrator 图层。图片会被省略。 |
| **导出 → 导出 LightBurn 项目…**（Export → Export LightBurn project…） | | LightBurn 项目（`.lbrn2`，或者在你这样命名文件时为 `.lbrn`）：带曲线的形状和图片，项目的每个图层对应一个 LightBurn 图层。 |
| **导出 → 导出 MillMage 项目…**（Export → Export MillMage project…） | | 同上，导出为 MillMage 项目（`.mage`），材料尺寸为工作区大小。 |
| **导出 → 将所选导出为 SVG…** / **将所选导出为 DXF…**（Export → Export selection as SVG… / Export selection as DXF…） | | 将选中的形状（没有选中任何内容时为全部）写为 SVG 或 DXF。 |
| **查看 G-code 文件…**（View G-code file…） | | 在 **G-code 分析**（G-code analysis）标签页中打开 G-code 文件（`.gcode`、`.nc`、`.gc`、`.gco`、`.ngc`、`.tap`、`.cnc`）：显示其刀具路径，不改变绘图。 |
| **打印…**（Print…） | Ctrl+P | 在纸张打印机上打印设计，或将其保存为 PDF。 |
| **退出**（Quit） | Ctrl+Q | 关闭程序；每个有未保存工作的绘图都会先询问。仅限 Windows 和 Linux（在 macOS 上它位于应用菜单中）。 |

## 编辑

| 菜单项 | 快捷键 | 作用 |
|---|---|---|
| **撤销**（Undo） | Ctrl+Z | 撤回对绘图的上一次更改（最多 100 步）。 |
| **重做**（Redo） | Ctrl+Y | 重新执行（Ctrl+Shift+Z 也可以）。 |
| **复制**（Copy） | Ctrl+C | 复制选中的形状。 |
| **粘贴**（Paste） | Ctrl+V | 粘贴复制的形状。 |
| **再制**（Duplicate） | Ctrl+D | 复制选中的形状并立即粘贴。 |
| **删除**（Delete） | Del | 删除选中的形状（使用 **节点编辑 (N)**（Node edit (N)）工具时，删除选中的节点）。 |
| **全选**（Select all） | Ctrl+A | 选中所有显示中的图层上的每个形状。 |

## 插入

左侧工具栏中用于在页面上放置新对象的工具：**矩形 (R)**（Rectangle (R)）、**椭圆 (E)**（Ellipse (E)）、**三角形 (Y)**（Triangle (Y)）、**星形 (S)**（Star (S)）、**多边形 (G)**（Polygon (G)）、**心形 (K)**（Heart (K)）、**螺旋 (I)**（Spiral (I)）、**直线 (L)**（Line (L)）、**折线 / 钢笔 (P)**（Polyline / pen (P)）和 **文本 (T)**（Text (T)）。正在使用的工具带勾号。然后是：

* **生成形状…**（Generate shape…）：已安装插件的生成器（与 **文件 → 导入**（File → Import）中的相同）。
* **在线剪贴画…**（Online clipart…）：打开 **剪贴画**（Clipart）标签页，并在互联网上搜索免费图片。

新形状会放到当前图层上（参见[图层与笔](Layers-and-Pens-ZH)）。

## 排列

大多数菜单项作用于选中的形状；同样的命令也在形状的右键菜单以及工作区右侧的排列栏中。它们的说明见[设计工具](Design-Tools-ZH)。

| 菜单项 | 快捷键 | 作用 |
|---|---|---|
| **左对齐**（Align left）、**水平居中对齐 (H)**（Align centre (H)）、**右对齐**（Align right）、**顶对齐**（Align top）、**垂直居中对齐 (V)**（Align centre (V)）、**底对齐**（Align bottom） | | 对齐选中的形状。 |
| **在工作台上居中**（Centre on bed） | | 把所选内容放到工作区中央。 |
| **相互居中**（Centre on each other） | | 把选中的形状（或编组）移到所选内容的中心，彼此重叠（两个或更多）。 |
| **水平翻转**（Flip horizontal）、**垂直翻转**（Flip vertical） | | 镜像所选内容。 |
| **顺时针旋转 90°**（Rotate 90° CW）、**逆时针旋转 90°**（Rotate 90° CCW） | | 将所选内容旋转四分之一圈。 |
| **编组**（Group） | Ctrl+G | 将选中的形状组成一个编组。 |
| **取消编组**（Ungroup） | Ctrl+Shift+G | 拆开编组。 |
| **锁定**（Lock） | Ctrl+L | 锁定的形状无法移动或更改。 |
| **解锁**（Unlock） | Ctrl+Shift+L | 重新释放它们。 |
| **置于顶层**（Bring to front）、**置于底层**（Send to back） | | 改变形状在绘图中的顺序。 |
| **编辑节点**（Edit nodes） | N | 对选中的形状使用 **节点编辑 (N)** 工具。 |
| **转换为路径**（Convert to path） | | 将形状（矩形、星形…）转换为路径，以便编辑其节点。 |
| **并集**（Union）、**交集**（Intersection）、**减去**（Subtract）、**异或**（Exclusive or） | | 合并、裁切和组合封闭形状。 |
| **偏移形状…**（Offset shape…）、**圆角…**（Round corners…）、**转换为曲线**（Convert to curves） | | 更大或更小的轮廓、圆角、用曲线代替直线。 |
| **自动连接路径…**（Auto-join paths…）、**按容差闭合路径…**（Close path with tolerance…） | | 连接端点相近的开放路径，闭合端点相近的路径。 |
| **狗骨 / T 骨转角…**（Dogbone / T-bone corners…） | | 在尖角处开缺口（用于铣削；在三个程序中相同）。 |
| **网格阵列…**（Grid array…）、**环形阵列…**（Circular array…）、**沿路径复制…**（Copy along path…）、**路径文本…**（Text on path…） | | 按网格、绕中心、沿路径复制；沿路径弯曲文本。 |
| **按形状修剪…**（Trim by shape…）、**删除重复**（Remove duplicates）、**拆分**（Break apart）、**简化…**（Simplify…） | | 用形状裁切线条、删除重复项、把形状拆分为各个轮廓、去除多余的点。 |
| **序列文本…**（Serial text…）、**条形码 / 二维码…**（Barcode / QR code…）、**嵌套排料…**（Nest shapes…）、**测量…**（Measure…） | | 编号文本、码、在区域内排布形状、测量。 |
| **调整图像…**（Adjust image…）、**描摹图像…**（Trace image…） | | 修改选中的图片，或将其转换为轮廓。未选中图片时为灰色。 |
| **编辑图像…**（Edit image…） | | 选中图片的照片编辑器：画笔、橡皮擦、仿制图章、填充，仅限灰度。 |
| **转换为图像…**（Convert to image…） | | 将选中的形状（不含图片）转换为一张图片，可设置分辨率和尺寸。 |
| **风格描摹…**（Style trace…） | | 将选中的图片重新绘制为 21 种风格之一的线条。参见[风格描摹](Style-Trace-ZH)。 |
| **几何多边形…**（Geometric polygons…） | | 将选中的图片转换为几何（低多边形）多边形，多边形之间及轮廓周围留有间隙：即打开选中该风格的风格描摹窗口。参见[风格描摹](Style-Trace-ZH)。 |
| **添加为禁入区**（Add as no-go zone） | | 每个未锁定的选中形状都变成设备的一个禁入区（圆形的为圆，否则为其外接矩形），并从绘图中移除。也在右键菜单中。 |

## 视图

| 菜单项 | 快捷键 | 作用 |
|---|---|---|
| **网格**（Grid）、**吸附到网格**（Snap to grid）、**吸附到对象**（Snap to objects） | | 开关（开启时带勾号）：工作区上的网格、吸附到网格、吸附到其他形状。 |
| **放大**（Zoom in） | Ctrl++ | 以视图中心为基准放大。 |
| **缩小**（Zoom out） | Ctrl+- | 缩小。 |
| **工作台适合窗口**（Fit bed to window） | Ctrl+Shift+0 | 显示整个工作区。 |
| **适合所有对象**（Fit all objects） | Ctrl+Shift+9 | 显示所有形状。 |
| **工作区自动适合窗口**（Auto-fit work area to window） | | 开关：窗口改变大小时，工作区保持适合窗口。手动缩放或平移会将其关闭。 |
| **拆分视图：并排**（Split view: side by side）、**拆分视图：上下排列**（Split view: one above the other） | | 将工作区划分为窗格（最多三个）。只有一个窗格时，其最后一个标签页会移到新窗格（当该窗格有两个或更多标签页时）。参见[设计工具](Design-Tools-ZH#标签页与窗格)。 |
| **单一视图**（Single view） | | 把所有标签页重新放到一个窗格中。 |
| **显示摄像头叠加**（Show camera overlay） | | 显示或隐藏绘图下方的图片。 |
| **显示禁入区**（Show no-go zones） | | 以红色绘制设备的禁入区；设备没有禁入区时为灰色。 |
| **预览**（Preview） | | 显示或隐藏预览，与 **预览…**（Preview…）按钮相同。 |
| **预览 G-code**（Preview G-code） | | 在标签页中显示作业的 G-code，与 **预览 G-code** 按钮相同。 |
| **叠加来源…**（Overlay from…） | | 在绘图下方放置照片或摄像头的实时画面，按其四角或参考点对齐。随项目保存。 |

## 机器

| 菜单项 | 作用 |
|---|---|
| **切换设备…**（Switch device…） | 回到开始屏幕选择另一台设备。 |
| **设备设置…**（Device settings…） | 项目所用设备的设置（页面：常规、机器、笔、G-code、连接、摄像头、点动、宏、维护、禁入区、备注（General, Machine, Pen, G-code, Connection, Camera, Jogging, Macros, Maintenance, No-go zones, Notes））。参见[快速入门](Quick-Start-ZH)和[机器控制](Machine-Control-ZH)。 |
| **固件设置 ($$)…**（Firmware settings ($$)…） | GRBL 控制器的设置，带副本和备份。 |
| **校准…**（Calibration…） | 每毫米步数和各轴的垂直度。 |
| **禁入区…**（No-go zones…） | | 打开设备窗口的 **禁入区**（No-go zones）页面。 |
| **控制设备 ▸**（Control devices ▸） | 控制设备设置…（Control device settings…）打开键盘和手柄及其按键的窗口；本机控制设备…（Control device of this machine…）为打开的机器选择控制设备。参见[控制设备](Control-Devices-ZH)。 |
| **诊断与统计…**（Diagnostics and statistics…） | 控制器的输入、机器的统计和维护、可保存的报告。 |
| **通过网络共享 (ser2net)…**（Share over network (ser2net)…） | 仅限 Linux：通过网络共享机器的串口。 |
| **边框**（Frame） | 沿作业的外框走一圈（需要已连接的机器）。 |
| **开始作业**（Start job） | 发送作业（需要已连接的机器）。 |
| **从指定行开始…**（Start from line…） | 打开从程序的某一行开始作业的窗口（当有内容可绘制时）；开始需要已连接的机器。 |
| **检查程序**（Check the program） | 控制器以检查模式（`$C`）运行一遍程序；机器不会移动。需要已连接的机器。 |

它们的详细说明见[机器控制](Machine-Control-ZH)。

## 绘图仪

| 菜单项 | 快捷键 | 作用 |
|---|---|---|
| **笔库…**（Pen library…） | | 现成的笔和你自己的笔。参见[图层与笔](Layers-and-Pens-ZH#笔库)。 |
| **刀具库…**（Knife library…） | | 现成的拖刀和你自己的拖刀。参见[设计工具](Design-Tools-ZH#拖刀)。 |
| **图层预设…**（Layer presets…） | | 图层预设窗口：以某个名称保存当前图层，或将预设应用到它。 |
| **预览 G-code**（Preview G-code） | | 在标签页中以文本显示作业的 G-code，不保存（与 **预览 G-code** 按钮相同）。 |
| **作业绘图…**（Plot of the job…） | Ctrl+Shift+P | 打开 **预览** 标签页：笔将要绘制的内容，包括抬笔移动、时间和动画。参见[快速入门](Quick-Start-ZH)。 |
| **导出 G-code…**（Export G-code…） | Ctrl+E | 询问保存位置并写入项目的 G-code（**保存 G-code**（Save G-code）按钮的作用相同）。 |

## 设置

| 菜单项 | 作用 |
|---|---|
| **视图选项…**（View options…） | 网格、工作区的颜色、线条粗细、**显示图层名称**（Show layer names）、**保存后预览 G-code**（Preview G-code after saving）、**显示变量名**（Show variable names）、**G-code 控制台**（G-code console）的字体大小和颜色。 |
| **程序选项…**（Program options…） | **程序启动时检查新版本**（Look for a newer version when the program starts）（默认开启）；**自动保存**（Auto-save）：自动保存工作（有名称的文件保存到原处，没有名称的保存到恢复文件），以及保存频率（10 秒到一小时，初始为 2 分钟）。 |
| **插件…**（Plugins…） | 安装、启用/停用和删除插件。参见[插件](Plugins-ZH)。 |
| **语言**（Language） | 38 种语言；选择会被保存。 |
| **配色方案**（Colour scheme） | 18 种配色方案（某些节日前后还有一种节日方案）。 |

## 帮助

| 菜单项 | 作用 |
|---|---|
| **GitHub 上的项目页面**（Project page on GitHub） | 在浏览器中打开 PenCreator 的页面。 |
| **下载备份…**（Download backup…） | 将设置和共享剪贴画库保存为 zip。参见[设置与文件](Settings-and-Files-ZH)。 |
| **从备份恢复…**（Restore from backup…） | 经确认后读回这样的 zip。 |
| **快速指南**（Quick guide）（F1） | 程序内置的指南，支持搜索。 |
| **导出诊断信息 (zip)…**（Export diagnostics (zip)…） | 一个包含报告、设置和控制台内容的文件，可附在错误报告中。 |
| **检查更新…**（Check for updates…） | 查找更新的版本。 |
| **关于**（About） | 版本、许可证和致谢。仅限 Windows 和 Linux（在 macOS 上：应用菜单中的 **关于 PenCreator**（About PenCreator））。 |
