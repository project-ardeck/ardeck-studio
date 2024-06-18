import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import TitleBar from "./component/TitleBar";

import "./main.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(

    <React.StrictMode>
        <div className="w-full h-full flex flex-col">
            <header>
                <TitleBar />
            </header>
            <main className="overflow-auto flex-1">
                <App />
            </main>
        </div>
    </React.StrictMode>
);
