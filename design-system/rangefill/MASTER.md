# RangeFill 디자인 시스템 (MASTER)

원본 토큰은 `src/style.css`의 `:root`이며, 이 문서는 그 값과 선택 이유를 기록한다.
`ui-ux-pro-max` 검색 결과를 근거로 하되, 스킬의 `--design-system` 자동 결과는 랜딩 페이지용(Trust & Authority, 초록 계열)이라 단일 화면 계산 도구에 맞지 않아 쓰지 않았다. 도메인 검색(`style`, `color`, `typography`, `google-fonts`, `ux`)에서 맞는 항목만 골랐다.

## 제품 성격

- 한 화면짜리 도구. 주유소에서 한 손으로 숫자 하나를 넣고 금액을 읽는다.
- 핵심 정보는 금액이고, 근거는 “남은 거리 / 채울 거리” 게이지다.

## 스타일: Minimalism & Swiss

- 근거: 스타일 검색의 Minimalism & Swiss Style (실용 도구·대시보드, 대비 높고 격자 기반, 접근성 위험 낮음).
- 장식 없이 대비와 크기로 위계를 만든다. 둥근 카드와 그림자를 쓰지 않는다.
- 기억에 남는 요소는 눈금 게이지 하나. 연료 노랑은 “채울 구간”을 뜻하는 곳에만 쓴다.

## 컬러 (HEX)

중립색은 아스팔트와 콘크리트 쪽으로 살짝 기운 따뜻한 회색이다. Tailwind 기본 slate(`#0F172A`, `#F8FAFC` 등)는 어떤 SaaS에나 있는 값이라 쓰지 않는다. 강조색은 오류색과 겹치지 않도록 빨강 대신 연료 노랑을 쓴다.

| 토큰 | 라이트 | 다크 | 용도 |
|---|---|---|---|
| `--color-background` | `#F3F2ED` | `#161614` | 화면 바탕 |
| `--color-foreground` | `#1C1C1A` | `#F0EFEA` | 본문, 금액 |
| `--color-muted-foreground` | `#5B5A55` | `#A5A39A` | 보조 설명, 단위 |
| `--color-border` | `#DAD8D0` | `#2F2E2A` | 설정 행 구분선 |
| `--color-border-strong` | `#7A786F` | `#7C7A70` | 빈 게이지 윤곽(비텍스트 3:1) |
| `--color-primary` | `#1C1C1A` | `#F0EFEA` | 밑줄, 저장 버튼, 설정 상단선, 포커스 밑줄 |
| `--color-on-primary` | `#F7F6F1` | `#161614` | 저장 버튼 글자 |
| `--color-gauge` | `#1C1C1A` | `#6A685F` | 게이지의 “남은 거리” 구간 |
| `--color-accent` | `#F4C21B` | `#F4C21B` | 게이지의 “채울 거리” 구간 |
| `--color-destructive` | `#B91C1C` | `#FCA5A5` | 오류 문구 |
| `--color-ring` | `#1C1C1A` | `#F0EFEA` | 키보드 포커스 |

대비 실측: 본문 15.23:1(다크 15.74:1), 보조 글자 6.17:1(다크 7.17:1), 오류 5.77:1(다크 9.55:1), 빈 게이지 윤곽 3.95:1(다크 4.21:1), 저장 버튼 15.77:1, 라이트 게이지 구간 대 노랑 10.22:1, 다크 게이지 구간 대 노랑 3.35:1·대 바탕 3.24:1.

순백과 순흑은 쓰지 않는다. 쓰지 않는 토큰은 만들지 않는다.

## 타이포그래피

- 제목·숫자: **Barlow Condensed** 600/700. 표지판 숫자 느낌의 좁은 서체라 큰 금액이 한 줄에 들어간다. `google-fonts` 도메인에서 확인(Sans Serif, latin, 인기 29위). `@fontsource`로 앱에 포함해 오프라인에서도 쓴다.
- 본문(한글): **Noto Sans KR**. 서체 검색의 “Korean Modern” 항목. 별도 번들 없이 시스템 서체를 쓴다(iOS는 Apple SD Gothic Neo, Android는 Noto Sans CJK KR). 스택은 `-apple-system, BlinkMacSystemFont, "Apple SD Gothic Neo", "Noto Sans KR", "Segoe UI", sans-serif`.
- 스케일(px): 12 푸터 · 13 보조/오류/눈금 · 14 라벨 · 15 설정 제목·단위 · 17 눈금 숫자 · 22 보조 숫자 · 26 제목 · 28 설정 입력 · 56 거리 입력 · 72~112 금액(`clamp(72px, 25vw, 112px)`).
- 숫자는 `font-variant-numeric: tabular-nums`. 한글 줄바꿈은 기본값.

## 간격·형태·모션

- 간격(4pt): 4 · 8 · 12 · 16 · 24 · 32 · 48 (`--space-1`~`--space-7`). 화면 좌우 여백 24px.
- 글자 굵기: 500 · 600 · 700만 쓴다(시스템 한글 서체에 없는 650·550 금지). 한글은 `word-break: keep-all`.
- 모서리: 컨트롤 6px, 눈금 범례 칩 2px. 그림자 없음.
- 터치 영역: 최소 48px(`--touch`). 입력과 버튼 모두 `min-height: 48px`.
- 모션: 눌림 160ms, 펼침 200ms, 게이지 220ms. 곡선은 `--ease-out: cubic-bezier(.23, 1, .32, 1)` 하나만 쓴다(내장 `ease`·`ease-in` 금지). `transform`과 `opacity`만 움직인다(게이지는 `width`가 아니라 `scaleX`). 버튼 눌림은 `scale(.97)`. 금액 숫자처럼 키 입력마다 바뀌는 값은 애니메이션하지 않는다. `prefers-reduced-motion`에서는 이동과 크기 변화를 끄고 투명도 변화만 남긴다.

## 폼 규칙

- 모든 입력에 보이는 라벨. 숫자 입력은 `inputmode="decimal"`(UX 검색: Mobile Keyboards, Input Labels).
- 오류는 입력 바로 아래, 해결 방법을 담아 한 문장으로.

## 화면 구성(메인)

```
RangeFill
얼마나 넣을까요?
현재 남은 주행가능거리
150 ______________________ km     ← 56px 숫자, 굵은 밑줄
차량 계기판에 표시된 거리를 입력하세요.

51,000 원                         ← 72~112px, 시스템의 중심
필요 주유량 30.00 L
[■■■▮▮▮▮▮▮▮▮▮▮▮▮▮▮▮▮▮▮▮▮]       ← 눈금 게이지(10% 눈금)
■ 남은 거리 150 km     ■ 채울 거리 450 km
───────────────────────────────
계산 기준 설정                  ⌄  ← 목록형, 숫자 우측 정렬
```

정렬은 왼쪽 기준이고 한 열이다.

## 앱 아이콘

- 원본: `src-tauri/app-icon.svg`(1024, 모서리 둥글림 없는 꽉 찬 정사각형. iOS와 Android가 직접 마스크한다). 플랫폼별 파일은 `pnpm tauri icon src-tauri/app-icon.svg`로 만들고, `src-tauri/icons/app-icon.svg.png`는 생성된 iOS 1024px 아이콘(`AppIcon-512@2x.png`)을 복사한다.
- 구성: 아스팔트 `#1C1C1A` 바탕에 앱 화면의 게이지를 그대로 쓴다. 10칸 눈금 막대에서 남은 구간은 `#F0EFEA`, 채울 구간은 연료 노랑 `#F4C21B`이고, 두 구간의 경계(30%)를 가리키는 바늘(역삼각형)을 얹었다.
- 막대 폭은 화면의 약 67%로 두어 Android 적응형 아이콘 안전 영역 안에 들어간다. 글자와 그라데이션, 그림자는 쓰지 않는다.
- Android 적응형 배경색(`ic_launcher_background`)은 `#1C1C1A`.

## 금지

- 노랑을 장식이나 버튼 색으로 쓰지 않는다(연료 = 채울 구간 전용).
- 이모지 아이콘, 둥근 카드 반복, 그라데이션 바탕, 화면별 임의 HEX.
