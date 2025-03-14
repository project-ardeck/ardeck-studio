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

import { Link, NavLink, Outlet } from "react-router";
import { invoke } from "./tauri/invoke";
import AppNavLink from "./pages/_component/AppNavLink";
import { useEffect, useRef, useState } from "react";
import { set } from "lodash";

export default function App() {
    // メニューバーの当たり判定の幅
    const menuBarWidth = useRef(0);
    // メニューの幅を変更する状態かどうか
    const [isMenuBarClick, setIsMenuBarClick] = useState(false);
    // メニューの幅
    const [menuWidth, setMenuWidth] = useState({
        default: 200, // 初期値
        current: 0, // 現在の値
        min: 128, // 最小値
        max: 400, // 最大値
    });

    // window.onmousemove = (e) => {
    //     // e.preventDefault();
    //     if (isMenuBarClick) {
    //         setMenuWidth((p) => {
    //             const width = e.clientX - menuBarWidth.current / 2;

    //             if (width < p.min) return { ...p, current: p.min };
    //             if (width > p.max) return { ...p, current: p.max };
    //             return {
    //                 ...p,
    //                 current: width,
    //             };
    //         });
    //     }
    //     // console.log(isMenuBarClick);
    // };

    // window.onmousedown = (e) => {
    //     const menuBar = document.getElementById("menu-bar");

    //     if (!menuBar) return;

    //     if (e.target !== menuBar) return;

    //     setIsMenuBarClick(true);
    // };

    // window.onmouseup = () => {
    //     setIsMenuBarClick(false);
    // };

    useEffect(() => {
        // メニューバーのつかめる範囲の幅を取得して保存
        const menuBar = document.getElementById("menu-bar");
        menuBarWidth.current = menuBar?.offsetWidth || 0;

        // メニューの幅をdefautで指定された幅に設定する
        setMenuWidth((p) => ({ ...p, current: p.default }));
    }, []);

    const openAbout = () => {
        invoke.openWindow.about();
    };
    return (
        <div className="flex h-full w-full select-none">
            <nav
                // data-tauri-drag-region
                className={`flex flex-col gap-1.5 px-4 py-2`}
                style={{ width: menuWidth.current }}
            >
                <AppNavLink to="config">Config</AppNavLink>
                <AppNavLink to="devices">Devices</AppNavLink>
                <AppNavLink to="mapping">Mapping</AppNavLink>
                <AppNavLink to="plugin">Plugin</AppNavLink>
                <div className="flex-1"></div>
                <button
                    className="hover:bg-bg-secondary rounded-md"
                    onClick={openAbout}
                >
                    About Ardeck
                </button>
            </nav>
            {/* <span
                id="menu-bar"
                className="flex h-full w-2 cursor-e-resize justify-center"
            >
                <span className="pointer-events-none w-1 bg-bg-secondary"></span>
            </span> */}
            <div className="h-full w-full overflow-auto px-4 py-2">
                <Outlet />
            </div>
        </div>
    );
}
