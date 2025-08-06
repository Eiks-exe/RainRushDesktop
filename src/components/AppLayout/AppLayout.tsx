import React from 'react';
import '../../App.css'
import { useAuth } from '../../contexts/AuthContext'; 
import style from './appLayout.module.css';
import DashboardComponent from '../DashboardComponent/DashboardComponent';
interface AppLayoutProps {
  children?: React.ReactNode;
}


const Applayout: React.FC<AppLayoutProps> = ({children}) => {
  const { state } = useAuth();
  
  return (
    <div className={state.isAuthenticated ? style.appLayout : style.appLayoutLogin}>
      {state.isAuthenticated && <DashboardComponent />}
      {children}
    </div>
  )
}

const MenuBar: React.FC = () => {
  return (
    <div className={style.menuBar}>
      
    </div>
  )
}

export default Applayout;

