🌐 [English](Settings-and-Files) · [Polski](Ustawienia-i-pliki) · **简体中文** · [한국어](Settings-and-Files-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Settings-and-Files)为准。欢迎提出修改建议。

# 设置与文件

PenCreator 的设置保存在一个文件 `app.ron` 中：语言、配色方案、网格和视图选项、最近的文件、**你的设备**（键 `devices`）、**你自己的笔**（键 `pens`）、侧面板标签页的顺序以及它是否隐藏、窗格布局，以及是否检查更新。

| 系统 | 位置 |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/pencreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\pencreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/pencreator/app.ron`，默认为 `~/.local/share/baseCreator/apps/pencreator/app.ron` |

* **共享文件夹。** LightCreator、MillCreator 和 PenCreator 把数据保存在同一个文件夹 `baseCreator` 中（上表显示的是其中本程序自己的设置）。设置环境变量 `BASECREATOR_DIR` 可使用其他文件夹。其中包含：
  * `clipart/`：各程序 **共享的剪贴画库**：在一个程序中保存或下载的图片会出现在其他程序中（`index.json` 记录名称、类别、来源、许可证和作者）；
  * `apps/pencreator/app.ron`：上述设置；`apps/pencreator/firmware-backups/`：在会话中第一次更改之前创建的控制器设置副本；
  * `pen_layer_presets.json`：[图层预设](Layers-and-Pens-ZH#图层预设)；
  * `saved_positions.json`：控制标签页中已保存的位置；
  * `camera_recording.json`：摄像头的文件夹、视频格式和延时摄影设置；`camera/`：尚无文件的项目的照片和视频；
  * `plugins/`：[插件](Plugins-ZH)。
* **设备** 可以保存到文件并重新读取：设备设置窗口中的 **导出…**（Export…），开始屏幕上的 **导入…**（Import…）。
* **笔** 可以在 **绘图仪 → 笔库…**（Plotter → Pen library…）中保存到文件并重新读取（**导出…**、**导出此笔…**（Export this pen…）、**导入…**）。
* 设置每半分钟以及程序关闭时写入一次。**删除 `app.ron` 即可重置所有设置。**
* **自动保存：** 更改后大约每 20 秒（以及程序关闭时），有文件的项目会写入该文件；尚无文件的绘图会写入共享文件夹中的恢复文件。下次启动时会提供这些工作（*发现一个未保存的项目*（A project that was not saved was found）：**恢复**（Recover）或 **丢弃**（Discard it））；以某个名称保存绘图会删除其恢复文件。
* **帮助 → 下载备份…**（Help → Download backup…）保存一个 zip，其中包含 PenCreator 的设置（设备列表、你的笔、语言、配色方案、视图选项、最近的文件）和整个共享剪贴画库，并附带一个 README。**帮助 → 从备份恢复…**（Help → Restore from backup…）经确认后读回这样的文件：设置会被替换，图片会被 **添加** 到图库中，因此其他程序在此期间保存的内容不会丢失。由其他程序创建的备份会被拒绝。LightCreator 和 MillCreator 也有同样的两个菜单项。图层预设也在 zip 中；恢复时会替换当前列表，不含图层预设的旧备份不会改动该列表。控制设备也在 zip 中；恢复时会在全部三个程序中替换当前的控制设备。
* 项目（`.cre`）、G-code 以及导出的 SVG 或 DXF 保存到你选择的任何位置。摄像头的照片和视频放在项目旁边（参见[机器控制](Machine-Control-ZH#摄像头rtsp照片视频和延时摄影)）。
* **设置**（Settings）菜单中有 **视图选项…**（View options…）、**程序选项…**（Program options…）（**程序启动时检查新版本**（Look for a newer version when the program starts））、**插件…**（Plugins…）、**语言**（Language）（30 种）和 **配色方案**（Colour scheme）（14 种）。
