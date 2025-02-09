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

// enum SwitchType {
//     Unknown = -1,
//     Digital = 0,
//     Analog = 1,
// }

export const SwitchType = {
    Unknown: "unknown",
    Digital: "digital",
    Analog: "analog",
} as const;

export type SwitchType = (typeof SwitchType)[keyof typeof SwitchType];

/**
 * アクションマッピングの設定を表す型
 * @property switchType - スイッチの種類（デジタルまたはアナログ）
 * @property switchId - スイッチの識別子
 * @property pluginId - プラグインの識別子
 * @property actionId - アクションの識別子
 */
export type ActionMap = {
    switchType: SwitchType;
    switchId: number;
    pluginId: string;
    actionId: string;
};

export const defaultActionMap: ActionMap = {
    switchType: SwitchType.Digital,
    switchId: 0,
    pluginId: "",
    actionId: "",
};

export type SerialPortInfo = {
    port_name: string;
    port_type: {
        UsbPort?: {
            vid: number;
            pid: number;
            serial_number: string;
            manufacturer: string;
            product: string;
        };
        PciPort?: {};
        BluetoothPort?: {};
    };
};

export type OnMessageSerial = {
    data: number;
    timestamp: number;
};

export type SwitchInfo = {
    switchType: SwitchType;
    switchId: number;
    switchState: number;
    timestamp: number;
}

export type ActionTarget = {
    actionId: string;
    pluginId: string;
}

export type Action = {
    switch: SwitchInfo;
    target: ActionTarget;
};

export type serialPortState = {
    port_name: string;
    // status: "open" | "closed" | "error" = "closed";
    // port_state: {
    //     open: boolean,
    //     baud_rate: number,
    //     data_bits: number,
    //     stop_bits: number,
    //     parity: string,
    //     flow_control: string
    // }
};
