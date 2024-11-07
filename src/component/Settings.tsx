import { invoke } from "@tauri-apps/api";

export default function Settings() {
    const getSettingFileNames = async () => {
        const list = await invoke("plugin:settings|get_setting_list");
        console.log(list);
    }

    return (
        <div>
            Settings
        </div>
    )
}