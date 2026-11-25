🌐 [English](Installation) · [Polski](Instalacja) · [简体中文](Installation-ZH) · **한국어**

> ⚠️ 이 페이지는 영어판을 기계 번역한 것으로, 부정확한 부분이 있을 수 있습니다. 의심스러우면 [영어판](Installation)을 기준으로 하십시오. 수정 제안을 환영합니다.

# 설치

[Releases 페이지](https://github.com/michalsarna/PenCreator/releases)의 모든 릴리스에는 시스템별로 바로 쓸 수 있는 패키지와 압축 파일, 그리고 다운로드를 검증하기 위한 `SHA256SUMS.txt`(`sha256sum -c SHA256SUMS.txt`)가 들어 있습니다. 릴리스는 변경할 수 없습니다. 한 번 게시되면 파일을 바꿀 수 없습니다.

| 시스템 | 설치 프로그램 / 패키지 | 압축 파일 |
|---|---|---|
| Windows 10/11 (64비트) | `pencreator-vX.Y.Z-windows-x86_64-setup.exe` | `pencreator-vX.Y.Z-windows-x86_64.zip` |
| macOS, Apple silicon (M1 이상) | `pencreator-vX.Y.Z-macos-arm64.dmg` | – |
| macOS, Intel | `pencreator-vX.Y.Z-macos-x86_64.dmg` | – |
| Debian / Ubuntu 및 파생 배포판, x86_64 | `pencreator-vX.Y.Z-linux-amd64.deb` | `pencreator-vX.Y.Z-linux-x86_64.tar.gz` |
| Debian / Ubuntu 및 파생 배포판, ARM64 | `pencreator-vX.Y.Z-linux-arm64.deb` | `pencreator-vX.Y.Z-linux-arm64.tar.gz` |
| Fedora, openSUSE, x86_64 | `pencreator-vX.Y.Z-linux-x86_64.rpm` | `pencreator-vX.Y.Z-linux-x86_64.tar.gz` |
| Fedora, openSUSE, ARM64 | `pencreator-vX.Y.Z-linux-aarch64.rpm` | `pencreator-vX.Y.Z-linux-arm64.tar.gz` |
| 기타 Linux | – | 위의 `.tar.gz` |
| FreeBSD, x86_64 | – | `pencreator-vX.Y.Z-freebsd-x86_64.tar.gz` |

## 패키지 관리자

릴리스는 패키지 관리자로도 받을 수 있으며, 패키지 관리자는 최신 버전을 설치하고 업데이트합니다.

| 시스템 | 명령 |
|---|---|
| macOS (Homebrew) | `brew install --cask michalsarna/tap/pencreator` (이후 `brew upgrade`) |
| Windows (Scoop) | `scoop bucket add creator https://github.com/michalsarna/scoop-bucket` 다음에 `scoop install pencreator` (이후 `scoop update pencreator`) |
| Windows (winget) | Microsoft가 패키지를 카탈로그에 승인한 뒤 `winget install MichalSarna.PenCreator` (제출됨, 검토 대기 중) |

Homebrew cask와 Scoop 매니페스트는 릴리스마다 자동으로 갱신됩니다. macOS 앱은 공증(notarize)되지 않았으므로 처음 실행할 때는 아래 *macOS* 섹션의 설명을 참고하십시오. Arch Linux(AUR) 패키지는 준비되었지만 아직 게시되지 않았습니다.

## Windows
`-setup.exe` 설치 프로그램을 실행하거나(시작 메뉴 항목과 *설정 → 앱*(Settings → Apps)의 제거 프로그램이 생깁니다) `.zip`을 아무 곳에나 풀고 `pencreator.exe`를 실행합니다. 설치 프로그램은 코드 서명되지 않았으므로 Windows SmartScreen이 경고할 수 있습니다. 이때는 *추가 정보 → 실행*(More info → Run anyway)을 누릅니다.

## macOS
사용하는 Mac에 맞는 `.dmg`를 열고(*Apple 메뉴 → 이 Mac에 관하여*(Apple menu → About This Mac)에서 Apple silicon인지 Intel인지 확인할 수 있습니다) *PenCreator*를 *응용 프로그램*(Applications)으로 끌어 놓습니다. 앱은 ad-hoc 서명만 되어 있고 공증되지 않았으므로 처음에는 **오른쪽 클릭 → 열기 → 열기**(right-click → Open → Open)를 사용합니다. macOS가 앱이 "손상되었다"고 하면 격리 플래그를 한 번 제거합니다.

```sh
xattr -dr com.apple.quarantine /Applications/PenCreator.app
```

## Linux
`.deb`를 설치하거나(`sudo apt install ./pencreator-vX.Y.Z-linux-amd64.deb`), Fedora나 openSUSE에서는 `.rpm`을 설치하거나(`sudo dnf install ./pencreator-vX.Y.Z-linux-x86_64.rpm`, openSUSE: `sudo zypper install ./...`; 최신 glibc가 필요하므로 RHEL / Alma / Rocky 9에서는 안 됩니다), `.tar.gz`를 풀고 `pencreator`를 실행합니다. 프로그램에는 일반적인 X11 / Wayland 및 OpenGL 라이브러리(`libxkbcommon-x11-0`, `libgl1`)가 필요하며, `.deb`와 `.rpm`에는 이들이 의존성으로 적혀 있습니다.

## FreeBSD
`pencreator-vX.Y.Z-freebsd-x86_64.tar.gz`를 풀고 `pencreator`를 실행합니다. 몇 가지 패키지와 X11 또는 Wayland 데스크톱이 필요합니다(root로 실행).

```sh
pkg install libudev-devd libxkbcommon mesa-libs ffmpeg
```

`ffmpeg`는 네트워크(IP) 카메라용입니다. USB 카메라는 아직 FreeBSD에서 작동하지 않습니다(카메라 화면에 그렇게 표시됩니다). USB로 연결한 플로터는 `/dev/cuaU0`입니다. 자신을 `dialer` 그룹에 추가한 다음(`pw groupmod dialer -m $USER`) 다시 로그인합니다. FreeBSD 15.0(x86_64만)에서 빌드하고 테스트를 실행했으며, FreeBSD 데스크톱에서는 아직 시험하지 않았습니다. 그곳에서 소스로 빌드하려면 먼저 `pkg install rust pkgconf libudev-devd evdev-proto`를 실행합니다.

## 소스에서 빌드
```sh
git clone https://github.com/michalsarna/PenCreator
cd PenCreator
cargo run --release -p pencreator
```
컴퓨터의 카메라를 쓰려면 `--features camera`를 추가합니다. baseCreator는 Cargo가 해당 태그에서 가져옵니다.
