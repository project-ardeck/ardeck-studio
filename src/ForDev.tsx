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

import { ReactNode, useEffect, useRef, useState } from "react";
import { invoke } from "./tauri/invoke";
import { emit, listen } from "@tauri-apps/api/event";

import ActionMappingForm from "./component/ActionMappingForm";
import Settings from "./component/Settings";
import { Action, SerialPortInfo, SwitchInfo, SwitchType } from "./types/ardeck";

type switchStatesObject = {
    state: number;
    timestamp: Date;
    raw: number[];
};

const SerialProtocolVersionList = ["2014-06-03", "2024-06-17"] as const;
type SerialProtocolVersion = (typeof SerialProtocolVersionList)[number];

const BaudRateList = [
    150,
    200,
    300,
    600,
    1200,
    1800,
    2400,
    4800,
    9600, //default
    19200,
    28800,
    38400,
    57600,
    76800,
    115200,
    192000, // ?!
    230400,
    576000,
    921600,
] as const;
type BaudRate = (typeof BaudRateList)[number];

const devLogLimit = 100;

export default function ForDev() {
    const isInit = useRef(false); // for Develop

    const [isShowotherArduinoDevice, setIsShowotherArduinoDevice] =
        useState(false);

    const [themeInfos, setThemeInfos] = useState<ThemeInfo[]>([]);

    const [deviceList, setDeviceList] = useState<SerialPortInfo[]>([]);
    const [devLogs, setDevLogs] = useState<string[]>([]);
    const [connectedSerialList, setConnectedSerialList] = useState<string[]>(
        [],
    );

    const [baudRateOption, setBaudRateOption] = useState<BaudRate>(9600);

    const [switchStates, setSwitchStates] = useState({
        Analog: new Map<number, SwitchInfo>(),
        Digital: new Map<number, SwitchInfo>(),
    });

    const pushLog = (log: string) => {
        const _DATE = new Date();
        const M = _DATE.getMonth().toString().padStart(2, "0");
        const d = _DATE.getDate().toString().padStart(2, "0");
        const h = _DATE.getHours().toString().padStart(2, "0");
        const m = _DATE.getMinutes().toString().padStart(2, "0");
        const s = _DATE.getSeconds().toString().padStart(2, "0");
        const mm = _DATE.getMilliseconds().toString().padStart(4, "0");
        const date = `${M}-${d} ${h}:${m}:${s}.${mm}`;
        const formattedLog = `[${date}] ${log}`;

        if (devLogs.length >= devLogLimit) {
            setDevLogs((prevLogs) => prevLogs.slice(0, devLogLimit - 1));
        }

        setDevLogs((prevLogs) => [formattedLog, ...prevLogs]);
    };

    const addCS = (addSerial: string) => {
        // CS: Connected Serial
        setConnectedSerialList((prevList) =>
            Array.from(new Set([...prevList, addSerial])),
        );
    };

    const rmvCS = (rmvSerial: string) => {
        setConnectedSerialList((prevList) =>
            prevList.filter((n) => n !== rmvSerial),
        );
    };

    const getPorts = async () => {
        // let ports = await invoke("plugin:ardeck|get_ports") as SerialPortInfo[];
        let ports = await invoke.ardeck.getPorts();
        setDeviceList(ports);
    };

    const getConnectingPorts = async () => {
        let ports = await invoke.ardeck.getConnectingSerials();

        setConnectedSerialList(ports);
    };

    const serialOpenRequest = async (portName: string) => {
        // invoke("plugin:ardeck|open_port", { portName: portName, baudRate: baudRateOption })
        //     .then(() => {
        //         // addCS(portName);
        //         // pushLog(`OPEN: ${portName}`);
        //     })
        //     .catch((e) => {
        //         pushLog(e);
        //     });

        await invoke.ardeck.openPort(portName, baudRateOption);
    };

    const closeHandler = async (portName: string) => {
        // invoke("plugin:ardeck|close_port", { portName: portName })
        //     .then(() => {
        //         // rmvCS(portName);
        //         // pushLog(`CLOSE: ${portName}`);
        //     })
        //     .catch((e) => {
        //         pushLog(e);
        //     });

        await invoke.ardeck.closePort(portName);
    };

    const getThemeInfos = async () => {
        const list = await window.windowTheme.themeList();
        setThemeInfos(list);
    };

    useEffect(() => {
        if (!isInit.current) {
            // 開発中にuseEffectが2回実行されないようにしている。
            // https://react.dev/learn/synchronizing-with-effects#how-to-handle-the-effect-firing-twice-in-development
            isInit.current = true;

            listen("on-ports", (e) => {
                // ポートリストの更新
                setDeviceList(e.payload as SerialPortInfo[]);
                pushLog("aaa");
            });

            listen("on-open-serial", (e) => {
                // シリアル通信の開始
                addCS(e.payload as string);
                pushLog(`OPEN: ${e.payload}`);
            });

            listen("on-close-serial", (e) => {
                // シリアル通信の終了
                rmvCS(e.payload as string);
                pushLog(`CLOSE: ${e.payload}`);
            });

            listen("on-error-serial", (e) => {
                // シリアル通信中にエラーが発生した
                pushLog(e.payload as string);
            });

            listen("on-message-serial", (e) => {
                // シリアル通信のメッセージ
                const payload = e.payload as SwitchInfo;
                // console.log(payload);
                // document.getElementById("raw_data")!.innerHTML = JSON.stringify(payload, null, 2);
                // pushLog(`${payload.switchType}`)

                const data = {
                    state: payload.switchState,
                    timestamp: new Date(payload.timestamp),
                    raw: [],
                };

                if (payload.switchType === SwitchType.Digital) {
                    // console.log("0");
                    setSwitchStates((prevState) => {
                        const newState = new Map(prevState.Digital);
                        newState.set(payload.switchId, payload);
                        return {
                            ...prevState,
                            Digital: newState,
                        };
                    });
                } else if (payload.switchType === SwitchType.Analog) {
                    // console.log("1");
                    setSwitchStates((prevState) => {
                        const newState = new Map(prevState.Analog);
                        newState.set(payload.switchId, payload);
                        return {
                            ...prevState,
                            Analog: newState,
                        };
                    });
                }
            });

            getPorts();

            getConnectingPorts();

            getThemeInfos();

            window.windowTheme.setTheme(
                (window.localStorage.getItem("theme") as ThemeList) ??
                    "default-dark",
            );
        }
    }, []);

    return (
        <div className="flex h-full w-full flex-col bg-bg-primary font-fordev text-text-primary">
            <div
                data-tauri-drag-region
                className="flex h-full flex-1 flex-col gap-2 overflow-auto p-4"
            >
                <Infomations title="Port">
                    <div className="flex flex-col gap-2">
                        <div>
                            <select
                                value={baudRateOption}
                                className="w-full rounded-md bg-bg-quaternary px-4 py-2 text-text-primary"
                                onChange={(e) => {
                                    setBaudRateOption(
                                        Number(e.target.value) as BaudRate,
                                    );
                                }}
                            >
                                {BaudRateList.map((e) => {
                                    return <option>{e}</option>;
                                })}
                            </select>
                        </div>
                        <div>
                            <input
                                type="checkbox"
                                className="rounded-lg"
                                name="isArduinoOnly"
                                id="isArduinoOnly"
                                checked={isShowotherArduinoDevice}
                                onChange={(e) => {
                                    setIsShowotherArduinoDevice(
                                        e.target.checked,
                                    );
                                }}
                            />
                            <label
                                htmlFor="isArduinoOnly"
                                className="select-none"
                            >
                                {" "}
                                Show other than Arduino
                            </label>
                        </div>
                    </div>
                    <div className="mt-2 flex gap-2">
                        {deviceList.map((port) => {
                            if (
                                port.port_type.UsbPort?.vid != 0x2341 &&
                                !isShowotherArduinoDevice
                            ) {
                                return null;
                            }

                            let product = port.port_type.UsbPort?.product
                                ? port.port_type.UsbPort?.product
                                : "Unknown";

                            let serialNum = port.port_type.UsbPort
                                ?.serial_number
                                ? port.port_type.UsbPort.serial_number
                                : "Unknown";

                            let isConnect = connectedSerialList.some((val) => {
                                return port.port_name == val;
                            });

                            return (
                                <div
                                    key={port.port_name}
                                    className="border-bg-4 rounded-lg border-2 bg-bg-secondary p-2 shadow-lg"
                                >
                                    <div>{port.port_name}</div>
                                    <div>{product}</div>
                                    <div className="text-sm text-text-primary text-opacity-50">
                                        {serialNum}
                                    </div>
                                    <div className="mt-2 flex flex-col gap-2">
                                        <button
                                            aria-disabled={isConnect}
                                            className="rounded-lg bg-accent-positive px-4 text-bg-primary transition-colors aria-disabled:cursor-default aria-disabled:bg-bg-quaternary aria-disabled:text-text-primary aria-disabled:text-opacity-50"
                                            onClick={() => {
                                                serialOpenRequest(
                                                    port.port_name,
                                                );
                                                // buttonLoadingAnimation;
                                            }}
                                        >
                                            Connect
                                        </button>
                                        <button
                                            aria-disabled={!isConnect}
                                            className="rounded-lg bg-accent-negative px-2 text-bg-primary transition-colors aria-disabled:cursor-default aria-disabled:bg-bg-quaternary aria-disabled:text-text-primary aria-disabled:text-opacity-50"
                                            onClick={() => {
                                                closeHandler(port.port_name);
                                                // buttonLoadingAnimation;
                                            }}
                                        >
                                            Disconnect
                                        </button>
                                    </div>
                                </div>
                            );
                        })}
                    </div>
                </Infomations>
                <Infomations title="Device Info">
                    <h3 className="mt-4 text-lg font-bold">Digital</h3>
                    {Array.from(switchStates.Digital).map(([pin, state]) => {
                        const timestamp = new Date(state.timestamp);
                        const className = state.switchState
                            ? "border-accent-primary border-opacity-50"
                            : "border-bg-quaternary";
                        const date = timestamp
                            .getDate()
                            .toString()
                            .padStart(2, "0");
                        const hour = timestamp
                            .getHours()
                            .toString()
                            .padStart(2, "0");
                        const minute = timestamp
                            .getMinutes()
                            .toString()
                            .padStart(2, "0");
                        const second = timestamp
                            .getSeconds()
                            .toString()
                            .padStart(2, "0");
                        const millisecond = timestamp
                            .getMilliseconds()
                            .toString()
                            .padStart(3, "0");
                        const time = `${hour}:${minute}:${second}.${millisecond}`;

                        // console.log(state.timestamp);

                        return (
                            <div
                                className={`${className} relative mb-1 flex place-content-between items-center rounded-md border-2 px-4 transition-colors`}
                            >
                                <div>
                                    {`${pin.toString().padStart(2, "0")} : ${state.switchState ? "HIGH" : "LOW"}`}
                                </div>
                                <div className="pointer-events-none absolute bottom-0 left-0 right-0 top-0 flex items-center justify-center">
                            
                                    {//state.raw.map((val, index) => {
                                     //   return (
                                     //       <span
                                     //           key={index}
                                     //           className="pointer-events-auto text-sm"
                                     //       >
                                     //           {val
                                     //               .toString(2)
                                     //               .padStart(8, "0")}
                                     //       </span>
                                     //   );
                                    //})
                                    }
                                </div>
                                <div className="text-sm text-text-primary text-opacity-50">
                                    {time}
                                </div>
                            </div>
                        );
                    })}
                    <h3 className="mt-4 text-lg font-bold">Analog</h3>
                    {Array.from(switchStates.Analog).map(([pin, state]) => {
                        const timestamp = new Date(state.timestamp);
                        const className = state.switchState
                            ? "border-accent-primary border-opacity-50"
                            : "border-bg-quaternary";
                        const date = timestamp
                            .getDate()
                            .toString()
                            .padStart(2, "0");
                        const hour = timestamp
                            .getHours()
                            .toString()
                            .padStart(2, "0");
                        const minute = timestamp
                            .getMinutes()
                            .toString()
                            .padStart(2, "0");
                        const second = timestamp
                            .getSeconds()
                            .toString()
                            .padStart(2, "0");
                        const millisecond = timestamp
                            .getMilliseconds()
                            .toString()
                            .padStart(3, "0");
                        const time = `${hour}:${minute}:${second}.${millisecond}`;

                        // console.log(state.timestamp);

                        const style = {
                            width: `${(state.switchState / 1023) * 100}%`,
                        };

                        return (
                            <div
                                className={`${className} relative mb-1 flex place-content-between items-center rounded-md border-2 px-4 transition-colors`}
                            >
                                <div className="z-10">
                                    {`${pin.toString().padStart(2, "0")} : ${state.switchState}`}
                                </div>
                                <div className="pointer-events-none absolute bottom-0 left-0 right-0 top-0 z-10 flex items-center justify-center gap-2">
                                    {//state.raw.map((val, index) => {
                                     //   return (
                                     //       <span
                                     //           key={index}
                                     //           className="pointer-events-auto text-sm"
                                     //       >
                                     //           {val
                                     //               .toString(2)
                                     //               .padStart(8, "0")}
                                     //       </span>
                                     //   );
                                     //   })
                                    }
                                </div>
                                <div className="z-10 text-sm text-text-primary text-opacity-50">
                                    {time}
                                </div>
                                <div
                                    className="pointer-events-none absolute bottom-0 left-0 top-0 z-0 bg-accent-primary opacity-25"
                                    style={style}
                                ></div>
                            </div>
                        );
                    })}
                </Infomations>
                <Infomations title="Logs">
                    <div className="h-80 overflow-auto">
                        <pre>
                            {devLogs.map((msg) => {
                                return <div key={msg}>{msg}</div>;
                            })}
                        </pre>
                    </div>
                </Infomations>

                <Infomations title="port info">
                    <pre>{JSON.stringify(deviceList, null, "\t")}</pre>
                </Infomations>

                <Infomations title="Theme">
                    <select
                        className="w-full rounded-md bg-bg-quaternary px-4 py-2 text-text-primary"
                        onChange={(e) => {
                            window.localStorage.setItem(
                                "theme",
                                e.target.value,
                            );
                            window.windowTheme.setTheme(
                                e.target.value as ThemeList,
                            );
                        }}
                    >
                        {themeInfos.map((theme) => {
                            return (
                                <option
                                    key={theme.id}
                                    value={theme.id}
                                    selected={
                                        theme.id == window.windowTheme.theme
                                            ? true
                                            : false
                                    }
                                >
                                    {theme.name}
                                </option>
                            );
                        })}
                    </select>
                </Infomations>

                <Infomations title="Settings">
                    <Settings />
                </Infomations>

                <Infomations title="Mappings">
                    <div className="flex flex-col gap-2">
                        <ActionMappingForm
                            // onSubmit={(e) => {
                            //     return;
                            // }}
                        />
                    </div>
                </Infomations>
            </div>
        </div>
    );
}

function Infomations({
    title,
    children,
}: {
    title?: string;
    children?: ReactNode;
}) {
    return (
        <div className="rounded-lg bg-bg-secondary p-4">
            <h2 className="mb-2 border-b-2 pb-2 text-xl font-bold">{title}</h2>
            <div>{children}</div>
        </div>
    );
}
