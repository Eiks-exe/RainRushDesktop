import React from "react";
import { invoke } from "@tauri-apps/api/core";
import { IUser } from "../../interfaces/IUser";
import styles from "./homelayout.module.css";
interface HomeLayoutProps {
  children: React.ReactNode;
  user: IUser;
}
const HomeLayout: React.FC<HomeLayoutProps> = ({ children, user }) => {
  return (
    <div className={styles.homeWrapper}>
      <div className={styles.homeHeader}>
        <div className={styles.headerTitle}>
          Hey, <span style={{color:"cyan", fontStyle:"italic"}}>{user?.name}</span> rain's falling...
        </div>
        <button
          className={styles.startButton}
          onClick={() => {
            invoke("launch_r2");
          }}
        >
          Start
        </button>
      </div>
      {children}
    </div>
  );
};

export default HomeLayout;
