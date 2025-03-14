/*
Ardeck studio - The ardeck command mapping software.
Copyright (C) 2024 Project Ardeck

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
import { useEffect, useState } from "react";

import {
    VscChromeClose,
    VscChromeMaximize,
    VscChromeRestore,
    VscChromeMinimize,
} from "react-icons/vsc";

export default function TitleBar() {
    const [title, setTitle] = useState("");

    const [isMaximized, setIsMaximized] = useState(false);

    useEffect(() => {
        appWindow.onResized(() => {
            appWindow.isMaximized().then((e) => setIsMaximized(e));
        });
        appWindow.isMaximized().then((e) => setIsMaximized(e));
    }, []);

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
        <div className="relative flex h-8 w-full items-center justify-between bg-bg-titlebar pl-4 text-text-primary">
            <div className="pointer-events-none select-none">
                <h1>{title}</h1>
            </div>
            <div className="absolute right-0 z-50 flex h-full select-none items-center justify-center">
                <div
                    onClick={minHandler}
                    onKeyDown={(e) => e.key === "Enter" && minHandler()}
                    tabIndex={0}
                    role="button"
                    className="flex h-full items-center px-2 transition-colors hover:bg-bg-secondary"
                >
                    {/* <img
                        src={minimizeButton}
                        className="pointer-events-none"
                        alt="minimize"
                    /> */}
                    <VscChromeMinimize />
                </div>
                <div
                    onClick={maxHandler}
                    onKeyDown={(e) => e.key === "Enter" && maxHandler()}
                    tabIndex={0}
                    role="button"
                    className="flex h-full items-center px-2 transition-colors hover:bg-bg-secondary"
                >
                    {/* <img
                        src={minimizeButton}
                        className="pointer-events-none"
                        alt="maximize"
                    /> */}
                    {isMaximized ? <VscChromeRestore /> : <VscChromeMaximize />}
                </div>
                <div
                    onClick={closeHandler}
                    onKeyDown={(e) => e.key === "Enter" && closeHandler()}
                    tabIndex={0}
                    role="button"
                    className="flex h-full items-center px-2 transition-colors hover:bg-accent-negative"
                >
                    {/* <img
                        src={VscChromeClose}
                        className="pointer-events-none text-text-primary"
                        alt="close"
                    /> */}
                    <VscChromeClose />
                </div>
            </div>
            <div
                data-tauri-drag-region
                className={`absolute ${isMaximized ? "left-0 right-0 top-0" : "left-1 right-1 top-1"} bottom-0 z-40`}
            ></div>
        </div>
    );
}
