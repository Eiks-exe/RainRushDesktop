import "./App.css";
import React from "react";
import AppLayout from "./components/AppLayout/AppLayout";
import { useAppContext } from "./contexts/AppContext";
import Home from "./pages/Home";
import Login from "./pages/Login";
import Register from "./pages/Register";
import Settings from "./pages/Settings";
import { useAuth } from "./contexts/AuthContext";
import DashboardComponent from "./components/DashboardComponent/DashboardComponent";

function App() {
  const { view, setView } = useAppContext();
  const { state } = useAuth();
  React.useEffect(() => {
    console .log('isAuthenticated', state.isAuthenticated);
    if (state.isAuthenticated) {
      setView('home');
    } else {
      setView('login');
    }
  }, [state.isAuthenticated, setView]);
  return (
    <AppLayout>
      {
        !state.isAuthenticated ? (
          <>
            {view === 'login' && <Login />}
            {view === 'register' && <Register />}
          </>   
        ) : (
          <>
            {view === 'home' && <Home />}
            {view === 'settings' && <Settings/>}
            {view === 'profile' && <h1>Profile</h1>}
          </>
        )
      }
    </AppLayout>
  );  
}

export default App;
