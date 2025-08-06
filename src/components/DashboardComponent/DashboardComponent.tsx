import react, { useEffect } from "react";
import { useAuth } from "../../contexts/AuthContext";
import style from "./dashboard.module.css";
import { useAppContext } from "../../contexts/AppContext";


const DashboardComponent: react.FC = () => {
  const { state } = useAuth();
  const { setView } = useAppContext();

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
            <li onClick={() => setView('home')}>Home</li>
            <li onClick={() => setView('settings')}>Settings</li>
            <li onClick={() => setView('profile')}>Profile</li>
          </ul>
        </div>
      </div>
    </div>
  );
}




export default DashboardComponent; 
