import React from "react";
import style from "./home.module.css"
import { useAuth } from "../contexts/AuthContext";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { fetchUserRuns } from "../utils/fetchData";
import { IRun } from "../interfaces/Irun";
import {secondsToHms, formatResult, formatSurvivor} from "../utils/utils";
const Home = () => {
  const { state } = useAuth();
  const { user } = state;
  const [history, setHistory] = React.useState<IRun[]>([]);
  const [loading, setLoading] = React.useState(true);
  useEffect(() => {
    console.log("Home component mounted, user:", user, user?.username);
    const unlisten = listen("steam_path_changed", (event) => {
      console.log("Steam path changed", event.payload);
    });
    unlisten.then((f) => f());
    if (!user) {
      return;
    }
    const fetchHistory = async () => {
      setLoading(true); 
      const data = await fetchUserRuns(user.id);
      if (data && data.runs) {
        setHistory(data.runs);
        console.log("Fetched runs:", data.runs);
      }
      setLoading(false);
    };
    fetchHistory();
  }, []);
  return (
  <div className={style.home_wrapper}>
    <div className={style.home_container}>
      <div className={style.home_header}>
        <div className={style.header_title}>Hey, {user?.name} rain's falling...</div>
        <button className={style.start_button} onClick={()=>{invoke("launch_r2")}}>Start</button>
      </div>
      <div className={style.history_card}>
        {loading ? <div className={style.loading}>Loading...</div> : (
          <ul className={style.history_list}>
            {history.map((run: any, index: number) => (
              <li key={index} className={formatResult(run.result) === "Win" ? style.runWon : style.runLost}>
                <span><img src= {`../assets/difficulty/${run.difficulty}.png`}/></span>
                <span><img src= {`../assets/survivors/${formatSurvivor(run.survivor)}.png`} width="50px"/></span>
                <span>{formatResult(run.result)} </span>
                <span>{secondsToHms(run.duration)} </span>
              </li>
             ))}
          </ul>
        )}
      </div>
    </div>
  </div> 
 )
}

export default Home;


