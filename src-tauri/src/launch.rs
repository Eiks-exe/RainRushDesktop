use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::ShellExt;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct LauncherState {
    pub steam_path: String,
    pub r2_path: String,
    pub bepinex_path: String,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct SteamPathChanged {
    located: bool,
    steam_path: String,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct R2PathChanged {
    located: bool,
    r2_path: String,
}

fn absolute_path(path: &str) -> String {
    std::path::Path::new(path).canonicalize().unwrap().to_str().unwrap().to_string()
}



////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tauri::command]
pub fn set_steam_path(app: AppHandle, _state: State<'_, Mutex<LauncherState>>) {
    let app_handle = app.clone();
    app.dialog()
        .file()
        .set_title("steam folder")
        .set_directory("C:/")
        .pick_folder(move |folder_path| {
            let binding = app_handle.state::<Mutex<LauncherState>>();
            let mut state = binding.lock().unwrap();
            let folder_path = folder_path.unwrap();
            println!(
                "checking path: {}",
                std::path::Path::new(&folder_path.to_string())
                    .join("steam.exe")
                    .display()
            );
            if !std::path::Path::new(&folder_path.to_string())
                .join("steam.exe")
                .exists()
            {
                println!("Invalid steam path");
                app.emit(
                    "steam_path_changed",
                    SteamPathChanged {
                        located: false,
                        steam_path: "not found".to_string(),
                    },
                )
                .unwrap();
                return;
            }

            state.steam_path = folder_path.to_string();
            let data = SteamPathChanged {
                located: true,
                steam_path: state.steam_path.clone(),
            };
            println!("steam path found: {}", data.steam_path);
            app.emit(
                "steam_path_changed",
                SteamPathChanged {
                    located: true,
                    steam_path: state.steam_path.clone(),
                },
            )
            .unwrap();
        });
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tauri::command]
pub fn check_bepinex_path(app: AppHandle) {
    // search automatically for bepinex path without user input
    // if found, set it as the bepinex path
    // if not found, prompt user to set it manually
    let binding = app.state::<Mutex<LauncherState>>();
    let mut state = binding.lock().unwrap();
    let possible_path = "./resources/dependencies/BepInExDep/BepInEx/";

    if std::path::Path::new(&possible_path).exists() {
        state.bepinex_path = possible_path.to_string();
        println!("check_bepinex_path: BepInEx found: {}", possible_path);
    } else {
        println!("check_bepinex_path: BepInEx not found");
        state.bepinex_path = "not found".to_string();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tauri::command]
pub fn launch_r2(app: AppHandle, state: State<'_, Mutex<LauncherState>>) {
    let state = state.lock().unwrap();
    app.emit("launch_ror2", "launch_ror2 as been called")
        .unwrap();

    let steam_path = std::path::Path::new(&state.steam_path);
    if !steam_path.exists() {
        println!("launch_r2: Steam not found, please install it");
    }
    let r2_path = std::path::Path::new(&state.r2_path);
    if !r2_path.exists() {
        println!("launch_r2: Risk of Rain 2 not found, please install it");
    }

    let bepinex_path = std::path::Path::new(&state.bepinex_path);
    if !bepinex_path.exists() {
        println!("launch_r2: BepInEx not found, please install it");
    }
    let shell = app.shell();
    let binding = bepinex_path.join("core/BepInEx.Preloader.dll");
    let bep_preloader = binding.to_str().unwrap();
    let absolute_bep_preloader = absolute_path(bep_preloader);
    if !steam_path.exists() || !bepinex_path.join("core/BepInEx.Preloader.dll").exists() {
        println!("launch_r2: configuration error...");
        println!(
            "launch_r2: Steam Path: {}, R2 Path: {}, BepInEx Path: {}",
            steam_path.display(),
            r2_path.display(),
            bepinex_path.display()
        );
    } else {
        println!(
            "launch_r2: Steam Path: {}, R2 path: {}, BepInEx Path: {}",
            steam_path.display(),
            r2_path.display(),
            bepinex_path.display(),
        );
        println!("launch_r2: Launching Risk of Rain 2... {}", absolute_bep_preloader);
        tauri::async_runtime::block_on(async move {
            shell
                .command(steam_path.join("steam.exe"))
                .args(["-applaunch", "632360" , "--doorstop-enable", "true", "--doorstop-target-assembly" , absolute_bep_preloader.as_str()])
                .output()
                .await
                .unwrap()
        });
    }
    app.emit("launch_ror2", "launch_ror2 has finished").unwrap();
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tauri::command]
pub fn setup_environment(app: tauri::AppHandle) {
    app.emit("setup_environment", "setup_environment as been called")
        .unwrap();
    let bepinex_path = std::path::Path::new("resources/dependencies/BepInExDep/BepInEx");
    if !bepinex_path.exists() {
        println!("BepInEx not found, extracting...");
    }
    app.emit("setup_environment", "setup_environment has finished")
        .unwrap();
}
