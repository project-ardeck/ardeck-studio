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

import { listen as _listen, UnlistenFn } from "@tauri-apps/api/event"
import { SerialPortInfo } from "../lib/ardeck";

export const listen = {
    async onPorts(callback: (payload: [string, SerialPortInfo][]) => void): Promise<UnlistenFn> {
        return _listen("on-ports", (e) => {
            console.log("on ports");
            callback(e.payload as [string, SerialPortInfo][])
        });
    }
}

