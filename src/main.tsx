/*
Ardeck studio - The ardeck command mapping software.
Copyright (C) 2024 project-ardeck

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3 of the License, or 
(at your option) any later version.

This program is distributed in the hope that it will be useful, 
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the 
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
*/


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
                <main className="flex-1 overflow-auto scrollbar-theme">
                    <App />
                </main>
            </div>
        </WindowTheme>
    </React.StrictMode>
);
