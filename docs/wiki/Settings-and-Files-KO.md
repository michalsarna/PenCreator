🌐 [English](Settings-and-Files) · [Polski](Ustawienia-i-pliki) · [简体中文](Settings-and-Files-ZH) · **한국어**

> ⚠️ 이 페이지는 영어판을 기계 번역한 것으로, 부정확한 부분이 있을 수 있습니다. 의심스러우면 [영어판](Settings-and-Files)을 기준으로 하십시오. 수정 제안을 환영합니다.

# 설정과 파일

PenCreator의 설정은 `app.ron` 파일 하나에 저장됩니다: 언어, 색 구성표, 격자와 보기 옵션, 최근 파일, **내 장치**(키 `devices`), **내 펜**(키 `pens`), 사이드 패널의 탭 순서와 숨김 여부, 창 배치, 업데이트 확인 여부.

| 시스템 | 위치 |
|---|---|
| macOS | `~/Library/Application Support/baseCreator/apps/pencreator/app.ron` |
| Windows | `%APPDATA%\baseCreator\apps\pencreator\app.ron` |
| Linux | `$XDG_DATA_HOME/baseCreator/apps/pencreator/app.ron`, 기본값은 `~/.local/share/baseCreator/apps/pencreator/app.ron` |

* **공유 폴더.** LightCreator, MillCreator, PenCreator는 데이터를 `baseCreator` 폴더 하나에 보관합니다(표는 그 안에 있는 프로그램 자체 설정의 위치입니다). 다른 폴더를 쓰려면 환경 변수 `BASECREATOR_DIR`을 설정합니다. 그 안에는:
  * `clipart/`: 프로그램들이 **공유하는 클립아트 갤러리**. 한 프로그램에서 저장하거나 내려받은 그림이 다른 프로그램에도 나타납니다(이름, 분류, 출처, 라이선스, 작성자를 기록한 `index.json` 포함).
  * `apps/pencreator/app.ron`: 위의 설정. `apps/pencreator/firmware-backups/`: 세션의 첫 변경 전에 만든 컨트롤러 설정의 복사본.
  * `pen_layer_presets.json`: [레이어 프리셋](Layers-and-Pens-KO#레이어-프리셋).
  * `saved_positions.json`: 제어 탭의 저장된 위치.
  * `camera_recording.json`: 카메라의 폴더, 동영상 형식, 타임랩스 설정. `camera/`: 아직 파일이 없는 프로젝트의 사진과 동영상.
  * `plugins/`: [플러그인](Plugins-KO).
* **장치**는 파일로 저장했다가 다시 읽을 수 있습니다: 장치 설정 창의 **내보내기…**(Export…), 시작 화면의 **가져오기…**(Import…).
* **펜**은 **플로터 → 펜 라이브러리…**(Plotter → Pen library…)에서 파일로 저장했다가 다시 읽을 수 있습니다(**내보내기…**, **이 펜 내보내기…**(Export this pen…), **가져오기…**).
* 설정은 30초마다, 그리고 프로그램을 닫을 때 기록됩니다. **모든 것을 초기화하려면 `app.ron`을 삭제하십시오.**
* **자동 저장:** 변경 후 약 20초마다(그리고 프로그램을 닫을 때) 파일이 있는 프로젝트는 그 파일에 기록되고, 아직 파일이 없는 도면은 공유 폴더의 복구 파일에 기록됩니다. 다음에 시작할 때 그런 작업을 제안합니다(*저장되지 않은 프로젝트를 찾았습니다*(A project that was not saved was found): **복구**(Recover) 또는 **버리기**(Discard it)). 도면을 이름을 붙여 저장하면 복구 파일이 지워집니다.
* **도움말 → 백업 다운로드…**(Help → Download backup…)는 PenCreator의 설정(장치 목록, 내 펜, 언어, 색 구성표, 보기 옵션, 최근 파일)과 공유 클립아트 갤러리 전체를 README와 함께 zip으로 저장합니다. **도움말 → 백업에서 복원…**(Help → Restore from backup…)은 확인 후 그런 파일을 다시 읽습니다. 설정은 바뀌고 그림은 갤러리에 **추가**되므로 그동안 다른 프로그램이 저장한 것은 잃지 않습니다. 다른 프로그램이 만든 백업은 거부됩니다. LightCreator와 MillCreator에도 같은 두 항목이 있습니다. 레이어 프리셋도 zip에 들어 있습니다. 복원하면 현재 목록을 대신하며, 그것이 없는 오래된 백업은 목록을 그대로 둡니다. 제어 장치도 zip에 들어 있습니다. 복원하면 세 프로그램 모두에서 현재 것을 대신합니다.
* 프로젝트(`.cre`), G-code, 내보낸 SVG나 DXF는 원하는 곳에 저장됩니다. 카메라의 사진과 동영상은 프로젝트 옆에 저장됩니다([기계 제어](Machine-Control-KO#카메라-rtsp-사진-동영상-타임랩스) 참조).
* **설정**(Settings) 메뉴에는 **보기 옵션…**(View options…), **프로그램 옵션…**(Program options…)(**프로그램 시작 시 새 버전 확인**(Look for a newer version when the program starts)), **플러그인…**(Plugins…), **언어**(Language)(30), **색 구성표**(Colour scheme)(14)가 있습니다.
