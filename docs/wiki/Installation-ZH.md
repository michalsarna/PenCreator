🌐 [English](Installation) · [Polski](Instalacja) · **简体中文** · [한국어](Installation-KO)

> ⚠️ 本页由英文版机器翻译而成，可能有不准确之处。如有疑问，请以[英文版](Installation)为准。欢迎提出修改建议。

# 安装

[Releases 页面](https://github.com/michalsarna/PenCreator/releases)上的每个版本都包含适用于各系统的现成安装包和压缩包，以及用于校验下载文件的 `SHA256SUMS.txt`（`sha256sum -c SHA256SUMS.txt`）。版本是不可变的：一旦发布，其文件就无法更改。

| 系统 | 安装程序 / 软件包 | 压缩包 |
|---|---|---|
| Windows 10/11（64 位） | `pencreator-vX.Y.Z-windows-x86_64-setup.exe` | `pencreator-vX.Y.Z-windows-x86_64.zip` |
| macOS，Apple 芯片（M1 及更新） | `pencreator-vX.Y.Z-macos-arm64.dmg` | – |
| macOS，Intel | `pencreator-vX.Y.Z-macos-x86_64.dmg` | – |
| Debian / Ubuntu 及衍生版，x86_64 | `pencreator-vX.Y.Z-linux-amd64.deb` | `pencreator-vX.Y.Z-linux-x86_64.tar.gz` |
| Debian / Ubuntu 及衍生版，ARM64 | `pencreator-vX.Y.Z-linux-arm64.deb` | `pencreator-vX.Y.Z-linux-arm64.tar.gz` |
| Fedora、openSUSE，x86_64 | `pencreator-vX.Y.Z-linux-x86_64.rpm` | `pencreator-vX.Y.Z-linux-x86_64.tar.gz` |
| Fedora、openSUSE，ARM64 | `pencreator-vX.Y.Z-linux-aarch64.rpm` | `pencreator-vX.Y.Z-linux-arm64.tar.gz` |
| 其他 Linux | – | 上面的 `.tar.gz` |
| FreeBSD，x86_64 | – | `pencreator-vX.Y.Z-freebsd-x86_64.tar.gz` |

## 包管理器

也可以通过包管理器获取本程序，包管理器会安装最新版本并负责更新：

| 系统 | 命令 |
|---|---|
| macOS（Homebrew） | `brew install --cask michalsarna/tap/pencreator`（之后用 `brew upgrade`） |
| Windows（Scoop） | `scoop bucket add creator https://github.com/michalsarna/scoop-bucket`，然后 `scoop install pencreator`（之后用 `scoop update pencreator`） |
| Windows（winget） | 待 Microsoft 将软件包收录到其目录后使用 `winget install MichalSarna.PenCreator`（已提交，等待审核） |

Homebrew cask 和 Scoop 清单会自动跟进每个版本。macOS 应用未经公证，因此首次运行时请参阅下面 *macOS* 一节中的说明。Arch Linux（AUR）软件包已准备好，但尚未发布。

## Windows
运行 `-setup.exe` 安装程序（会添加开始菜单项，并在 *设置 → 应用*（Settings → Apps）中提供卸载程序），或将 `.zip` 解压到任意位置并启动 `pencreator.exe`。安装程序没有代码签名，因此 Windows SmartScreen 可能会发出警告：*更多信息 → 仍要运行*（More info → Run anyway）。

## macOS
打开适合你的 Mac 的 `.dmg`（*苹果菜单 → 关于本机*（Apple menu → About This Mac）会告诉你是 Apple 芯片还是 Intel），然后将 *PenCreator* 拖到 *应用程序*（Applications）。该应用只有临时（ad-hoc）签名，未经公证：首次运行时请使用 **右键 → 打开 → 打开**（right-click → Open → Open）。如果 macOS 提示应用"已损坏"，请执行一次以下命令移除隔离标记：

```sh
xattr -dr com.apple.quarantine /Applications/PenCreator.app
```

## Linux
安装 `.deb`（`sudo apt install ./pencreator-vX.Y.Z-linux-amd64.deb`），在 Fedora 或 openSUSE 上安装 `.rpm`（`sudo dnf install ./pencreator-vX.Y.Z-linux-x86_64.rpm`，openSUSE：`sudo zypper install ./...`；它需要较新的 glibc，因此不支持 RHEL / Alma / Rocky 9），或者解压 `.tar.gz` 并运行 `pencreator`。程序需要常见的 X11 / Wayland 和 OpenGL 库（`libxkbcommon-x11-0`、`libgl1`）；`.deb` 和 `.rpm` 已列出这些依赖。

## FreeBSD
解压 `pencreator-vX.Y.Z-freebsd-x86_64.tar.gz` 并运行 `pencreator`。它需要几个软件包以及 X11 或 Wayland 桌面（以 root 身份执行）：

```sh
pkg install libudev-devd libxkbcommon mesa-libs ffmpeg
```

`ffmpeg` 用于网络（IP）摄像头；USB 摄像头目前在 FreeBSD 上还不能使用（摄像头界面会提示）。USB 连接的绘图仪是 `/dev/cuaU0`：将自己加入 `dialer` 组（`pw groupmod dialer -m $USER`，然后重新登录）。程序已在 FreeBSD 15.0（仅 x86_64）上构建并运行了测试；尚未在 FreeBSD 桌面上试用。若要在该系统上从源代码构建：先执行 `pkg install rust pkgconf libudev-devd evdev-proto`。

## 从源代码构建
```sh
git clone https://github.com/michalsarna/PenCreator
cd PenCreator
cargo run --release -p pencreator
```
添加 `--features camera` 可使用计算机的摄像头。baseCreator 由 Cargo 按其标签获取。
