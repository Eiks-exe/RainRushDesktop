import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { Window } from '@tauri-apps/api/window'


document.getElementById("titlebar-minimize")?.addEventListener("click", () => {
  Window.getCurrent().minimize();
});

document.getElementById("titlebar-maximize")?.addEventListener("click", () => {
  Window.getCurrent().maximize();
});

document.getElementById("titlebar-close")?.addEventListener("click", () => {
  Window.getCurrent().close();
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
