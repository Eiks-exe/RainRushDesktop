import react, { useEffect } from "react";
import { useAuth } from "../../contexts/AuthContext";
import style from "./dashboard.module.css";
import { useAppContext } from "../../contexts/AppContext";


const DashboardComponent: react.FC = () => {
  const { state } = useAuth();
  const { view, setView } = useAppContext();
  const views = ["profile", "home", "settings"]
  useEffect(() => {
    console.log('isAuthenticated', state.isAuthenticated, state.user);
  }, [state.isAuthenticated, state.user]);

  return (
    <div className={style.dashboard_wrapper}>
      <div className={style.container}>
        <div className={style.dashboard_header}>
        </div>
        <div className={style.dashboard_nav}>
          <ul>
            {views.map((v)=>(
              <li className={view === v ? style.active : "" } onClick={()=>{setView(v)}}>{v}</li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}




export default DashboardComponent; 
