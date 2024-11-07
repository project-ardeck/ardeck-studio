import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";

export default function Settings() {
    const [settingList, setSettingList] = useState<string[]>([]);

    const getSettingIdList = async () => {
        const list: string[] = await invoke("plugin:settings|get_setting_list");

        setSettingList(list);
        console.log("setting list: ", list);
    }

    const getSetting = async (e: string) => {
        const setting = await invoke("plugin:settings|get_setting", { configId: e });
        console.log("setting: ", setting);
    }
    

    useEffect(() => {
        getSettingIdList();
    }, []);

    return (
        <div>
            {settingList.map((s, i) => <div><input onClick={() => getSetting(s)} type="button" key={i} value={s} /></div>)}
        </div>
    )
}
