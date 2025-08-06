import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { Window } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { AppContextProvider } from "./contexts/AppContext";
import { AuthProvider } from "./contexts/AuthContext";

const init =  async () => {
  document.getElementById("titlebar-minimize")?.addEventListener("click", () => {
    Window.getCurrent().minimize();
  });
  
  document.getElementById("titlebar-maximize")?.addEventListener("click", () => {
    Window.getCurrent().maximize();
  });
  
  document.getElementById("titlebar-close")?.addEventListener("click", () => {
    Window.getCurrent().close();
  });
  await invoke("setup_env")
};


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <AppContextProvider>
    <AuthProvider>
      <App />
    </AuthProvider>
    </AppContextProvider>
  </React.StrictMode>,
);

init().then(() => {
  console.log("App initialized");
}).catch((err) => {
  console.error("Error initializing app", err);
})
