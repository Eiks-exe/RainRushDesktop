import style from "./home.module.css";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";


interface SteamPathChangedPayload {
  located: boolean;
  steam_path: string;
}

import React, { useEffect } from "react";

const Home: React.FC = () => {
  const [gamePath, setGamePath] = React.useState<string>("");

  const handleSignOut = async (e: React.MouseEvent) => {
    e.preventDefault();
    invoke("toogle_auth");
  };
  
  const handleSetGamePath = async () => {
    invoke("set_steam_path")
  };

  useEffect(() => {
    invoke("check_steam_path");
    invoke("check_r2p_path")
    invoke("check_bepinex_path")
     
    const unlisten = () => { listen<SteamPathChangedPayload>("steam_path_changed", (event) => {
      setGamePath(event.payload.steam_path);
    })};
    unlisten();
  }, []);

  return (
    <div className={style.container}>
      <h1 className={style.title}>Welcome to the Home Page</h1>
        <div
        onClick={() => {
          handleSetGamePath();
        }}
        className={style.select_path} 
      >
        <div className={style.select_path_title}>
          Risk of Rain 2 executable (click to select path)
        </div>
        <sub>{gamePath}</sub>
      </div>
      <nav className={style.temp_nav}>
        <button
          onClick={() => invoke("launch_r2")}
          className={style.btn}
        >
          launch ror2
        </button>
        <button
          type="submit"
          onClick={(e: React.MouseEvent) => {
            handleSignOut(e);
          }}
          className={style.btn}
        >
          sign out
        </button>
      </nav>
    </div>
  );
};

export default Home;
