use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;
use std::fs;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;
use reqwest::Client;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct UtilsState {
    pub app_data_dir: String,
    pub dep_dir: String,
    pub steam_path: String,
    pub r2_path: String,
    pub bepinex_path: String,
    pub token: String,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct SteamPathChanged {
    pub located: bool,
    pub steam_path: String,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct R2PathChanged {
    located: bool,
    r2_path: String,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct UserData {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct LoginSuccess {
    pub token: String,
    pub user_id: String,
    pub user_name: String,
    pub user_email: String,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct LoginFailed {
    pub error: String,
}

fn check_app_data_dir(app: AppHandle) -> String {
    let app_data_dir = app.path().app_data_dir().unwrap();
    let data = app_data_dir.to_str().unwrap().to_string();
    data
}

fn check_steam_path() -> String {
    // search automatically for steam path without user input
    // if found, set it as the steam path
    // if not found, prompt user to set it manually
    let possible_paths = vec![
        "C:/Program Files (x86)/Steam/",
        "C:/Program Files/Steam/",
        "D:/Steam/",
        "E:/Steam/",
    ];

    let mut found_path = None;

    for path in possible_paths {
        println!(
            "Checking path: {}",
            std::path::Path::new(path).join("steam.exe").display()
        );
        if std::path::Path::new(path).join("steam.exe").exists() {
            found_path = Some(path.to_string());
            break;
        }
    }

    if let Some(path) = found_path {
        let data = path.to_string();
        data
    } else {
        let data = "not found".to_string();
        data
    }
}

#[tauri::command]
pub fn get_steam_state(app: tauri::AppHandle) -> String {
    let binding = app.state::<Mutex<UtilsState>>();
    let state = binding.lock().unwrap();
    state.steam_path.clone()
}

pub fn check_r2_path(steam_dir_path: &String) -> String {
    // search automatically for r2 path without user input
    // if found, set it as the r2 path
    // if not found, prompt user to set it manually
    let steam_dir = std::path::Path::new(steam_dir_path);
    let possible_path = steam_dir.join("steamapps/common/Risk of Rain 2/");
    if steam_dir.exists() && possible_path.exists() {
        let data = possible_path.clone().to_str().unwrap().to_string();
        data
    } else {
        println!("Risk of Rain 2 not found");
        let data = "not found".to_string();
        data
    }
}

pub fn get_bep_loader(dep_dir: &String) -> String {
    let bepinex_path =
        std::path::Path::new(dep_dir).join("BepInExDep/BepInEx/core/BepInEx.Preloader.dll");
    if bepinex_path.exists() {
        let data = bepinex_path.to_str().unwrap().to_string();
        data
    } else {
        println!("BepInEx not found");
        let data = "not found".to_string();
        data
    }
}
#[derive(Default, Clone, Deserialize, Serialize)]
struct EnvSetup {
    completed: bool,
}

#[tauri::command]
pub fn index_dirs(app: AppHandle, state: State<'_, Mutex<UtilsState>>) {
    println!("Indexing directories");
    let mut state = state.lock().unwrap();
    let app_data_dir = check_app_data_dir(app.clone());
    let mut steam_path = state.steam_path.clone();
    let bepinex_path = get_bep_loader(&app_data_dir);

    if steam_path == "not found" {
        steam_path = check_steam_path();
    }

    let r2_path = check_r2_path(&steam_path);

    state.steam_path = steam_path;
    state.r2_path = r2_path;
    state.bepinex_path = bepinex_path;
    state.app_data_dir = app_data_dir;

    app.emit("env_setup", EnvSetup { completed: true }).unwrap();
}

#[tauri::command]
pub fn setup_env(app: AppHandle) {
    let app_data_dir = app.path().app_data_dir().unwrap();
    let dep_dir = app
        .path()
        .resolve("dependencies", tauri::path::BaseDirectory::Resource)
        .unwrap();

    if !dep_dir.exists() {
        println!("Dependencies not found");
    }

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).unwrap();
    }

    match copy_dir_recursive(&dep_dir, &app_data_dir) {
        Ok(_) => {
            println!("Dependencies copied successfully");
            app.emit("env_setup", EnvSetup { completed: true }).unwrap();
        }
        Err(e) => {
            println!("Error copying dependencies: {}", e);
        }
    }
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    // Create the destination directory if it doesn't exist
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    // Iterate over the entries in the source directory
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        // If it's a directory, recurse, otherwise copy the file
        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn set_steam_path(app: tauri::AppHandle) {
    let app_handle = app.clone();
    app.dialog()
        .file()
        .set_title("steam folder")
        .set_directory("C:/")
        .pick_folder(move |folder_path| {
            let binding = app_handle.state::<Mutex<UtilsState>>();
            let mut state = binding.lock().unwrap();
            let folder_path = folder_path.unwrap();
            let string_folder_path = folder_path.to_string();
            let path = std::path::Path::new(&string_folder_path);
            println!("checking path: {}", path.join("steam.exe").display());
            if !path.join("steam.exe").exists() {
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

            state.steam_path = path.join("steam.exe").to_str().unwrap().to_string();
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
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
    pub user: UserData,
}

#[tauri::command]
pub async fn login(identifier: String , password: String, _app: AppHandle) -> Result<LoginResponse, String> {
    let app_handle = _app.clone(); 
    let binding = app_handle.state::<Mutex<UtilsState>>();
    
    let url = "http://localhost:8080/api/auth/login";
    let client = Client::new();
    
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "identifier": identifier,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if res.status().is_success() {
        let data = res.json::<LoginResponse>()
            .await
            .map_err(|e| e.to_string())?;
        {
            let mut state = binding.lock().unwrap();
            state.token = data.token.clone();
        }
        Ok(data)
    } else {
        Err(format!("Login failed: {}", res.status()))
    }
}
