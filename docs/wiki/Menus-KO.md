🌐 [English](Menus) · [Polski](Menu) · [简体中文](Menus-ZH) · **한국어**

> ⚠️ 이 페이지는 영어판을 기계 번역한 것으로, 부정확한 부분이 있을 수 있습니다. 의심스러우면 [영어판](Menus)을 기준으로 하십시오. 수정 제안을 환영합니다.

# 메뉴

메뉴 바에는 다음 순서로 메뉴 아홉 개가 있습니다: **파일 · 편집 · 삽입 · 정렬 · 보기 · 기계 · 플로터 · 설정 · 도움말**(File · Edit · Insert · Arrange · View · Machine · Plotter · Settings · Help). LightCreator와 MillCreator도 같은 배치이며(baseCreator에서 옵니다), 공정 메뉴만 다릅니다(**레이저**(Laser), **밀링**(Milling), **플로터**(Plotter)).

* **Windows와 Linux**에서는 메뉴 바가 창 안의 제목 표시줄 아래에 있습니다. 편집기에서 표시되며 시작 화면에서는 표시되지 않습니다.
* **macOS**에서는 메뉴가 화면 맨 위의 시스템 메뉴 바에 있습니다. 메뉴가 두 개 더 있습니다: 앱 메뉴 **PenCreator**(**PenCreator에 관하여**(About PenCreator), 시스템의 일반 항목, **종료**(Quit))와 **윈도우**(Window)(최소화와 최대화). 그래서 Mac에서는 **종료**가 **파일** 끝에 없고 **정보**(About)가 **도움말** 끝에 없습니다.
* 지금 쓸 수 없는 항목은 회색입니다(예: 그림을 선택하지 않았을 때의 **이미지 트레이스…**(Trace image…), 시작 화면의 **저장**(Save)). 스위치는 켜져 있으면 체크 표시가 있습니다.
* 단축키는 항목 옆에 표시되며, 모두 [키보드 단축키](Keyboard-Shortcuts-KO) 페이지에 있습니다. 모든 시스템에서 작동하며, macOS의 시스템 메뉴 바에는 **새로 만들기**(New), **열기…**(Open…), **저장**(Save), **다른 이름으로 저장…**(Save as…), **종료**(Quit)의 단축키만 표시됩니다.

## 파일 (File)

| 항목 | 단축키 | 기능 |
|---|---|---|
| **새로 만들기**(New) | Ctrl+N | 장치의 용지 위에 새 빈 도면을 자체 탭에 만듭니다(동시에 최대 10개 도면). |
| **열기…**(Open…) | Ctrl+O | 프로젝트(`.cre`), SVG, PDF 또는 Illustrator(`.ai`) 파일을 자체 탭에 엽니다. DXF나 UGS 디자인(`.ugsd`)은 도면으로 가져옵니다. 이미 열린 프로젝트는 두 번 열지 않고 표시만 합니다. [빠른 시작](Quick-Start-KO)과 [프로젝트 파일](Project-File-KO)을 참조하십시오. |
| **최근 파일 열기**(Open recent) | | 마지막 파일 10개. **목록 지우기**(Clear list)는 목록을 비웁니다. |
| **저장**(Save) | Ctrl+S | 프로젝트를 `.cre`로 씁니다. 아직 프로젝트 파일이 없는 도면(새 도면, 또는 SVG나 PDF에서 연 도면)은 먼저 이름을 묻습니다. |
| **다른 이름으로 저장…**(Save as…) | Ctrl+Shift+S | 프로젝트를 새 이름으로 씁니다. |
| **프로젝트 정보…**(Project information…) | | 파일(이름, 폴더, 크기, 만든 시각과 바꾼 시각), 프로젝트를 만든 시각, 만든 사람, 기여자, 편집 기록. 직접 저장할 때마다 **설정 → 프로그램 옵션 → 작성자**(Settings → Program options → Author)에 설정된 사람(이름, 이메일, 기타. 모든 프로그램에서 같음), 시각, 프로그램과 그 버전을 기록하며, 최대 1분에 한 번입니다. 만든 사람은 세 가지를 모두, 기여자는 이름과 이메일만(이메일이 만든 사람과 같으면 이름만) 저장합니다. |
| **가져오기 → 이미지 가져오기…**(Import → Import image…) | | 가져오기 대화 상자를 거쳐 그림(PNG, JPG, BMP, GIF, WebP)을 가져옵니다. [디자인 도구](Design-Tools-KO)를 참조하십시오. |
| **가져오기 → SVG 가져오기…**(Import → Import SVG…) | Ctrl+I | SVG의 패스를 열린 도면에 추가합니다. 각 패스는 색이 가장 가까운 레이어에, 그림은 활성 레이어에 놓입니다. **열기…**는 그 파일로 새 탭을 만듭니다. |
| **가져오기 → Adobe Illustrator / PDF / EPS / VectorCraft 가져오기…**(Import → Import Adobe Illustrator / PDF / EPS / VectorCraft…) | | PDF, PDF 호환 `.ai`, 옛 PostScript 형식의 `.ai`, EPS 또는 VectorCraft 도면(`.vectorcraft`, 그림 포함)에 대해 같은 일을 합니다. Illustrator 레이어가 있는 파일은 색 기준으로 할지 그 레이어 기준으로 할지 묻습니다. |
| **가져오기 → DXF 가져오기…**(Import → Import DXF…) | | DXF 도면의 선, 호, 곡선을 도면에 추가합니다. |
| **가져오기 → LightBurn / MillMage 프로젝트 가져오기…**(Import → Import LightBurn / MillMage project…) | | LightBurn 프로젝트(`.lbrn2`, `.lbrn`)나 MillMage 프로젝트(`.mage`)의 도형을 도면에 추가합니다. |
| **가져오기 → UGS 디자인 가져오기…**(Import → Import UGS design…) | | UGS Designer 파일(`.ugsd`)의 도형을 도면에 추가합니다. |
| **가져오기 → 플러그인으로 가져오기…**(Import → Import with a plugin…) | | 설치된 플러그인이 읽는 형식의 파일. [플러그인](Plugins-KO)을 참조하십시오. |
| **가져오기 → 도형 생성…**(Import → Generate shape…) | | 설치된 플러그인의 생성기. |
| **내보내기 → G-code 내보내기…**(Export → Export G-code…) | | **플로터 → G-code 내보내기…**(Plotter → Export G-code…)와 같습니다. |
| **내보내기 → SVG 내보내기…**(Export → Export SVG…) | | 도면 전체를 작업 영역 크기의 SVG로. 곡선과 그림은 유지됩니다. |
| **내보내기 → PLT / DXF 내보내기…**(Export → Export PLT / DXF…) | | 도면 전체의 윤곽선을 고른 확장자에 따라 HPGL(`.plt`, 레이어마다 펜 하나) 또는 DXF R12로. 기계의 이동이 아니라 도형입니다. |
| **내보내기 → DXF 내보내기…**(Export → Export DXF…) | | 도면 전체를 현재 형식의 DXF(AutoCAD 2007)로. 곡선은 0.01 mm로 평탄화됩니다. |
| **내보내기 → DXF R12 내보내기 (legacy)…**(Export → Export DXF R12 (legacy)…) | | 오래된 프로그램과 커터를 위해 같은 내용을 DXF R12로(*legacy*라고도 합니다). |
| **내보내기 → Adobe Illustrator 내보내기…**(Export → Export Adobe Illustrator…) | | 윤곽선을 PostScript 형식의 Illustrator 파일로, 곡선을 포함하고 프로젝트 레이어마다 Illustrator 레이어 하나를 만듭니다. 그림은 빠집니다. |
| **내보내기 → LightBurn 프로젝트 내보내기…**(Export → Export LightBurn project…) | | LightBurn 프로젝트(`.lbrn2`, 또는 파일 이름을 그렇게 지으면 `.lbrn`): 곡선을 포함한 도형과 그림, 프로젝트 레이어마다 LightBurn 레이어 하나. |
| **내보내기 → MillMage 프로젝트 내보내기…**(Export → Export MillMage project…) | | 같은 내용을 MillMage 프로젝트(`.mage`)로, 재료는 작업 영역 크기. |
| **내보내기 → 선택 영역을 SVG로 내보내기…**(Export → Export selection as SVG…) / **선택 영역을 DXF로 내보내기…**(Export selection as DXF…) | | 선택한 도형(아무것도 선택하지 않았으면 모두)을 SVG나 DXF로 씁니다. |
| **G-code 파일 보기…**(View G-code file…) | | G-code 파일(`.gcode`, `.nc`, `.gc`, `.gco`, `.ngc`, `.tap`, `.cnc`)을 **G-code 분석**(G-code analysis) 탭에서 엽니다: 도면을 바꾸지 않고 공구 경로를 보여 줍니다. |
| **인쇄…**(Print…) | Ctrl+P | 디자인을 종이 프린터로 인쇄하거나 PDF로 저장합니다. |
| **종료**(Quit) | Ctrl+Q | 프로그램을 닫습니다. 저장하지 않은 작업이 있는 도면마다 먼저 묻습니다. Windows와 Linux에서만 있습니다(macOS에서는 앱 메뉴에 있습니다). |

## 편집 (Edit)

| 항목 | 단축키 | 기능 |
|---|---|---|
| **실행 취소**(Undo) | Ctrl+Z | 도면의 마지막 변경을 되돌립니다(최대 100단계). |
| **다시 실행**(Redo) | Ctrl+Y | 다시 합니다(Ctrl+Shift+Z도 됩니다). |
| **복사**(Copy) | Ctrl+C | 선택한 도형을 복사합니다. |
| **붙여넣기**(Paste) | Ctrl+V | 복사한 도형을 붙여 넣습니다. |
| **복제**(Duplicate) | Ctrl+D | 선택한 도형을 복사해 바로 붙여 넣습니다. |
| **삭제**(Delete) | Del | 선택한 도형을 지웁니다(**노드 편집 (N)**(Node edit (N)) 도구에서는 선택한 노드). |
| **모두 선택**(Select all) | Ctrl+A | 표시된 레이어의 모든 도형을 선택합니다. |

## 삽입 (Insert)

새 객체를 페이지에 놓는 왼쪽 팔레트의 도구: **사각형 (R)**(Rectangle (R)), **타원 (E)**(Ellipse (E)), **삼각형 (Y)**(Triangle (Y)), **별 (S)**(Star (S)), **다각형 (G)**(Polygon (G)), **하트 (K)**(Heart (K)), **나선 (I)**(Spiral (I)), **선 (L)**(Line (L)), **폴리라인 / 펜 (P)**(Polyline / pen (P)), **텍스트 (T)**(Text (T)). 사용 중인 도구에 체크 표시가 있습니다. 그 다음:

* **도형 생성…**(Generate shape…): 설치된 플러그인의 생성기(**파일 → 가져오기**와 같음).
* **온라인 클립아트…**(Online clipart…): **클립아트**(Clipart) 탭과 인터넷의 무료 그림 검색을 엽니다.

새 도형은 활성 레이어에 놓입니다([레이어와 펜](Layers-and-Pens-KO) 참조).

## 정렬 (Arrange)

대부분의 항목은 선택한 도형에 작용합니다. 같은 명령이 도형의 오른쪽 클릭 메뉴와 작업 영역 오른쪽의 배치 바에도 있습니다. [디자인 도구](Design-Tools-KO)에 설명되어 있습니다.

| 항목 | 단축키 | 기능 |
|---|---|---|
| **왼쪽 맞춤**(Align left), **가운데 맞춤 (가로)**(Align centre (H)), **오른쪽 맞춤**(Align right), **위쪽 맞춤**(Align top), **가운데 맞춤 (세로)**(Align centre (V)), **아래쪽 맞춤**(Align bottom) | | 선택한 도형을 줄 맞춥니다. |
| **작업대 가운데에 놓기**(Centre on bed) | | 선택 항목을 작업 영역 가운데에 놓습니다. |
| **서로의 가운데에 놓기**(Centre on each other) | | 선택한 도형(또는 그룹)을 선택 항목의 가운데로 옮겨 서로 겹치게 합니다(두 개 이상). |
| **좌우 뒤집기**(Flip horizontal), **상하 뒤집기**(Flip vertical) | | 선택 항목을 대칭 반전합니다. |
| **시계 방향 90° 회전**(Rotate 90° CW), **반시계 방향 90° 회전**(Rotate 90° CCW) | | 선택 항목을 4분의 1 바퀴 돌립니다. |
| **그룹**(Group) | Ctrl+G | 선택한 도형을 한 그룹으로 만듭니다. |
| **그룹 해제**(Ungroup) | Ctrl+Shift+G | 그룹을 풉니다. |
| **잠금**(Lock) | Ctrl+L | 잠긴 도형은 옮기거나 바꿀 수 없습니다. |
| **잠금 해제**(Unlock) | Ctrl+Shift+L | 다시 풉니다. |
| **맨 앞으로 가져오기**(Bring to front), **맨 뒤로 보내기**(Send to back) | | 도면에서 도형의 순서를 바꿉니다. |
| **노드 편집**(Edit nodes) | N | 선택한 도형에 **노드 편집 (N)** 도구를 씁니다. |
| **패스로 변환**(Convert to path) | | 도형(사각형, 별…)을 패스로 만들어 노드를 편집할 수 있게 합니다. |
| **합치기**(Union), **교차**(Intersection), **빼기**(Subtract), **배타적 합**(Exclusive or) | | 닫힌 도형을 합치고, 자르고, 결합합니다. |
| **도형 오프셋…**(Offset shape…), **모서리 둥글게…**(Round corners…), **곡선으로 변환**(Convert to curves) | | 더 크거나 작은 윤곽선, 둥근 모서리, 선 대신 곡선. |
| **패스 자동 연결…**(Auto-join paths…), **허용 오차로 패스 닫기…**(Close path with tolerance…) | | 끝점이 서로 가까운 열린 패스를 연결하고, 끝점이 가까운 패스를 닫습니다. |
| **도그본 / T본 모서리…**(Dogbone / T-bone corners…) | | 날카로운 모서리에 홈(밀링용. 세 프로그램 모두 같음). |
| **격자 배열…**(Grid array…), **원형 배열…**(Circular array…), **패스를 따라 복사…**(Copy along path…), **패스 위 텍스트…**(Text on path…) | | 격자로, 중심 둘레로, 패스를 따라 복사합니다. 패스를 따라 휘어진 텍스트. |
| **도형으로 자르기…**(Trim by shape…), **중복 제거**(Remove duplicates), **분리**(Break apart), **단순화…**(Simplify…) | | 도형으로 선을 자르고, 중복을 없애고, 도형을 윤곽선별로 나누고, 불필요한 점을 없앱니다. |
| **일련번호 텍스트…**(Serial text…), **바코드 / QR 코드…**(Barcode / QR code…), **도형 배치 최적화…**(Nest shapes…), **측정…**(Measure…) | | 번호가 붙은 텍스트, 코드, 영역에 도형 배치, 측정. |
| **이미지 조정…**(Adjust image…), **이미지 트레이스…**(Trace image…) | | 선택한 그림을 바꾸거나 윤곽선으로 바꿉니다. 그림을 선택하지 않으면 회색입니다. |
| **이미지 편집…**(Edit image…) | | 선택한 그림의 사진 편집기: 브러시, 지우개, 복제 도장, 채우기, 회색조로. |
| **이미지로 변환…**(Convert to image…) | | 선택한 도형(그림 제외)을 해상도와 크기를 정해 그림 하나로. |
| **스타일 트레이스…**(Style trace…) | | 선택한 그림을 21가지 스타일 중 하나의 선으로 다시 그립니다. [스타일 트레이스](Style-Trace-KO)를 참조하십시오. |
| **기하학적 다각형…**(Geometric polygons…) | | 선택한 그림을 다각형 사이와 윤곽선 둘레에 틈이 있는 기하학적(로우 폴리) 다각형으로: 그 스타일로 연 스타일 트레이스 창. [스타일 트레이스](Style-Trace-KO)를 참조하십시오. |
| **진입 금지 구역으로 추가**(Add as no-go zone) | | 잠기지 않은 선택 도형 각각이 장치의 진입 금지 구역이 되고(둥글면 원, 그렇지 않으면 둘러싼 사각형) 도면에서 빠집니다. 오른쪽 클릭 메뉴에도 있습니다. |

## 보기 (View)

| 항목 | 단축키 | 기능 |
|---|---|---|
| **격자**(Grid), **격자에 스냅**(Snap to grid), **객체에 스냅**(Snap to objects) | | 스위치(켜져 있으면 체크): 작업 영역의 격자, 격자에 스냅, 다른 도형에 스냅. |
| **확대**(Zoom in) | Ctrl++ | 보기의 가운데를 중심으로 확대합니다. |
| **축소**(Zoom out) | Ctrl+- | 축소합니다. |
| **작업대를 창에 맞추기**(Fit bed to window) | Ctrl+Shift+0 | 작업 영역 전체를 보여 줍니다. |
| **모든 객체 맞추기**(Fit all objects) | Ctrl+Shift+9 | 모든 도형을 보여 줍니다. |
| **작업 영역을 창에 자동 맞춤**(Auto-fit work area to window) | | 스위치: 창 크기가 바뀌어도 작업 영역이 창에 맞춰진 상태로 유지됩니다. 직접 확대/축소하거나 이동하면 꺼집니다. |
| **분할 보기: 나란히**(Split view: side by side), **분할 보기: 위아래로**(Split view: one above the other) | | 작업 영역을 창으로 나눕니다(최대 세 개). 창이 하나일 때 그 마지막 탭이 새 창으로 갑니다(창에 탭이 두 개 이상 있을 때). [디자인 도구](Design-Tools-KO#탭과-창)를 참조하십시오. |
| **단일 보기**(Single view) | | 모든 탭을 다시 한 창에 모읍니다. |
| **카메라 오버레이 표시**(Show camera overlay) | | 도면 아래의 그림을 보이거나 숨깁니다. |
| **진입 금지 구역 표시**(Show no-go zones) | | 장치의 진입 금지 구역을 빨간색으로 그립니다. 장치에 구역이 없으면 회색입니다. |
| **미리 보기**(Preview) | | **미리 보기…**(Preview…) 버튼처럼 미리 보기를 보이거나 숨깁니다. |
| **G-code 미리 보기**(Preview G-code) | | **G-code 미리 보기** 버튼처럼 작업의 G-code를 탭에 표시합니다. |
| **오버레이 가져오기…**(Overlay from…) | | 사진이나 카메라의 실시간 영상을 도면 아래에 놓고, 네 모서리나 기준점으로 맞춥니다. 프로젝트와 함께 저장됩니다. |

## 기계 (Machine)

| 항목 | 기능 |
|---|---|
| **장치 바꾸기…**(Switch device…) | 다른 장치를 고르도록 시작 화면으로 돌아갑니다. |
| **장치 설정…**(Device settings…) | 프로젝트 장치의 설정(페이지: 일반(General), 기계(Machine), 펜(Pen), G-code, 연결(Connection), 카메라(Camera), 조그(Jogging), 매크로(Macros), 유지 보수(Maintenance), 진입 금지 구역(No-go zones), 메모(Notes)). [빠른 시작](Quick-Start-KO)과 [기계 제어](Machine-Control-KO)를 참조하십시오. |
| **펌웨어 설정 ($$)…**(Firmware settings ($$)…) | 복사본과 백업이 있는 GRBL 컨트롤러 설정. |
| **보정…**(Calibration…) | 밀리미터당 스텝 수와 축의 직각도. |
| **진입 금지 구역…**(No-go zones…) | | **진입 금지 구역**(No-go zones) 페이지로 연 장치 창. |
| **제어 장치 ▸**(Control devices ▸) | 제어 장치 설정…(Control device settings…)은 키보드와 패드, 그 키의 창을 엽니다. 이 기계의 제어 장치…(Control device of this machine…)는 열린 기계의 제어 장치를 고릅니다. [제어 장치](Control-Devices-KO)를 참조하십시오. |
| **진단 및 통계…**(Diagnostics and statistics…) | 컨트롤러의 입력, 기계의 통계와 유지 보수, 저장할 수 있는 보고서. |
| **네트워크로 공유 (ser2net)…**(Share over network (ser2net)…) | Linux 전용: 기계의 시리얼 포트를 네트워크로 공유합니다. |
| **프레임**(Frame) | 작업의 경계 상자를 한 바퀴 돕니다(연결된 기계 필요). |
| **작업 시작**(Start job) | 작업을 보냅니다(연결된 기계 필요). |
| **줄에서 시작…**(Start from line…) | 프로그램의 특정 줄에서 작업을 시작하는 창을 엽니다(그릴 것이 있을 때). 시작하려면 연결된 기계가 필요합니다. |
| **프로그램 검사**(Check the program) | 컨트롤러가 검사 모드(`$C`)로 프로그램을 훑습니다. 아무것도 움직이지 않습니다. 연결된 기계가 필요합니다. |

모두 [기계 제어](Machine-Control-KO)에 설명되어 있습니다.

## 플로터 (Plotter)

| 항목 | 단축키 | 기능 |
|---|---|---|
| **펜 라이브러리…**(Pen library…) | | 기본 제공 펜과 내 펜. [레이어와 펜](Layers-and-Pens-KO#펜-라이브러리)을 참조하십시오. |
| **나이프 라이브러리…**(Knife library…) | | 기본 제공 드래그 나이프와 내 나이프. [디자인 도구](Design-Tools-KO#드래그-나이프)를 참조하십시오. |
| **레이어 프리셋…**(Layer presets…) | | 레이어 프리셋 창: 활성 레이어를 이름을 붙여 저장하거나 프리셋을 적용합니다. |
| **G-code 미리 보기**(Preview G-code) | | 작업의 G-code를 저장하지 않고 탭에 텍스트로 표시합니다(**G-code 미리 보기** 버튼과 같음). |
| **작업 플롯…**(Plot of the job…) | Ctrl+Shift+P | **미리 보기**(Preview) 탭을 엽니다: 펜이 그릴 것, 펜을 든 이동, 시간, 애니메이션. [빠른 시작](Quick-Start-KO)을 참조하십시오. |
| **G-code 내보내기…**(Export G-code…) | Ctrl+E | 저장할 위치를 묻고 프로젝트의 G-code를 씁니다(**G-code 저장**(Save G-code) 버튼도 같은 일을 합니다). |

## 설정 (Settings)

| 항목 | 기능 |
|---|---|
| **보기 옵션…**(View options…) | 격자, 작업 영역의 색, 선 굵기, **레이어 이름 표시**(Show layer names), **저장 후 G-code 미리 보기**(Preview G-code after saving), **변수 이름 표시**(Show variable names), **G-code 콘솔**(G-code console)의 글꼴 크기와 색. |
| **프로그램 옵션…**(Program options…) | **프로그램 시작 시 새 버전 확인**(Look for a newer version when the program starts)(기본값 켜짐). **자동 저장**(Auto-save): 작업을 자동으로 저장(이름이 있는 파일은 그 자리에, 이름이 없는 파일은 복구 파일로)하며 그 간격(10초에서 1시간, 처음에는 2분). |
| **플러그인…**(Plugins…) | 플러그인을 설치, 전환, 제거합니다. [플러그인](Plugins-KO)을 참조하십시오. |
| **언어**(Language) | 38개 언어. 선택은 유지됩니다. |
| **색 구성표**(Colour scheme) | 18가지 색 구성표(일부 명절 무렵에는 축제 구성표도). |

## 도움말 (Help)

| 항목 | 기능 |
|---|---|
| **GitHub 프로젝트 페이지**(Project page on GitHub) | 브라우저에서 PenCreator 페이지를 엽니다. |
| **백업 다운로드…**(Download backup…) | 설정과 공유 클립아트 갤러리를 zip으로 저장합니다. [설정과 파일](Settings-and-Files-KO)을 참조하십시오. |
| **백업에서 복원…**(Restore from backup…) | 확인 질문 뒤 그런 zip을 다시 읽어 들입니다. |
| **빠른 안내서**(Quick guide)(F1) | 검색 기능이 있는 프로그램 내 안내서. |
| **진단 정보 내보내기 (zip)…**(Export diagnostics (zip)…) | 버그 보고에 첨부할 보고서, 설정, 콘솔이 담긴 파일 하나. |
| **업데이트 확인…**(Check for updates…) | 더 새로운 릴리스를 찾습니다. |
| **정보**(About) | 버전, 라이선스, 크레딧. Windows와 Linux에서만 있습니다(macOS에서는 앱 메뉴의 **PenCreator에 관하여**(About PenCreator)). |
