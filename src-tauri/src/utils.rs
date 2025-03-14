use std::fs;
use std::path::Path;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{App, Emitter, Manager, State};

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct UtilsState {
    pub app_data_dir: String,
    pub dep_dir: String,
    pub steam_path: String,
    pub r2_path: String,
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

pub fn check_r2_path(steam_dir_path: &String) -> String {
    // search automatically for r2 path without user input
    // if found, set it as the r2 path
    // if not found, prompt user to set it manually
    let steam_dir = std::path::Path::new(&steam_dir_path);
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

pub fn index_dirs(app: &App, state: &State<'_, Mutex<UtilsState>>) {
    let mut state = state.lock().unwrap();
    
    app.emit("index_dir", ()).unwrap();
    let steam_dir = check_steam_path();
    if steam_dir == "not found"{
        app.emit("steam_path_changed", SteamPathChanged {
            located: false,
            steam_path: steam_dir.clone(),
        }).unwrap();
    }
    
    let r2_dir = check_r2_path(&steam_dir);
    
    if r2_dir == "not found"{
        app.emit("r2_path_changed", R2PathChanged {
            located: false,
            r2_path: r2_dir.clone(),
        }).unwrap();
    }

    app.emit("steam_path_changed", SteamPathChanged {
        located: true,
        steam_path: steam_dir.clone(),
    }).unwrap();
    app.emit("r2_path_changed", R2PathChanged {
        located: true,
        r2_path: r2_dir.clone(),
    }).unwrap();
    
    let app_data_dir = app.path().app_data_dir().unwrap();
    let dep_dir = app
        .path()
        .resolve("dependencies/", tauri::path::BaseDirectory::Resource)
        .unwrap();

    if !dep_dir.exists() {
        println!("Dependencies not found");
    }

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).unwrap();
    }
    state.app_data_dir = app_data_dir.to_str().unwrap().to_string();
    state.dep_dir = dep_dir.to_str().unwrap().to_string();
    state.steam_path = steam_dir;
    state.r2_path = r2_dir;
    
}

#[derive(Default, Clone, Deserialize, Serialize)]
struct EnvSetup {
    completed: bool,
}

pub fn setup_env(app: &App, state: &State<'_, Mutex<UtilsState>>) {
    let mut _state = state.lock().unwrap();
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
    println!("Copying dependencies to app data dir {} {}", dep_dir.display(), app_data_dir.display());
    

    match copy_dir_recursive(&dep_dir, &app_data_dir){
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