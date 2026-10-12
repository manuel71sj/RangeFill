# RangeFill

현재 남은 주행가능거리를 입력하면 가득 채우는 데 필요한 주유량(L)과 금액(원)을 계산하는 Tauri 2 모바일 앱(iOS/Android)입니다.

- 계산: `주유량 = max(0, a − b) ÷ c`, `금액 = 주유량 × d` (a 최대 거리, b 남은 거리, c 연비, d 주유단가). 주유량은 소수 둘째 자리, 금액은 원 단위로 반올림합니다. 계산은 Rust(`src-tauri/src/calc.rs`)가 합니다.
- 설정(a, c, d)은 “설정 저장”을 눌렀을 때만 `tauri-plugin-store`(`settings.json`, 앱 데이터 폴더)에 저장되고 다음 실행 때 복원됩니다. 저장·불러오기에 실패하면 오류를 표시하고 입력값은 유지합니다. 첫 실행에는 기본값이 없어 설정 영역이 열려 있고, 설정을 저장하고 남은 거리를 입력해야 결과가 나옵니다.
- 프런트엔드는 Vite + strict TypeScript(프레임워크 없음)입니다.

## 필요한 도구

공통
- Node.js (Vite가 요구하는 버전, 20.19 이상 권장)와 `pnpm`
- Rust(rustup) stable

iOS (macOS 전용)
- Xcode와 command line tools, CocoaPods
- `rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios`
- 실기기 실행·배포에는 Apple 개발자 팀 ID가 필요합니다(`APPLE_DEVELOPMENT_TEAM` 환경변수 또는 `tauri.conf.json`의 `bundle > iOS > developmentTeam`).

Android
- Android Studio에서 SDK, Platform-Tools, NDK(Side by side), Build-Tools, Command-line Tools 설치
- 환경변수 `JAVA_HOME`(Android Studio 내장 JBR), `ANDROID_HOME`, `NDK_HOME`
- `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`

공식 안내: <https://v2.tauri.app/start/prerequisites/>

## 처음 한 번

```sh
pnpm install
pnpm ios:init        # macOS에서만. src-tauri/gen/apple 생성
pnpm android:init    # src-tauri/gen/android 생성
pnpm tauri icon src-tauri/app-icon.svg   # init 이후 실행: gen/ 프로젝트의 아이콘까지 채움
```

- 원본 아이콘은 `src-tauri/app-icon.svg`이며, Rust 컴파일에 사용하는 PNG는 `src-tauri/icons/app-icon.svg.png`입니다. 모바일 프로젝트 초기화 후 `tauri icon` 명령으로 플랫폼별 아이콘을 생성하세요.

- 앱 식별자는 `src-tauri/tauri.conf.json`의 `identifier`(`com.manuel71.rangefill`)입니다. 모바일 프로젝트는 이 값으로 생성되므로 바꾸려면 init 전에 바꾸세요.
- 최소 지원 OS는 Tauri 2 기본값을 그대로 씁니다. 바꾸려면 init 후 생성된 네이티브 프로젝트나 Tauri 설정에서 지정하세요.
- `gen/apple`, `gen/android`는 생성 후 커밋하는 것이 Tauri 권장입니다.
- `tauri icon`이 SVG 입력을 받지 않는 버전이면 1024×1024 PNG를 만들어 넘기세요.

## 개발·빌드

```sh
pnpm ios:dev         # iOS 시뮬레이터/기기, HMR
pnpm android:dev     # Android 에뮬레이터/기기, HMR
pnpm ios:build       # iOS 릴리스 빌드
pnpm android:build   # Android 릴리스 빌드(APK/AAB)
```

## 검증

```sh
pnpm typecheck       # TypeScript
pnpm test:rust       # Rust 계산기 단위 테스트
pnpm build           # tsc + vite build
```
