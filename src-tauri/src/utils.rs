use tauri::{Emitter, Manager, State, AppHandle};
use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct UtilsState {
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

#[tauri::command]
pub fn check_steam_path(app: AppHandle, _state: State<'_, Mutex<UtilsState>>) {
    // search automatically for steam path without user input 
    // if found, set it as the steam path
    // if not found, prompt user to set it manually

    let app_handle = app.clone();
    let binding = app_handle.state::<Mutex<UtilsState>>();
    let mut state = binding.lock().unwrap();

    println!("Checking for steam path");
    let possible_paths = vec![
        "C:/Program Files (x86)/Steam/",
        "C:/Program Files/Steam/",
        "D:/Steam/",
        "E:/Steam/",
    ];

    let mut found_path = None;

    for path in possible_paths {
        println!("Checking path: {}", std::path::Path::new(path).join("steam.exe").display());
        if std::path::Path::new(path).join("steam.exe").exists() {
            found_path = Some(path.to_string());
            break;
        } 
    }

    
    
    if let Some(path) = found_path {
        state.steam_path = path.clone();
        let data = &state.steam_path;
        println!("Steam path found: {}", data);
        app.emit("steam_path_changed", SteamPathChanged {
            located: true,
            steam_path: data.to_string(),
        }).unwrap();
    } else {
        state.steam_path = "not found".to_string();
        let data = &state.steam_path;
        println!("Steam path not found");
        app.emit("steam_path_changed", SteamPathChanged {
            located: false,
            steam_path: data.to_string(),
        }).unwrap();
    }
    
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tauri::command]
pub fn set_steam_path(app: AppHandle, _state: State<'_, Mutex<UtilsState>>) {
    let app_handle = app.clone();
    app.dialog()
        .file()
        .set_title("steam folder")
        .set_directory("C:/")
        .pick_folder(move |folder_path| {
            let binding = app_handle.state::<Mutex<UtilsState>>();
            let mut state = binding.lock().unwrap();
            let folder_path = folder_path.unwrap();
            println!("checking path: {}", std::path::Path::new(&folder_path.to_string()).join("steam.exe").display());
            if !std::path::Path::new(&folder_path.to_string()).join("steam.exe").exists() {
                println!("Invalid steam path");
                app.emit("steam_path_changed", SteamPathChanged {
                    located: false,
                    steam_path: "not found".to_string(),
                }).unwrap();
                return;
            }   

            state.steam_path = folder_path.to_string();
            let data = SteamPathChanged {
                located: true,
                steam_path: state.steam_path.clone(),
            };
            println!("steam path found: {}", data.steam_path);
            app.emit("steam_path_changed", SteamPathChanged  {
                located: true,
                steam_path: state.steam_path.clone(),
            }).unwrap();
        });
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tauri::command]
pub fn check_r2_path(app: AppHandle, state: State<'_, Mutex<UtilsState>>) {
    // search automatically for r2 path without user input 
    // if found, set it as the r2 path
    // if not found, prompt user to set it manually
    let binding = app.state::<Mutex<UtilsState>>();
    let mut state = binding.lock().unwrap();
    let possible_path = state.steam_path.clone() + "/steamapps/common/Risk of Rain 2";

    if std::path::Path::new(&possible_path).exists() {
        state.r2_path = possible_path.clone();
        app.emit("r2_path_changed", R2PathChanged {
            located: true,
            r2_path: possible_path,
        }).unwrap();

    } else {
        println!("Risk of Rain 2 not found");
        state.r2_path = "not found".to_string();
        app.emit("r2_path_changed", R2PathChanged {
            located: false,
            r2_path: "not found".to_string(),
        }).unwrap();
    }


}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tauri::command]
pub fn check_bepinex_path(app: AppHandle, state: State<'_, Mutex<UtilsState>>) {
    // search automatically for bepinex path without user input 
    // if found, set it as the bepinex path
    // if not found, prompt user to set it manually
    let binding = app.state::<Mutex<UtilsState>>();
    let mut state = binding.lock().unwrap();
    let possible_path = "./resources/dependencies/BepInExDep/BepInEx";

    if std::path::Path::new(&possible_path).exists() {
        state.bepinex_path = possible_path.to_string();
        println!("BepInEx found: {}", possible_path);
    } else {
        println!("BepInEx not found");
        state.bepinex_path = "not found".to_string();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tauri::command]
pub fn launch_r2(app:AppHandle, state: State<'_, Mutex<UtilsState>>) {
    let state = state.lock().unwrap();
    app.emit("launch_ror2", "launch_ror2 as been called")
        .unwrap();
    let steam_path = std::path::Path::new(&state.steam_path);
    if !steam_path.exists() {
        println!("Steam not found, please install it");
    }
    let r2_path = std::path::Path::new(&state.r2_path);
    if !r2_path.exists() {
        println!("Risk of Rain 2 not found, please install it");
    }

    let bepinex_path = std::path::Path::new(&state.bepinex_path);
    if !bepinex_path.exists() {
        println!("BepInEx not found, please install it");
    }

    std::process::Command::new("cmd")
        .arg(steam_path)
        .arg("-applaunch")
        .arg("632360")
        .arg("-doorstop-enable")
        .arg("true")
        .arg("-doorstop-target")
        .arg(bepinex_path.join("/core/BepInEx.Preloader.dll"))
        .current_dir(r2_path)
        .spawn()
        .expect("Failed to launch Risk of Rain 2");
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