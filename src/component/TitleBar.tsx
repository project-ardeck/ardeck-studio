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
        appWindow.minimize()
    }
    // const maxHandler = () => {
    //     appWindow.toggleMaximize();
    // }
    const closeHandler = () => {
        appWindow.hide();
    }

    return (
        <div data-tauri-drag-region className="w-full h-8 flex justify-between items-center pl-4 bg-bg-1 text-white">
            <div className="pointer-events-none select-none">
                <h1>{title}</h1>
            </div>
            <div className="flex select-none h-full justify-center items-center">
                <div onClick={minHandler} className="hover:bg-bg-2 h-full flex items-center px-2 transition-colors">
                    <img src={minimizeButton} className="pointer-events-none" alt="minimize" />
                </div>
                {/* <div onClick={maxHandler}>
                    max
                </div> */}
                <div onClick={closeHandler} className="hover:bg-red h-full flex items-center px-2 transition-colors">
                    <img src={closeButton} className="pointer-events-none" alt="close" />
                </div>
            </div>
        </div>
    )
}