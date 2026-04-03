import { useEffect } from "react"
import { AppHeader } from "../../components/AppHeader"
import styles from "./Profile.module.css"
import { useAuth } from "../../contexts/AuthContext"


export const Profile = ()=>{
  const {state} = useAuth()
  const {user} = state;
  return(
    <div className={styles.wrapper}>
     {user.name} 
    </div>
  )
}
