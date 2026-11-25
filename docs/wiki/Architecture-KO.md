🌐 [English](Architecture) · [Polski](Architektura) · [简体中文](Architecture-ZH) · **한국어**

> ⚠️ 이 페이지는 영어판을 기계 번역한 것으로, 부정확한 부분이 있을 수 있습니다. 의심스러우면 [영어판](Architecture)을 기준으로 하십시오. 수정 제안을 환영합니다.

# 아키텍처

PenCreator는 **baseCreator** 위에 놓인 바이너리 크레이트 하나 `crates/pencreator`입니다(`base-core`: 기하, 장면, SVG / PDF / DXF / LightBurn / UGS 리더, 프로젝트 컨테이너, G-code 도우미, 기계 연결. `base-ui`: 창, 메뉴, 편집기, 창과 탭, 클립아트, 카메라, 제어 탭, G-code 뷰어와 G-code 텍스트 탭, 플러그인). baseCreator는 git 태그로 고정됩니다.

| 파일 | 내용 |
|---|---|
| `main.rs` | 애플리케이션: 열기, 저장, 시작 화면, 바, 사이드 패널, 단축키, 창. |
| `menu.rs` | 메뉴 항목(배치는 base-ui의 `standard_menus`)과 두 항목이 같은 단축키를 쓰지 않는지 확인하는 테스트. |
| `native_menu.rs` | macOS 메뉴 바. |
| `workspaces.rs` | 탭으로 열린 도면. |
| `panes_glue.rs` | 작업 영역의 창과 프로젝트의 G-code 텍스트 탭. |
| `device.rs`, `device_ui.rs` | 플로터, 시작 화면 목록, 장치 설정 창, 장치 탭과 카메라 탭. |
| `layers.rs`, `layers_ui.rs` | 레이어(펜)와 레이어 탭. |
| `layer_presets.rs` | 레이어 프리셋이 담는 내용. |
| `pens.rs`, `pens_ui.rs` | 펜 라이브러리와 그 창, 그리고 미리 보기에서 펜이 선을 그리는 방식. |
| `plot.rs` | 획, 채우기, 선으로 그리는 그림, 순서, 펜 교체, G-code. |
| `preview_ui.rs` | 미리 보기, 공구 경로, G-code 미리 보기, G-code 내보내기. |
| `machine_ui.rs` | 기계 연결: 연결, 시작, 프레임, 검사, 펜 버튼. |
| `section.rs` | 프로젝트 파일의 `pen` 섹션. |
| `ugs_import.rs` | UGS Designer 디자인의 레이어. |
| `gcode_view_glue.rs`, `plugins_glue.rs`, `autosave_glue.rs`, `backup_ui.rs`, `update_glue.rs` | 프로그램에 연결된 G-code 뷰어, 플러그인, 자동 저장, 백업, 업데이트 확인. |
| `texts.rs`, `warn.rs` | PenCreator 자체 번역과 프로그램의 경고. |
| `screenshots.rs` | 그림 렌더러(`cargo test -p pencreator render_screenshots -- --ignored`). |
| `grbl_sim_tests.rs` | PC용으로 컴파일한 GRBL 1.1h(grbl-sim)에 대한 작업 테스트. |
