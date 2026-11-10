#!/bin/bash
# 무료 Apple 계정 서명은 7일 뒤 만료된다. 마지막 설치본 만료가 2일 안쪽으로 다가오면
# 새 프로파일로 다시 빌드해 iPhone에 덮어 설치한다(설정 데이터는 유지된다).
# launchd(scripts/com.manuel71.rangefill.refresh.plist)가 매일 실행한다.
#   RANGEFILL_DEVICE  설치할 iPhone UDID (xcrun devicectl list devices)
set -euo pipefail

: "${RANGEFILL_DEVICE:?RANGEFILL_DEVICE(iPhone UDID)를 지정하세요}"

REPO="$(cd "$(dirname "$0")/.." && pwd)"
STATE_DIR="$HOME/Library/Logs/RangeFill"
STAMP="$STATE_DIR/installed-expiry"
PROFILES="$HOME/Library/Developer/Xcode/UserData/Provisioning Profiles"
APP="$REPO/src-tauri/gen/apple/build/rangefill_iOS.xcarchive/Products/Applications/RangeFill.app"
BUNDLE_ID="com.manuel71.rangefill"
RENEW_BEFORE=$((2 * 24 * 3600))

# launchd는 셸 프로필을 읽지 않으므로 빌드 도구 경로를 직접 잡는다.
export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin"
set +u
export NVM_DIR="$HOME/.nvm"
. "$NVM_DIR/nvm.sh"
set -u

mkdir -p "$STATE_DIR"
log() { echo "[$(date '+%F %T')] $*"; }

# 프로파일 파일의 만료 시각(epoch 초)
profile_expiry() {
  local d
  d=$(security cms -D -i "$1" | plutil -extract ExpirationDate raw -o - -)
  date -j -u -f "%Y-%m-%dT%H:%M:%SZ" "$d" +%s
}

now=$(date +%s)
if [[ -f "$STAMP" ]] && (( $(cat "$STAMP") - now > RENEW_BEFORE )); then
  log "건너뜀: 설치본 만료 $(date -r "$(cat "$STAMP")" '+%F %T')"
  exit 0
fi

# Xcode는 아직 유효한 프로파일을 재사용하므로, 이 앱의 캐시 프로파일을 지워 새로 발급받는다.
shopt -s nullglob
for p in "$PROFILES"/*.mobileprovision; do
  if security cms -D -i "$p" | plutil -extract Entitlements.application-identifier raw -o - - | grep -q "\.$BUNDLE_ID$"; then
    rm "$p"
  fi
done

cd "$REPO"
log "빌드 시작"
pnpm tauri ios build --debug --ci --target aarch64 --export-method debugging --archive-only
xcrun devicectl device install app --device "$RANGEFILL_DEVICE" "$APP"

expiry=$(profile_expiry "$APP/embedded.mobileprovision")
echo "$expiry" > "$STAMP"
log "설치 완료: 만료 $(date -r "$expiry" '+%F %T')"
