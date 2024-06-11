import { ReactNode, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";


const FakePortList: serialPortInfo[] = [
    {
        port_name: "COM2",
        port_type: {
            UsbPort: {
                vid: 19191919,
                pid: 45454545,
                serial_number: "Fuck",
                manufacturer: "Arduino ??",
                product: "Arduino ?"
            }
        }
    },
    {
        port_name: "COM102",
        port_type: {
            UsbPort: {
                vid: 1915234,
                pid: 45462344354545,
                serial_number: "weodfiajwe",
                manufacturer: "Arduino ??",
                product: "Arduino ?"
            }
        }
    },
    {
        port_name: "COM0725",
        port_type: {
            UsbPort: {
                vid: 1236527,
                pid: 5686793546245,
                serial_number: "gaerandfgad",
                manufacturer: "Arduino ??",
                product: "Arduino ?"
            }
        }
    }
]

export default function ForDev() {
    const isInit = useRef(false); // for Develop

    const [portList, setPortList] = useState([] as serialPortInfo[]);
    // const [targetPort, setTargetPort] = useState("");
    const [devLogs, setDevLogs] = useState([] as string[]);
    const [connectedSerialList, setConnectedSerialList] = useState([] as string[]);


    const pushLog = (log: string) => {
        const _DATE = new Date();
        const M = _DATE.getMonth().toString().padStart(2, '0');
        const d = _DATE.getDate().toString().padStart(2, '0');
        const h = _DATE.getHours().toString().padStart(2, '0');
        const m = _DATE.getMinutes().toString().padStart(2, '0');
        const s = _DATE.getSeconds().toString().padStart(2, '0');
        const mm = _DATE.getMilliseconds().toString().padStart(4, '0');
        const TZ = _DATE.getUTCHours().toString().padStart(2, '0');
        const date = `${M}-${d} ${h}:${m}:${s}.${mm} (${TZ})`;
        const formattedLog = `[${date}]: ${log}`

        setDevLogs(prevLogs => [formattedLog, ...prevLogs]);
    }

    const addCS = (addSerial: string) => { // CS: Connected Serial
        setConnectedSerialList(prevList => Array.from(new Set([...prevList, addSerial])));
    }

    const rmvCS = (rmvSerial: string) => {
        setConnectedSerialList(prevList => prevList.filter(n => n !== rmvSerial));
    }

    // let isSelectTargetPort = false;

    // if (!(targetPort == "")) {
    //     isSelectTargetPort = true;
    // }

    const getPorts = async () => {
        let ports = await invoke("get_ports") as serialPortInfo[];
        setPortList(ports);
        // setPortList(FakePortList);
    }

    const serialOpenRequest = async (portName: string) => {
        emit("request-open-serial", { targetPort: portName })
    }

    const resetHandler = async (portName: string) => {
        emit("request-close-serial", { targetPort: portName });
    }

    useEffect(() => {
        if (!isInit.current) {
            // 開発中にuseEffectが2回実行されないようにしている。
            // https://react.dev/learn/synchronizing-with-effects#how-to-handle-the-effect-firing-twice-in-development
            isInit.current = true;

            listen("on-ports", (e) => {
                setPortList(e.payload as serialPortInfo[]);
            });

            listen("on-open-serial", (e) => {
                addCS(e.payload as string);
                pushLog(`OPEN: ${e.payload}`);
            });

            listen("on-close-serial", (e) => {
                rmvCS(e.payload as string);
                pushLog(`CLOSE: ${e.payload}`)
            })

            listen("on-error-serial", (e) => {
                pushLog(e.payload as string);
            });

            listen("on-message-serial", (e) => {
                pushLog(e.payload as string);
            });


            getPorts();
            // console.log("fook test");
        }
    }, []);

    return (
        <div className="font-0xp w-full h-full bg-bg-2 text-white flex flex-col">
            <div data-tauri-drag-region className="p-4 h-full flex-1 overflow-auto flex flex-col gap-2">

                <Infomations>
                    <h2 className="text-xl font-bold mb-2 pb-2 border-b-2">
                        Ports
                    </h2>
                    <div className="flex gap-2">
                        {/* Arduinoだと確定でわかる場合 */}
                        {portList.map((port) => {
                            if (!(port.port_type.UsbPort?.manufacturer.split(" ")[0] == "Arduino")) {
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
                                <div className="bg-bg-1 p-2 rounded-lg border-bg-4 border-2 shadow-lg">
                                    <div>
                                        {port.port_name}
                                    </div>
                                    <div>
                                        {product}
                                    </div>
                                    <div className="text-sm text-white opacity-40">
                                        {serialNum}
                                    </div>
                                    <div className="flex flex-col gap-2 mt-2">
                                        <button disabled={isConnect} className="bg-green rounded-lg text-black px-4 transition-colors disabled:bg-bg-2 disabled:text-bg-4" onClick={() => serialOpenRequest(port.port_name)}>Connect</button>
                                        <button disabled={!isConnect} className="bg-red rounded-lg text-white px-2 transition-colors disabled:bg-bg-2 disabled:text-bg-4" onClick={() => resetHandler(port.port_name)}>Disconnect</button>
                                    </div>
                                </div>
                            );
                        })}
                    </div>
                    <div className="flex gap-2 mt-2">
                        {/* Arduinoではないかもしれない */}
                        {portList.map((port) => {
                            if (port.port_type.UsbPort?.manufacturer.split(" ")[0] == "Arduino") {
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
                                <div className="bg-bg-3 p-2 rounded-lg border-bg-4 border-2 shadow-lg">
                                    <div>
                                        {port.port_name}
                                    </div>
                                    <div>
                                        {product}
                                    </div>
                                    <div className="text-sm text-white opacity-40">
                                        {serialNum}
                                    </div>
                                    <div className="flex flex-col gap-2 mt-2">
                                        <button disabled={isConnect} className="bg-green rounded-lg text-black px-4 transition-colors disabled:bg-bg-2 disabled:text-bg-4" onClick={() => serialOpenRequest(port.port_name)}>Connect</button>
                                        <button disabled={!isConnect} className="bg-red rounded-lg text-white px-2 transition-colors disabled:bg-bg-2 disabled:text-bg-4" onClick={() => resetHandler(port.port_name)}>Disconnect</button>
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
                </Infomations>
                <Infomations>
                    <h2 className="text-xl font-bold mb-2 pb-2 border-b-2">
                        Logs
                    </h2>
                    <div className="h-80 overflow-auto">
                        {devLogs.map((msg) => {
                            return (
                                <div>
                                    {msg}
                                </div>
                            )
                        })}
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
        <div className="p-4 bg-bg-3 rounded-lg">
            {children}
        </div>
    );
}


