use crate::utils;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Mutex};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_shell::ShellExt;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct LaunchState {
    pub steam_path: String,
    pub r2_path: String,
    pub bepinex_path: String,
}

#[tauri::command]
pub fn launch_r2(app: AppHandle, state: State<'_, Mutex<utils::UtilsState>>) {
    let binding = state.lock().unwrap().clone();
    let steam_path = binding.steam_path.as_str();
    let r2_path = binding.r2_path.as_str();
    let bep_loader_path = binding.bepinex_path.as_str();
    let app_data_dir_path = std::path::Path::new(&binding.app_data_dir).join("BepInExDep/BepInEx/config/token.txt");
    std::fs::write(app_data_dir_path, binding.token.as_str()).unwrap();
    println!("steam path launch fn: {}", steam_path);
    println!("risk of rain 2 path launch fn: {}", r2_path);
    let steam_path = std::path::Path::new(steam_path);
    let _r2_path = std::path::Path::new(r2_path);
    let bepinex_path = std::path::Path::new(bep_loader_path);

    println!("steam path: {}", steam_path.display());
    println!("bepinex path: {}", bepinex_path.display());
    let shell = app.shell();
    if !steam_path.exists() {
        app.dialog()
            .message("Steam not found at the specified path.")
            .kind(MessageDialogKind::Error)
            .blocking_show();
        return;
    };
    if !bepinex_path.exists() {
        app.dialog()
            .message("BepInEx not found at the specified path.")
            .kind(MessageDialogKind::Error)
            .blocking_show();
        return;
    }
    tauri::async_runtime::block_on(async move {
        shell
            .command(steam_path)
            .env("RR_TOKEN", binding.token.as_str())
            .args([
                "-applaunch",
                "632360",
                "--doorstop-enable",
                "true",
                "--doorstop-target-assembly",
                bepinex_path.to_str().unwrap(),
            ])
            .output()
            .await
            .unwrap()
    });
}
