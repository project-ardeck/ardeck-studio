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

import { useEffect, useState } from "react";
import BackToRoot from "../_component/back_to_root";
import { ArdeckProfileConfigItem, SerialPortInfo } from "../../lib/ardeck";
import { invoke } from "../../tauri/invoke";
import { listen } from "../../tauri/listen";
import Popup from "../../component/popup";
import { Link } from "react-router";
import { UnlistenFn } from "@tauri-apps/api/event";

export default function Devices() {
    const [devices, setDevices] = useState<[string, SerialPortInfo][]>([]);
    const [deviceProfileList, setDeviceProfileList] = useState<
        [string, string][]
    >([]);
    const [deviceSetting, setDeviceSetting] = useState<string>("");

    const questionDeviceName = (): string => {
        const name = prompt("Device name");
        return name ? name : "";
    };

    /**
     * 接続中のデバイスのうち、保存されていないデバイスを新たに保存する
     * @param deviceId - デバイスID
     */
    const saveNewDeviceHandler = (deviceId: string) => {
        const deviceName = questionDeviceName();

        console.log("save new device");

        const newDevice: ArdeckProfileConfigItem = {
            deviceId,
            deviceName: "",
            baudRate: 9600,
            description: "",
        };

        invoke.settings.ardeckPresets.saveArdeckProfile(newDevice);
    };

    useEffect(() => {
        invoke.ardeck.getPorts().then((ports) => setDevices(ports));
        invoke.settings.ardeckPresets
            .getArdeckProfileList()
            .then((profiles) => setDeviceProfileList(profiles));

        const onPorts = listen.onPorts((ports) => {
            console.log("listen on ports");
            setDevices(ports);
        });

        return () => {
            onPorts.then((unlisten) => unlisten());
        };
    }, []);

    return (
        <div>
            <div>Devices</div>
            <h2 className="text-xl font-bold">Saved</h2>
            {devices.map((device) => {
                if (!device[1].port_type.UsbPort) return null;

                const profile = deviceProfileList.find(
                    (profile) => profile[0] === device[0],
                );

                if (!profile) return null;

                return (
                    <Link
                        className="flex w-64 flex-col rounded-md bg-bg-secondary p-4 *:overflow-hidden *:text-ellipsis *:text-nowrap"
                        key={device[0]}
                        to={device[0]}
                    >
                        <div
                            className={`text-xl font-bold ${profile[1] ? "" : "italic opacity-50"}`}
                        >
                            {profile[1] ? profile[1] : "No name"}
                        </div>
                        <div>port_name: {device[1].port_name}</div>
                    </Link>
                );
            })}
            <h2 className="text-xl font-bold">New</h2>
            <div className="flex flex-col gap-2">
                {devices.map((device) => {
                    console.log(device);
                    if (!device[1].port_type.UsbPort) return null;

                    if (
                        deviceProfileList.find(
                            (profile) => profile[0] === device[0],
                        )
                    )
                        return null;

                    return (
                        <div
                            className="flex w-64 flex-col rounded-md bg-bg-secondary p-4 *:overflow-hidden *:text-ellipsis *:text-nowrap"
                            key={device[0]}
                        >
                            <div className="text-xl font-bold">
                                {device[1].port_name}
                            </div>
                            <div
                                title={device[1].port_type.UsbPort.manufacturer}
                            >
                                {device[1].port_type.UsbPort.manufacturer}
                            </div>
                            <div title={device[1].port_type.UsbPort.product}>
                                {device[1].port_type.UsbPort.product}
                            </div>
                            {/* <div>port_id: {device[0]}</div> */}
                            {/* <div>pid: {device[1].port_type.UsbPort.pid}</div> */}
                            {/* <div>
                                serial_number:{" "}
                                {device[1].port_type.UsbPort.serial_number}
                            </div>
                            <div>vid: {device[1].port_type.UsbPort.vid}</div> */}
                            <input
                                className="mt-2"
                                type="button"
                                value="Save Device"
                                onClick={() => saveNewDeviceHandler(device[0])}
                            />
                        </div>
                    );
                })}
            </div>
            <BackToRoot>Back to root</BackToRoot>
        </div>
    );
}
