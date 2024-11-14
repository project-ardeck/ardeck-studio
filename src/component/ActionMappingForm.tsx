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

import {
    Component,
    FunctionComponent,
    ReactNode,
    RefCallback,
    useEffect,
    useRef,
    useState,
} from "react";
import { makeUid, randomStr } from "../util/props";
import { invoke } from "../tauri/invoke";
import {
    ActionMap,
    ActionMapConfig,
    ActionMapPreset,
    SwitchType,
} from "../types/ardeck";
import { MappingPresetsJSON } from "../types/settings";
import { settings } from "../tauri/settings";

type UID = string;
type ActionMapWithUID = {
    uid: UID;
} & ActionMap;
type ActionMapWithUIDList = ActionMapWithUID[];
type ActionMapKey = "switchType" | "switchId" | "pluginId" | "actionId";

export default function ActionMappingForm(props: {
    actionMapPresets?: ActionMapPreset[];
    onSubmit: (e: ActionMapPreset) => void;
}) {
    const isInit = useRef(false);
    const [item, setItem] = useState<ActionMapWithUIDList>([]);

    const [mappingPresets, setMappingPresets] = useState<MappingPresetsJSON>([]);

    useEffect(() => {
        if (!isInit.current) {
            isInit.current = true;

            const aaa = async () => {
                const setting: any = await settings.getMappingPresets();
                setMappingPresets(setting);
            };

            aaa();
        }
    }, []);

    return (
        <div>
            <select
                className="rounded-md bg-bg-quaternary text-text-primary px-4 py-2 w-full"
                
            >
                <option value="">[new preset]</option>
                {mappingPresets.map((a) => {
                    return (
                        <option value={a.presetId}>
                            {a.presetName}
                        </option>
                    );
                })}
            </select>
        </div>
    );
}
