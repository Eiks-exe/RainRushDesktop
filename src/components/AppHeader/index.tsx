import React from "react"
import styles from "./appHeader.module.css"

interface AppHeaderProps {
  children: React.ReactNode
}


export const AppHeader = ({children}: AppHeaderProps)=>{
  return(
    <div className={styles.AppHeader}>
      {children}
    </div>
  )
}
