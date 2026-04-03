import React from "react";
import styles from "./home.module.css";
import { useAuth } from "../contexts/AuthContext";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { fetchUserRuns } from "../utils/fetchData";
import { IRun } from "../interfaces/Irun";
import { secondsToHms, formatResult, formatSurvivor } from "../utils/utils";
import { AppHeader } from "../components/AppHeader";
import HomeLayout from "../components/HomeLayout/HomeLayout";

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
    <>
      {user ? (
        <HomeLayout user={user}>
          <div className={styles.history_container}>
            <h4>History</h4>
            <div className={styles.history_content}>
              {history.map((run: IRun) => (
                <div
                  className={
                    formatResult(run.result) === "Win"
                      ? styles.runWon
                      : styles.runLost
                  }
                >
                  <img
                    src={`../assets/survivors/${formatSurvivor(run.survivor)}.png`}
                    alt={formatSurvivor(run.survivor).toString()}
                  />
                  <div className={styles.items_wrapper}>
                    {run.items &&
                      Object.keys(run.items).map((item) => (
                        <img src={`assets/items/${item}.png`} />
                      ))}
                  </div>
                  <img src={`/assets/difficulty/${run.difficulty}.png`} />
                </div>
              ))}
            </div>
          </div>
        </HomeLayout>
      ) : (
        <span>error</span>
      )}
    </>
  );
};

export default Home;
