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
import BackToPrev from "../_component/back_to_prev";
import { VscArrowLeft } from "react-icons/vsc";
import { useEffect, useState } from "react";
import { defaultMappingPreset, MappingPreset } from "../../lib/settings";
import { invoke } from "../../tauri/invoke";
import LoadingScreen from "../_component/loading/legacy";

export default function MappingSetting() {
    const { mapping_id } = useParams();
    const [mappingPreset, setMappingPreset] =
        useState<MappingPreset | null>();

    useEffect(() => {
        const getMappingPreset = async () => {
            const mapping =
                await invoke.settings.mappingPresets.getMappingPreset(
                    mapping_id!,
                );
            setMappingPreset(mapping ?? defaultMappingPreset);
        };
        getMappingPreset();
    }, [mapping_id]);

    return (
        <div className="flex flex-col gap-4 px-8 py-4">
            <LoadingScreen isLoading={!mappingPreset} />
            <BackToPrev className="flex items-center gap-2">
                <VscArrowLeft />
                Back to list
            </BackToPrev>
            {/* <div>Device Setting: {device_id}</div> */}
            <h1 className="text-2xl font-bold">
                Mapping Setting: {mappingPreset?.presetName || "New Mapping"}
                {/* Device Setting: {deviceSetting?.deviceName || "New Device"} */}
            </h1>
        </div>
    );
}
