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
import Button from "../_component/Button";

export default function Devices() {
    const [devices, setDevices] = useState<[string, SerialPortInfo][]>([]);
    const [deviceProfileList, setDeviceProfileList] = useState<
        [string, string][]
    >([]);

    const [connectingDevice, setConnectingDevice] = useState<string[]>();

    const questionDeviceName = (): string => {
        const name = prompt("Device name");
        return name ? name : "";
    };

    /**
     * 接続中のデバイスのうち、保存されていないデバイスを新たに保存する
     * @param deviceId - デバイスID
     */
    const saveNewDeviceHandler = (deviceId: string) => {
        const newDevice: ArdeckProfileConfigItem = {
            deviceId,
            deviceName: "",
            baudRate: 9600,
            description: "",
        };

        invoke.settings.ardeckPresets.saveArdeckProfile(newDevice);
    };

    /**
     * デバイスを接続する
     *  @param deviceId - デバイスID
     */
    const openPort = async (deviceId: string) => {
        const deviceName = devices.find((device) => device[0] === deviceId);

        if (!deviceName) return;
        console.log(deviceName[1].port_name);

        await invoke.ardeck.openPort(deviceName[1].port_name, 9600);
    };

    /**
     * デバイスを切断する
     * @param deviceId - デバイスID
     */
    const closePort = async (deviceId: string) => {
        const deviceName = devices.find((device) => device[0] === deviceId);

        if (!deviceName) return;

        console.log(deviceName[1].port_name);

        await invoke.ardeck.closePort(deviceName[1].port_name);
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

        const refreshConnectingDevice = async () => {
            setConnectingDevice(await invoke.ardeck.getConnectingSerials());
        };

        const onOpenHandle = listen.onOpenSerial(refreshConnectingDevice);
        const onCloseHandle = listen.onCloseSerial(refreshConnectingDevice);
        refreshConnectingDevice();

        return () => {
            onPorts.then((unlisten) => unlisten());
            onOpenHandle.then((unlisten) => unlisten());
            onCloseHandle.then((unlisten) => unlisten());
        };
    }, []);

    return (
        <div>
            <h1 className="pagetitle mb-4">Devices</h1>
            <h2 className="text-xl font-bold">Saved</h2>
            <div className="flex gap-2">
                {devices.map((device) => {
                    if (!device[1].port_type.UsbPort) return null;

                    const profile = deviceProfileList.find(
                        (profile) => profile[0] === device[0],
                    );

                    if (!profile) return null;

                    console.log("connectingDevice: ", connectingDevice);

                    const isConnecting = Boolean(
                        connectingDevice?.find(
                            (id) => id === device[1].port_name,
                        ),
                    );

                    console.log("isConnecting: ", isConnecting);

                    return (
                        <div
                            className="bg-bg-secondary flex w-64 flex-col rounded-md p-4 *:overflow-hidden *:text-nowrap *:text-ellipsis"
                            key={device[0]}
                        >
                            <div
                                className={`text-xl font-bold ${profile[1] ? "" : "italic opacity-50"}`}
                            >
                                {profile[1] ? profile[1] : "No name"}
                            </div>
                            <div>port_name: {device[1].port_name}</div>
                            <Button
                                onClick={() => {
                                    isConnecting
                                        ? closePort(device[0])
                                        : openPort(device[0]);
                                }}
                                className={`bg-bg-tertiary mt-2 rounded-sm ${
                                    isConnecting
                                        ? "hover:bg-accent-negative hover:text-text-reverse"
                                        : "hover:bg-accent-positive hover:text-text-reverse"
                                }`}
                            >
                                {isConnecting ? "Disconnect" : "Connect"}
                            </Button>
                            <Link
                                className="input bg-bg-tertiary mt-2 rounded-sm text-center"
                                to={device[0]}
                            >
                                Edit
                            </Link>
                        </div>
                    );
                })}
            </div>
            <h2 className="text-xl font-bold">New</h2>
            <div className="flex gap-2">
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
                            className="bg-bg-secondary flex w-64 flex-col rounded-md p-4 *:overflow-hidden *:text-nowrap *:text-ellipsis"
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
            {/* <BackToRoot>Back to root</BackToRoot> */}
        </div>
    );
}
