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

import { invoke as tauriInvoke } from "@tauri-apps/api";
import { SerialPortInfo } from "../types/ardeck";

// TODO: error handling
export const invoke = {
    settings: {
        async getSetting(settingId: string): Promise<any> {
            return await tauriInvoke("plugin:settings|get_setting", { configId: settingId });
        },
        async getSettingList(): Promise<Array<string>> {
            return await tauriInvoke("plugin:settings|get_setting_list");
        }
    },
    ardeck: {
        async openPort(portName: string, baudRate: number) {
            return await tauriInvoke("plugin:ardeck|open_port", { portName, baudRate });
        },
        async closePort(portName: string) {
            return await tauriInvoke("plugin:ardeck|close_port", { portName });
        },
        async getConnectingSerials(): Promise<Array<string>> {
            return await tauriInvoke("plugin:ardeck|get_connecting_serials");
        },
        async getPorts(): Promise<Array<SerialPortInfo>> {
            return await tauriInvoke("plugin:ardeck|get_ports");
        }
    }
};