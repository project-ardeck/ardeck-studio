import { ReactNode, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

type switchStatesObject = {
    state: number,
    timestamp: Date
}

export default function ForDev() {
    const isInit = useRef(false); // for Develop

    const theme = useRef("" as string);

    const [portList, setPortList] = useState([] as SerialPortInfo[]);
    const [devLogs, setDevLogs] = useState([] as string[]);
    const [connectedSerialList, setConnectedSerialList] = useState([] as string[]);

    const [switchStates, setSwitchStates] = useState({
        Analog: new Map<number, switchStatesObject>(),
        Digital: new Map<number, switchStatesObject>()
    });

    const pushLog = (log: string) => {
        const _DATE = new Date();
        const M = _DATE.getMonth().toString().padStart(2, '0');
        const d = _DATE.getDate().toString().padStart(2, '0');
        const h = _DATE.getHours().toString().padStart(2, '0');
        const m = _DATE.getMinutes().toString().padStart(2, '0');
        const s = _DATE.getSeconds().toString().padStart(2, '0');
        const mm = _DATE.getMilliseconds().toString().padStart(4, '0');
        const date = `${M}-${d} ${h}:${m}:${s}.${mm}`;
        const formattedLog = `[${date}] ${log}`

        setDevLogs(prevLogs => [formattedLog, ...prevLogs]);
    }

    const addCS = (addSerial: string) => { // CS: Connected Serial
        setConnectedSerialList(prevList => Array.from(new Set([...prevList, addSerial])));
    }

    const rmvCS = (rmvSerial: string) => {
        setConnectedSerialList(prevList => prevList.filter(n => n !== rmvSerial));
    }

    const getPorts = async () => {
        let ports = await invoke("get_ports") as SerialPortInfo[];
        setPortList(ports);
    }

    const getConnectingPorts = async () => {
        let ports = await invoke("get_connecting_serials") as string[];

        setConnectedSerialList(ports);
    }

    const serialOpenRequest = async (portName: string) => {
        invoke("open_port", { portName: portName, baudRate: 9600 })
            .then(() => {
                // addCS(portName);
                // pushLog(`OPEN: ${portName}`);
            })
            .catch((e) => {
                pushLog(e);
            });
    }

    const closeHandler = async (portName: string) => {
        invoke("close_port", { portName: portName })
            .then(() => {
                // rmvCS(portName);
                // pushLog(`CLOSE: ${portName}`);
            })
            .catch((e) => {
                pushLog(e);
            });
    }

    useEffect(() => {
        if (!isInit.current) {
            // 開発中にuseEffectが2回実行されないようにしている。
            // https://react.dev/learn/synchronizing-with-effects#how-to-handle-the-effect-firing-twice-in-development
            isInit.current = true;

            listen("on-ports", (e) => { // ポートリストの更新
                setPortList(e.payload as SerialPortInfo[]);
            });

            listen("on-open-serial", (e) => { // シリアル通信の開始
                addCS(e.payload as string);
                pushLog(`OPEN: ${e.payload}`);
            });

            listen("on-close-serial", (e) => { // シリアル通信の終了
                rmvCS(e.payload as string);
                pushLog(`CLOSE: ${e.payload}`)
            })

            listen("on-error-serial", (e) => { // シリアル通信中にエラーが発生した
                pushLog(e.payload as string);
            });

            listen("on-message-serial", (e) => { // シリアル通信のメッセージ
                const payload = e.payload as OnMessageSerial;
                // console.log(payload);
                const msg = payload.data;
                const msgBinRaw = msg.toString(2);
                let msgBin = ""
                msgBinRaw.padStart((Math.floor(msgBinRaw.length / 8) + 1) * 8, "0").split("").map((val, index) => {
                    msgBin += val;

                    if ((index + 1) % 4 == 0) {
                        msgBin += " ";
                    }
                });
                // pushLog(`${msg.toString().padStart(3, "0")} : ${msgBin}`)

                const DorA = (msg & 0b10000000) >> 7;
                let pin: number;
                let state: number;
                const timestamp = payload.timestamp; // millis
                if (DorA) {
                    // Analog
                } else {
                    // Digital
                    pin = (msg & 0b01111110) >> 1;
                    state = msg & 0b00000001;

                    setSwitchStates((prev) => {
                        const newMap = new Map(prev.Digital);
                        newMap.set(pin, {
                            state: state,
                            timestamp: new Date(timestamp)
                        });
                        return { ...prev, Digital: newMap };
                    });

                    console.log(switchStates);

                    // pushLog(`\t[Digital] ${pin.toString().padStart(2, '0')} : ${state ? "HIGH" : "LOW"}`);
                }
            });

            getPorts();

            getConnectingPorts();
            // console.log("fook test");

            window.windowTheme.setTheme(
                window.localStorage.getItem("theme") as ThemeList ?? "default-dark"
            );
        }
    }, []);

    return (
        <div className="font-fordev w-full h-full bg-bg-primary text-text-primary flex flex-col">
            <div data-tauri-drag-region className="p-4 h-full flex-1 overflow-auto flex flex-col gap-2">

                <Infomations>
                    <h2 className="text-xl font-bold mb-2 pb-2 border-b-2">
                        Port
                    </h2>                    
                    <div className="flex gap-2 mt-2">
                        {portList.map((port) => {
                            // if (port.port_type.UsbPort?.manufacturer.split(" ")[0] == "Arduino") {
                            //     return null;
                            // }

                            // TODO: プロトコルバージョンの指定をできるようにする

                            if (port.port_type.UsbPort?.vid != 0x2341) {
                                return null;
                            }

                            let product = port.port_type.UsbPort?.product ?
                                port.port_type.UsbPort?.product :
                                "Unknown";

                            let serialNum = port.port_type.UsbPort?.serial_number ?
                                port.port_type.UsbPort.serial_number :
                                "Unknown";

                            let isConnect = connectedSerialList.some(val => {
                                return port.port_name == val;
                            })

                            return (
                                <div key={port.port_name} className="bg-bg-secondary p-2 rounded-lg border-bg-4 border-2 shadow-lg">
                                    <div>
                                        {port.port_name}
                                    </div>
                                    <div>
                                        {product}
                                    </div>
                                    <div className="text-sm text-text-primary text-opacity-50">
                                        {serialNum}
                                    </div>
                                    <div className="flex flex-col gap-2 mt-2">
                                        <button
                                            disabled={isConnect}
                                            className="bg-accent-positive rounded-lg text-bg-primary px-4 transition-colors disabled:bg-bg-quaternary disabled:text-text-primary disabled:text-opacity-50"
                                            onClick={() => {
                                                serialOpenRequest(port.port_name);
                                                // buttonLoadingAnimation;
                                            }}
                                        >
                                            Connect
                                        </button>
                                        <button
                                            disabled={!isConnect}
                                            className="bg-accent-negative rounded-lg text-bg-primary px-2 transition-colors disabled:bg-bg-quaternary disabled:text-text-primary disabled:text-opacity-50"
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
                <Infomations>
                    <h2 className="text-xl font-bold mb-2 pb-2 border-b-2">
                        Device Info
                    </h2>
                    <div>
                        <h3 className="font-bold text-lg">
                            Digital
                        </h3>
                        {Array.from(switchStates.Digital).map(([pin, state]) => {
                            const className = state.state ? "border-accent-primary border-opacity-50" : "border-bg-quaternary";
                            const date = state.timestamp.getDate().toString().padStart(2, '0');
                            const hour = state.timestamp.getHours().toString().padStart(2, '0');
                            const minute = state.timestamp.getMinutes().toString().padStart(2, '0');
                            const second = state.timestamp.getSeconds().toString().padStart(2, '0');
                            const millisecond = state.timestamp.getMilliseconds().toString().padStart(3, '0');
                            const time = `${hour}:${minute}:${second}.${millisecond}`;

                            console.log(state.timestamp);

                            return (

                                <div className={`${className} px-4 mb-1 border-2 rounded-md flex items-center place-content-between transition-colors`}>
                                    <div>

                                        {`${pin.toString().padStart(2, '0')} : ${state.state ? "HIGH" : "LOW"}`}
                                    </div>
                                    <div className="text-sm text-text-primary text-opacity-50">
                                        {time}
                                    </div>
                                </div>
                            )
                        })}
                    </div>
                </Infomations>
                <Infomations>
                    <h2 className="text-xl font-bold mb-2 pb-2 border-b-2">
                        Logs
                    </h2>
                    <div className="h-80 overflow-auto">
                        <pre>
                            {devLogs.map((msg) => {
                                return (
                                    <div key={msg}>
                                        {msg}
                                    </div>
                                )
                            })}
                        </pre>
                    </div>
                </Infomations>

                <Infomations>
                    <h2 className="text-xl font-bold mb-2 pb-2 border-b-2">
                        port info
                    </h2>
                    <div>
                        <pre>
                            {JSON.stringify(portList, null, "\t")}
                        </pre>
                    </div>
                </Infomations>

                <Infomations>
                    <h2 className="text-xl font-bold mb-2 pb-2 border-b-2">
                        Theme
                    </h2>
                    <div>
                        <select
                            className="rounded-md bg-bg-quaternary text-text-primary px-4 py-2 w-full"
                            onChange={(e) => {
                                window.localStorage.setItem("theme", e.target.value);
                                window.windowTheme.setTheme(e.target.value as ThemeList);
                            }}
                        >
                            {window.windowTheme.themeList.map((theme) => {
                                return (
                                    <option
                                        key={theme}
                                        value={theme}
                                        selected={theme == window.windowTheme.theme ? true : false}
                                    >
                                        {theme}
                                    </option>
                                );
                            })}
                        </select>
                    </div>
                </Infomations>
            </div>
        </div>
    );
}

function Infomations({
    children
}: {
    children: ReactNode
}) {
    return (
        <div className="p-4 bg-bg-secondary rounded-lg">
            {children}
        </div>
    );
}


