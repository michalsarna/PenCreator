🌐 [English](Keyboard-Shortcuts) · [Polski](Skroty-klawiszowe) · **简体中文** · [한국어](Keyboard-Shortcuts-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Keyboard-Shortcuts)为准。欢迎提出修改建议。

# 键盘快捷键

在 Mac 上 Ctrl 即 Cmd。快捷键在编辑器中有效（开始屏幕上无效），并且当文本字段获得键盘焦点时，任何快捷键都不会被占用：在字段、**描述**（Description）标签页或 G-code 文本标签页中输入时，键入的就是字母。在 Mac 上，系统菜单栏不会显示所有快捷键；参见[在 macOS 上](#在-macos-上)。

## 菜单快捷键

这些快捷键显示在菜单项旁边（参见[菜单](Menus-ZH)）。在窗口的菜单栏中，即使菜单关闭，它们也有效；当时为灰色的菜单项不会执行任何操作。

| 快捷键 | 命令 |
|---|---|
| Ctrl+N | **文件 → 新建**（File → New） |
| Ctrl+O | **文件 → 打开…**（File → Open…） |
| Ctrl+S | **文件 → 保存**（File → Save） |
| Ctrl+Shift+S | **文件 → 另存为…**（File → Save as…） |
| Ctrl+Q | **文件 → 退出**（File → Quit） |
| Ctrl+Z | **编辑 → 撤销**（Edit → Undo） |
| Ctrl+Y 或 Ctrl+Shift+Z | **编辑 → 重做**（Edit → Redo） |
| Ctrl+C | **编辑 → 复制**（Edit → Copy） |
| Ctrl+V | **编辑 → 粘贴**（Edit → Paste） |
| Ctrl+D | **编辑 → 再制**（Edit → Duplicate） |
| Delete 或 Backspace | **编辑 → 删除**（Edit → Delete）（使用 **节点编辑 (N)**（Node edit (N)）工具并选中节点时：删除节点） |
| Ctrl+A | **编辑 → 全选**（Edit → Select all） |
| Ctrl+G | **排列 → 编组**（Arrange → Group） |
| Ctrl+Shift+G | **排列 → 取消编组**（Arrange → Ungroup） |
| Ctrl+L | **排列 → 锁定**（Arrange → Lock） |
| Ctrl+Shift+L | **排列 → 解锁**（Arrange → Unlock） |
| N | **排列 → 编辑节点**（Arrange → Edit nodes）（**节点编辑 (N)** 工具） |
| Ctrl++（Ctrl+=） | **视图 → 放大**（View → Zoom in） |
| Ctrl+- | **视图 → 缩小**（View → Zoom out） |
| Ctrl+Shift+0 | **视图 → 工作台适合窗口**（View → Fit bed to window） |
| Ctrl+Shift+9 | **视图 → 适合所有对象**（View → Fit all objects） |
| Ctrl+Shift+P | **绘图仪 → 作业绘图…**（Plotter → Plot of the job…） |
| Ctrl+E | **绘图仪 → 导出 G-code…**（Plotter → Export G-code…） |

任何两个菜单项都不共用同一个快捷键；程序中有一项测试专门检查这一点。

## 绘图标签页

| 快捷键 | 作用 |
|---|---|
| Ctrl+1 … Ctrl+9、Ctrl+0 | 显示第 1 … 9 个标签页的绘图，Ctrl+0 显示第 10 个（最多打开 10 个绘图）。 |
| Ctrl+W | 关闭当前显示的绘图；如果有未保存的工作，会先询问（**保存**（Save）、**放弃**（Discard）、**取消**（Cancel））。 |

## 工具

不带 Ctrl 的字母会选择左侧工具栏中的工具（字母显示在其按钮的提示中）：

| 快捷键 | 工具 | 快捷键 | 工具 |
|---|---|---|---|
| V | **选择 (V)**（Select (V)） | L | **直线 (L)**（Line (L)） |
| N | **节点编辑 (N)**（Node edit (N)） | P | **折线 / 钢笔 (P)**（Polyline / pen (P)） |
| R | **矩形 (R)**（Rectangle (R)） | T | **文本 (T)**（Text (T)） |
| E | **椭圆 (E)**（Ellipse (E)） | H | **平移 (H)**（Pan (H)） |
| Y | **三角形 (Y)**（Triangle (Y)） | Z | **放大 (Z)**（Zoom in (Z)） |
| S | **星形 (S)**（Star (S)） | X | **缩小 (X)**（Zoom out (X)） |
| G | **多边形 (G)**（Polygon (G)） | | |
| K | **心形 (K)**（Heart (K)） | M | **标尺 (M)**（Ruler (M)） |
| I | **螺旋 (I)**（Spiral (I)） | | |

| 快捷键 | 作用 |
|---|---|
| Esc | 清除选择（以及正在绘制的折线）。 |
| 按两次 Esc（半秒内） | 回到 **选择 (V)** 工具。 |
| 空格（按住） | 使用任何工具时平移；松开后回到原来的工具。 |
| 方向键 | 将所选内容移动 1 mm；**Shift** + 方向键 = 10 mm（在控制标签页上勾选 **键盘**（Keyboard）时无效）。 |
| G、I | 多边形和螺旋：一个小窗口用于设置边数或圈数，也适用于最近绘制的形状。 |
| Enter | 结束用 **折线 / 钢笔 (P)** 正在绘制的折线。 |

## 鼠标配合按键

| 操作 | 作用 |
|---|---|
| 在工作区上滚动滚轮（或双指捏合） | 以指针为中心缩放。 |
| 按住中键或右键拖动 | 平移视图，在工作区上及其旁边均可（使用 **平移 (H)** 时用左键；没有移动的右键单击会打开菜单）。 |
| Shift+单击或 Ctrl+单击 | 将形状加入选择，或将其移出选择。 |
| 绘制矩形、椭圆、三角形、星形、多边形、心形或螺旋时按住 Shift | 保持宽度和高度相等。 |
| 拖动角上的控制柄 | 保持比例；按住 Shift 时宽度和高度可自由改变。 |
| 旋转时按住 Shift | 以 15° 为步长旋转。 |
| 使用 **放大 (Z)** / **缩小 (X)** 时按住 Shift | 反向缩放。 |
| 使用 **折线 / 钢笔 (P)** 时右键单击或双击 | 结束折线；在第一个点附近单击会将其闭合。 |
| 在形状上右键单击 | 形状的菜单（参见[设计工具](Design-Tools-ZH)）。 |
| 在窗格的标签页上单击中键 | 关闭该标签页。 |
| 双击窗格之间的分隔线 | 使窗格等大。 |
| 在工作区下方的图层行上滚动滚轮 | 横向滚动该行。 |

在 **预览**（Preview）标签页中，滚轮缩放，拖动移动画面，双击回到初始视图。

## 控制标签页

在 **控制**（Control）标签页上勾选 **键盘**（Keyboard）（默认关闭）后，方向键移动 X 和 Y，Page Up / Page Down 移动 Z，步长为那里选择的值。参见[机器控制](Machine-Control-ZH)。

## 在 macOS 上

菜单位于 macOS 的系统菜单栏中。本页的所有快捷键在那里也都有效：

* **Cmd+N**、**Cmd+O**、**Cmd+S**、**Cmd+Shift+S** 和 **Cmd+Q**（在应用菜单 **PenCreator** 中）属于系统菜单栏，它会在相应菜单项旁显示这些快捷键。
* 其他菜单快捷键（**Cmd+E**、**Cmd+Shift+P**、**Cmd+L**、**Cmd+Shift+L**、**Cmd+=** / **Cmd+-**、**Cmd+Shift+0** / **Cmd+Shift+9**、编辑快捷键）由程序在自己的窗口中处理。它们的作用与在 Windows 和 Linux 上相同，但系统菜单栏不会在菜单项旁显示它们；上面的表格列出了这些快捷键。
