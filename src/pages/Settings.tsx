import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import React from "react";

import style from "./settings.module.css";
import { AppHeader } from "../components/AppHeader";

interface SteamPathChangedPayload {
  located: boolean;
  steam_path: string; 
}

const Settings = () => {
const [settingsView, setSettingsView] = React.useState<string>("general");

const handleSettingsView = (e: React.MouseEvent, category: string)=>{
    e.preventDefault();
    console.log(settingsView); 
    setSettingsView(category);
    console.log(settingsView);
}
const categories = ["general", "mods"];
  return (
    <div className={style.wrapper}>
      <AppHeader>
      <h1>Settings</h1>
      </AppHeader>
      <div className={style.settings_container}>
        <div className={style.categories}>
        {categories.map((category)=>(
          <div className={settingsView === category ? `${style.category} ${style.fcsCategory}` : style.category} key={category} id={category} onClick={(e)=>{handleSettingsView(e, `${category}`)}}>
            {category}
          </div>
        ))}
          <div className={style.filler}></div>
        </div>
        {settingsView === "general" && <General/>}
        {settingsView === "mods" && <Mods/>}
      </div>
    </div>
  );
}

const  General = ()=> {
  const [steamPath, setSteamPath] = React.useState<string>("not set");
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
  return(
    <>
      <div className={style.settings_item} onClick={(e) => { handleSetSteamPath(e) }}>
        <div className={style.settings_text}>Steam_Path: {steamPath}</div>
      </div>
    </>
  )
}

const Mods = ()=>{
  return(
    <>
      mods
    </>
  )
}

export default Settings;
