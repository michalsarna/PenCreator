🌐 [English](Safety-and-Limitations) · [Polski](Bezpieczenstwo-i-ograniczenia) · **简体中文** · [한국어](Safety-and-Limitations-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Safety-and-Limitations)为准。欢迎提出修改建议。

# 安全与限制

> 已确认 PenCreator 可正常工作的机器，以及如何添加你的机器：[TESTS_ON_DEVICES.md](https://github.com/michalsarna/PenCreator/blob/main/TESTS_ON_DEVICES.md)。

PenCreator 生成绘图路径并写出 **面向 GRBL 的 G-code**。它可以 **连接到 GRBL 绘图仪并流式发送作业**（设备标签页：**连接**（Connect）；控制标签页：**开始**（Start）、**STOP**、**边框**（Frame）、**从指定行…**（From line…）），你也可以保存文件并用你自己的程序发送。G-code 已经过自动测试和 GRBL 模拟器的检查（每次抬笔移动之前笔都已抬起，换笔时以 `M0` 停止），并且 PenCreator 已在一台按照 [Drawing Robot](https://www.printables.com/model/137296-drawing-robot-arduino-uno-cnc-shield-grbl)（Arduino UNO、CNC Shield、GRBL）制作的笔式绘图仪上测试过。你的机器可能有所不同。**在固定位置换笔尚未在真实的绘图仪上试用。**

* 请谨慎设置设备的 **抬笔**（Pen up）和 **落笔**（Pen down）：使用 Z 轴时，它们是以 mm 为单位的高度（落笔 0 即纸面）；使用舵机时，它们是 `S` 值；使用 **自定义 G-code**（Own G-code）时，它们是你编写的行（例如对电磁铁使用 `M3 S1000` 和 `M5`，或对 Marlin 的舵机使用 `M280 P0 S…`），按原样发送。错误的值可能会把笔压进机器或台面。请先在控制标签页上试用 **抬笔**（Raise the pen）和 **落笔**（Lower the pen），在抬笔或不装笔的情况下运行一次作业，并留在机器旁。
* 笔从设备的机器零点（你选择的角落）开始；第一次移动从那里出发，结束时笔会回到那个角落。
* 点动、笔按钮、**边框** 和换笔暂停都会发送真实的移动。**边框** 会抬起笔并沿绘图的外框走一圈：请检查抬起的笔不会碰到纸和夹具。
* 图片（位图）只在 **绘制方式**（Drawing）为 **图片**（Picture）的图层上绘制；在其他图层上会被省略并发出警告（请先将其描摹为线条，或将图层切换为 **图片**）。文本按字体轮廓绘制（每个字母沿其边缘绘制），而不是用单线字母。
* 填充线是有一定角度的平行线；很小的间距会产生大量线条，超过 200 000 条填充线后，其余的填充会被省略并发出警告。
* 目前还没有单线字体。HPGL 绘图仪和切割机通过串口驱动，但尚未在真实设备上试用。
* 测试内容：编辑器和项目文件（保留其他程序的部分）、文件的打开、设备、图层、笔画顺序、填充、G-code、菜单及其快捷键以及翻译，每次更改都会运行自动测试；作业的 G-code 还会在模拟的 GRBL 控制器中测试。每次更改时，界面也会在无头渲染器中检查。
