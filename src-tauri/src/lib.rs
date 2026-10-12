mod calc;

use std::sync::Arc;

use calc::{Quote, Settings};
use tauri::{AppHandle, Wry};
use tauri_plugin_store::{Store, StoreExt};

const STORE_FILE: &str = "settings.json";
const STORE_KEY: &str = "settings";

// 자동 저장과 플러그인의 종료 저장을 막고 명시적 save()만 디스크에 쓴다.
fn open_store(app: &AppHandle) -> Result<Arc<Store<Wry>>, String> {
    let store = app
        .store_builder(STORE_FILE)
        .disable_auto_save()
        .build()
        .map_err(|e| e.to_string())?;
    store.close_resource();
    Ok(store)
}

#[tauri::command]
fn calculate(settings: Settings, b: f64) -> Result<Quote, String> {
    calc::quote(settings, b)
}

#[tauri::command]
fn load_settings(app: AppHandle) -> Result<Option<Settings>, String> {
    let store = open_store(&app)?;
    let path =
        tauri_plugin_store::resolve_store_path(&app, STORE_FILE).map_err(|e| e.to_string())?;
    if path.try_exists().map_err(|e| e.to_string())? {
        store.reload().map_err(|e| e.to_string())?;
    }
    let Some(value) = store.get(STORE_KEY) else {
        return Ok(None);
    };
    let settings: Settings = serde_json::from_value(value)
        .map_err(|e| format!("저장된 설정이 올바르지 않습니다: {e}"))?;
    settings.validate().map(Some)
}

#[tauri::command]
fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    let settings = settings.validate()?;
    let store = open_store(&app)?;
    store.set(STORE_KEY, serde_json::json!(settings));
    store.save().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            calculate,
            load_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
