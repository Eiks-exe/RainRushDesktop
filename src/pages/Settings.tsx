import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import React from "react";

import style from "./settings.module.css";

interface SteamPathChangedPayload {
  located: boolean;
  steam_path: string; 
}

const Settings = () => {
const [steamPath, setSteamPath] = React.useState<string>("");
  const handleSetSteamPath = async (e: React.MouseEvent) => {
    e.preventDefault(); 
    await invoke("set_steam_path");
    
  };
  React.useEffect(() => {
    invoke("get_steam_state").then((path: any) => {
      console.log("Steam path", path);
      setSteamPath(path)
    });
    const unlisten = listen<SteamPathChangedPayload>("steam_path_changed", (event: any) => {
      setSteamPath(event.payload.steam_path);
      invoke("index_dirs")
    });
    return () => {
      unlisten.then((f: any) => f());
    };
  }, [steamPath]); 
  return (
    <div className={style.wrapper}>
      <h1>Settings</h1>
      <div className={style.settings_container}>
        <div className={style.settings_item} onClick={(e) => { handleSetSteamPath(e) }}>
          <div className={style.settings_text}>R2_path: {steamPath}</div>
        </div>
        <div className={style.settings_item}>
        </div>
      </div>
    </div>
  );
}

export default Settings;
