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

import { useParams } from "react-router";
import LoadingScreen from "../_component/loading/legacy";
import BackToPrev from "../_component/back_to_prev";
import { VscArrowLeft } from "react-icons/vsc";
import { useEffect, useState } from "react";
import { PluginActionList } from "../../lib/plugin";
import { invoke } from "../../tauri/invoke";

export default function PluginActions() {
    const { plugin_id } = useParams();

    const [pluginActions, setPluginActions] = useState<PluginActionList>([]);

    useEffect(() => {
        const getPluginActions = async () => {
            const list = await invoke.plugin.getPluginActions(plugin_id!);
            setPluginActions(list);
        };
        getPluginActions();
    }, []);

    return (
        <div className="flex flex-col gap-4 px-8 py-4">
            {/* <LoadingScreen isLoading={!deviceSetting} /> */}
            <BackToPrev className="flex items-center gap-2">
                <VscArrowLeft />
                Back to list
            </BackToPrev>
            {/* <div>Device Setting: {device_id}</div> */}
            <h1 className="text-2xl font-bold">Actions</h1>
            <div className="flex flex-col gap-2">
                {pluginActions.map((action) => {
                    return (
                        <div
                            className="bg-bg-secondary flex flex-col items-center gap-2 rounded-md px-4 py-2"
                            key={action.id}
                        >
                            <div className="flex w-full items-center justify-between">
                                <div>{action.name}</div>
                                <div className="text-sm">
                                    {action.description}
                                </div>
                            </div>
                            {/* <div className="flex w-full items-center justify-center">
                                {action.id}
                            </div> */}
                        </div>
                    );
                })}
            </div>
        </div>
    );
}
