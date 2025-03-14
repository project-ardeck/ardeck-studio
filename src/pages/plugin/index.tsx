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
import { PluginManifestJSON } from "../../lib/plugin";
import { invoke } from "../../tauri/invoke";
import { Link } from "react-router";

export default function Plugin() {
    const [pluginManifestList, setPluginManifestList] = useState<
        Array<PluginManifestJSON>
    >([]);

    useEffect(() => {
        const getPluginManifestList = async () => {
            const list = await invoke.plugin.getPluginManifests();
            setPluginManifestList(list);
        };
        getPluginManifestList();
    }, []);

    return (
        <div>
            <h1 className="pagetitle mb-4">Plugin</h1>
            <div className="flex flex-col gap-2">
                {pluginManifestList.map(
                    ({ name, id, version, description }) => {
                        return (
                            <Link
                                className="bg-bg-secondary flex justify-between rounded-md px-4 py-2"
                                key={id}
                                to={`/plugin/${id}`}
                            >
                                <div>{name}</div>
                                <div>{version}</div>
                            </Link>
                        );
                    },
                )}
            </div>
        </div>
    );
}
