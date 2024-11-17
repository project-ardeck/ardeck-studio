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

import { invoke } from "./invoke";
import { MappingPresetsJSON } from "../types/settings";

function extractFromEnum(e: any): any {
    return Object.values(e)[0];
}

export const settings = {
    async getMappingPresets(): Promise<MappingPresetsJSON> {
        const mp: { [key: string]: any } =
            await invoke.settings.getSetting("mapping_presets");

        return extractFromEnum(mp) as MappingPresetsJSON;
    },
};
