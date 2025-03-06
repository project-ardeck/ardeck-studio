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
import { SerialPortInfo } from "../../types/ardeck";
import { invoke } from "../../tauri/invoke";
import { listen } from "../../tauri/listen";
import Popup from "../../component/popup";
import { Link } from "react-router";

export default function Devices() {
    const [devices, setDevices] = useState<[string, SerialPortInfo][]>([]);
    const [deviceSetting, setDeviceSetting] = useState<string>("");

    useEffect(() => {
        invoke.ardeck.getPorts().then((ports) => setDevices(ports));

        listen.onPorts((ports) => setDevices(ports));
    }, []);

    return (
        <div>
            <div>Devices</div>
            <h2 className="text-xl font-bold">Saved</h2>
            None
            <h2 className="text-xl font-bold">New</h2>
            <div className="flex flex-col gap-2">
                {devices.map((device) => {
                    console.log(device);
                    if (!device[1].port_type.UsbPort) return null;

                    return (
                        <Link
                            className="flex flex-col bg-bg-secondary"
                            key={device[1].port_name}
                            to={device[0]}
                        >
                            <div>port_name: {device[1].port_name}</div>
                            {/* <Popup
                                title={device.port_name}
                                onClose={() => {}}
                                onOpen={() => {}}
                            > */}
                            <div>port_id: {device[0]}</div>
                            <div>
                                manufacturer:{" "}
                                {device[1].port_type.UsbPort.manufacturer}
                            </div>
                            <div>pid: {device[1].port_type.UsbPort.pid}</div>
                            <div>
                                serial_number:{" "}
                                {device[1].port_type.UsbPort.serial_number}
                            </div>
                            <div>vid: {device[1].port_type.UsbPort.vid}</div>
                            <div>
                                product: {device[1].port_type.UsbPort.product}
                            </div>
                            {/* </Popup> */}
                        </Link>
                    );
                })}
            </div>
            <BackToRoot>Back to root</BackToRoot>
        </div>
    );
}
