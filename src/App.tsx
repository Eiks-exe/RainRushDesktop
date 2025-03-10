import Home from "./components/Home";
import Auth from "./components/Authentification";
import React from "react";
import { listen } from '@tauri-apps/api/event';
import "./App.css";

interface AuthStatusChanged {
  authStatus: boolean;
}

function App() {
  const [loggedIn, setLoggedIn] = React.useState<boolean>(false);
  React.useEffect(() => {
    console.log(`logged in: ${loggedIn}`);
    const unlisten = listen<AuthStatusChanged>('auth_status_changed', (event) => {
      setLoggedIn(event.payload.authStatus);
    });

    const downloadEventListener = listen('setup_environment', (event) => {
      console.log(event.payload);
    }
    );

    return () => {
      unlisten.then((f) => f());
      downloadEventListener.then((f
      ) => f());
    }; 
  }, [loggedIn]);
   
  
  return (
    <>
      {loggedIn  ? <Home/> : <Auth.Login/>}
    </>
  );
}

export default App;
