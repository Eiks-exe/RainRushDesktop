mod launch;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Listener, Manager, State};

#[derive(Default, Clone, Deserialize)]
pub struct AppState {
    auth_status: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthStatusChanged {
    auth_status: bool,
}

#[tauri::command]
fn toogle_auth(state: State<'_, Mutex<AppState>>, app: AppHandle) {
    let mut state = state.lock().unwrap();
    state.auth_status = !state.auth_status;
    app.emit(
        "auth_status_changed",
        AuthStatusChanged {
            auth_status: state.auth_status,
        },
    )
    .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let _window = app.get_webview_window("main").unwrap();
            app.manage(Mutex::new(AppState::default()));
            app.manage(Mutex::new(utils::UtilsState {
                app_data_dir: app
                    .path()
                    .app_data_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                steam_path: Default::default(),
                r2_path: Default::default(),
                bepinex_path: Default::default(),
                dep_dir: Default::default(),
                token: Default::default(), 
            }));
            app.listen("auth_status_changed", |event| {
                if let Ok(payload) = serde_json::from_str::<AuthStatusChanged>(event.payload()) {
                    println!("auth status changed: {}", payload.auth_status);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            toogle_auth,
            utils::set_steam_path,
            utils::setup_env,
            utils::index_dirs,
            utils::get_steam_state,
            utils::login,
            launch::launch_r2,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
