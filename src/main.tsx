import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import TitleBar from "./component/TitleBar";
import WindowTheme from "./component/WindowTheme";

import "./main.css";



ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(

    <React.StrictMode>
        <WindowTheme>
            <div className="w-full h-full flex flex-col overflow-hidden">
                <header>
                    <TitleBar />
                </header>
                <main className="flex-1 overflow-auto">
                    <App />
                </main>
            </div>
        </WindowTheme>
    </React.StrictMode>
);
