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
import { invoke } from "../../tauri/invoke";
import { Link } from "react-router";
import { VscAdd } from "react-icons/vsc";

export default function Mapping() {
    const [mappingList, setMappingList] = useState<Array<[string, string]>>([]);

    useEffect(() => {
        const getMappingList = async () => {
            const list = await invoke.settings.mappingPresets.getMappingList();
            setMappingList(list);
        };
        getMappingList();
    }, []);

    return (
        <div>
            <div className="mb-4 flex justify-between">
                <h1 className="pagetitle">Mapping</h1>
                <Link
                    className="bg-bg-secondary rounded-md px-4 py-2 flex gap-1 items-center justify-center hover:bg-bg-tertiary"
                    to={`/mapping/new`}
                >
                    <VscAdd />
                </Link>
            </div>
            <div className="flex flex-col gap-2">
                {mappingList.map(([id, name]) => {
                    return (
                        <Link
                            className="bg-bg-secondary rounded-md px-4 py-2"
                            key={id}
                            to={`/mapping/${id}`}
                        >
                            {name}
                        </Link>
                    );
                })}
            </div>
        </div>
    );
}
