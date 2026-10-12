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
- Xcode와 command line tools, CocoaPods, XcodeGen, libimobiledevice
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
pnpm ios:init        # iOS 프로젝트 재생성이 필요한 경우, macOS에서만 실행
pnpm android:init    # Android 프로젝트 재생성이 필요한 경우
pnpm tauri icon src-tauri/app-icon.svg   # init 이후 실행: gen/ 프로젝트의 아이콘까지 채움
```

- 원본 아이콘은 `src-tauri/app-icon.svg`이며, Rust 컴파일에 사용하는 PNG는 `src-tauri/icons/app-icon.svg.png`입니다. 모바일 프로젝트 초기화 후 `tauri icon` 명령으로 플랫폼별 아이콘을 생성하세요.

- 앱 식별자는 `src-tauri/tauri.conf.json`의 `identifier`(`com.manuel71.rangefill`)입니다. 모바일 프로젝트는 이 값으로 생성되므로 바꾸려면 init 전에 바꾸세요.
- 생성된 프로젝트의 최소 지원 OS는 iOS 15와 Android 7(API 24)입니다.
- `gen/apple`, `gen/android`와 플랫폼별 아이콘이 저장소에 포함되어 있습니다.
- iOS 프로젝트는 현재 개발 팀으로 생성되어 있습니다. 다른 팀으로 빌드하려면 Xcode의 Signing & Capabilities에서 팀을 선택하고 `project.yml`의 `DEVELOPMENT_TEAM`도 맞추세요.

## 개발·빌드

```sh
pnpm ios:dev         # iOS 시뮬레이터/기기, HMR
pnpm android:dev     # Android 에뮬레이터/기기, HMR
pnpm ios:build       # iOS 릴리스 빌드
pnpm android:build   # Android 릴리스 빌드(APK/AAB)
```

## iPhone에 개발용 앱 설치

```sh
pnpm tauri ios build --debug --ci --target aarch64 --export-method debugging --archive-only
xcrun devicectl device install app --device "<iPhone UDID>" src-tauri/gen/apple/build/rangefill_iOS.xcarchive/Products/Applications/RangeFill.app
xcrun devicectl device process launch --device "<iPhone UDID>" com.manuel71.rangefill
```

기기에 개발자 모드가 켜져 있어야 합니다. 설치 후 프로필 신뢰 문제로 실행이 거부되면
iPhone의 설정 → 일반 → VPN 및 기기 관리에서 해당 개발자 앱을 신뢰한 뒤 다시 실행하세요.
이 빌드는 화면을 앱에 포함하므로 개발 서버 없이 사용할 수 있습니다.

## 검증

```sh
pnpm typecheck       # TypeScript
pnpm test:rust       # Rust 계산기 단위 테스트
pnpm build           # tsc + vite build
```
