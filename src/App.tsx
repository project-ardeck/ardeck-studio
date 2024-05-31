import { FormEventHandler, ReactNode, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";


export default function App() {
    const [portList, setPortList] = useState([] as serialPortInfo);
    const [targetCOM, setTargetCOM] = useState("");

    // async function greet() {
    //     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //     setGreetMsg(await invoke("greet", { name }));
    // }

    const selectTargetCOM = (e: any) => {
        console.log(e.target.value);
        setTargetCOM(e.target.value);
    }

    async function getPorts() {
        let ports = await invoke("get_ports") as serialPortInfo;
        setPortList(ports);
    }
    useEffect(() => {
        async function aaa() {
            const test = await listen("on_ports", (e) => {
                setPortList(e.payload as serialPortInfo);
            });

        }
        aaa();
    }, [])

    const connectHandler = async () => {
        // emit("click");
        console.log("aaa");
        await invoke("open_port", { portName: targetCOM });
    }

    const resetHandler = async () => {
        await invoke("reset_port");
    }

    return (
        <div className="w-full h-full bg-bg-2 text-white flex flex-col">
            <div data-tauri-drag-region className="flex gap-2 pt-2 pb-4 px-4 bg-bg-1">
                {/* <button className="bg-blue px-2 border-2" onClick={getPorts}>get port list.</button> */}
                {/* <input type="text" onInput={refreshInput} className="text-black" /> */}
                <select name="" id="" className="bg-bg-3 flex-1 p-2 rounded-lg" onClick={getPorts} onChange={(e) => selectTargetCOM(e)}>
                    <option value="">Select Port</option>
                    {
                        portList.map((port) => {
                            return (
                                <option value={port.port_name}>
                                    {port.port_name}
                                </option>
                            )
                        })
                    }
                </select>
                <button className="bg-green rounded-lg text-black px-4" onClick={connectHandler}>Connect</button>
                <button className="bg-red rounded-lg text-white px-2" onClick={resetHandler}>STOP</button>
            </div>
            <div className="p-4 h-full flex-1 overflow-auto flex flex-col gap-2">
                <Infomations>
                    <h2 className="text-xl font-bold mb-2 pb-2 border-b-2">
                        port info
                    </h2>
                    <div>
                        Port List:{"\t"}
                        {JSON.stringify(portList)}
                    </div>
                    <div>
                        Target Port:{"\t"}
                        {targetCOM}
                    </div>
                </Infomations>
                <Infomations>
                aaa
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


