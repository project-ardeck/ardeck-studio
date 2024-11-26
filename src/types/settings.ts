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

import { ActionMap, defaultActionMap } from "./ardeck";

// * mapping_presets
export type MappingPreset = {
    uuid: string;
    presetName: string;

    mapping: ActionMap[];
};

export const defaultMappingPreset: MappingPreset = {
    // uuidを空にした状態で渡すと自動で新規作成扱いになる
    uuid: "",
    presetName: "",
    mapping: [],
};

export type MappingPresetsJSON = MappingPreset[];

// * plugin

// * ardeck

// * ardeck_studio
