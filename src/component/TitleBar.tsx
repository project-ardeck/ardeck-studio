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

import { appWindow } from "@tauri-apps/api/window";
import { useState } from "react";

import closeButton from "/titlebar/close_24dp_FILL0_wght200_GRAD0_opsz24.svg";
import minimizeButton from "/titlebar/remove_24dp_FILL0_wght200_GRAD0_opsz24.svg";

export default function TitleBar() {
    const [title, setTitle] = useState("");

    appWindow.title().then((e) => {
        setTitle(e);
    });

    const minHandler = () => {
        appWindow.minimize();
    };
    const maxHandler = () => {
        appWindow.toggleMaximize();
    };
    const closeHandler = () => {
        appWindow.hide();
    };

    return (
        <div
            data-tauri-drag-region
            className="flex h-8 w-full items-center justify-between bg-bg-titlebar pl-4 text-text-primary"
        >
            <div className="pointer-events-none select-none">
                <h1>{title}</h1>
            </div>
            <div className="flex h-full select-none items-center justify-center">
                <div
                    onClick={minHandler}
                    className="flex h-full items-center px-2 transition-colors hover:bg-bg-secondary"
                >
                    <img
                        src={minimizeButton}
                        className="pointer-events-none"
                        alt="minimize"
                    />
                </div>
                {/* <div onClick={maxHandler}>
                    max
                </div> */}
                <div
                    onClick={closeHandler}
                    className="flex h-full items-center px-2 transition-colors hover:bg-accent-negative"
                >
                    <img
                        src={closeButton}
                        className="pointer-events-none text-text-primary"
                        alt="close"
                    />
                </div>
            </div>
        </div>
    );
}
