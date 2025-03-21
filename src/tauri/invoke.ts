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

import { invoke as tauriInvoke } from "@tauri-apps/api";
import { ArdeckProfileConfigItem, SerialPortInfo } from "../lib/ardeck";
import { MappingPreset } from "../lib/settings";
import { PluginActionList, PluginManifestJSON } from "../lib/plugin";

// TODO: error handling
export const invoke = {
    settings: {
        async getSetting(settingId: string): Promise<any> {
            return await tauriInvoke("plugin:settings|get_setting", {
                configId: settingId,
            });
        },
        async getSettingList(): Promise<Array<string>> {
            return await tauriInvoke("plugin:settings|get_setting_list");
        },

        mappingPresets: {
            async getMappingList(): Promise<Array<[string, string]>> {
                return await tauriInvoke("plugin:settings|get_mapping_list");
            },

            async getMappingPreset(uuid: string): Promise<MappingPreset> {
                return await tauriInvoke("plugin:settings|get_mapping_preset", {
                    uuid,
                });
            },

            async saveMappingPreset(
                mappingPreset: MappingPreset,
            ): Promise<MappingPreset> {
                return await tauriInvoke(
                    "plugin:settings|save_mapping_preset",
                    {
                        mappingPreset,
                    },
                );
            },
        },
        ardeckPresets: {
            async getArdeckProfileList(): Promise<[string, string][]> {
                return await tauriInvoke(
                    "plugin:settings|get_ardeck_profile_list",
                );
            },

            async getArdeckProfile(
                deviceId: string,
            ): Promise<ArdeckProfileConfigItem> {
                return await tauriInvoke("plugin:settings|get_ardeck_profile", {
                    deviceId,
                });
            },

            async saveArdeckProfile(
                ardeckProfile: ArdeckProfileConfigItem,
            ): Promise<ArdeckProfileConfigItem> {
                return await tauriInvoke(
                    "plugin:settings|save_ardeck_profile",
                    { profile: ardeckProfile },
                );
            },
        },
    },
    plugin: {
        async getPluginManifests(): Promise<Array<PluginManifestJSON>> {
            return await tauriInvoke("plugin:ardeck-plugin|get_plugin_manifests");
        },
        async getPluginActions(pluginId: string): Promise<PluginActionList> {
            return await tauriInvoke("plugin:ardeck-plugin|get_plugin_actions", {
                pluginId,
            })
        }
    },
    ardeck: {
        async openPort(portName: string, baudRate: number): Promise<undefined> {
            return await tauriInvoke("plugin:ardeck|open_port", {
                portName,
                baudRate,
            });
        },
        async closePort(portName: string): Promise<undefined> {
            return await tauriInvoke("plugin:ardeck|close_port", { portName });
        },
        async getConnectingSerials(): Promise<Array<string>> {
            return await tauriInvoke("plugin:ardeck|get_connecting_serials");
        },
        async getPorts(): Promise<Array<[string, SerialPortInfo]>> {
            return await tauriInvoke("plugin:ardeck|get_ports");
        },
    },
    openWindow: {
        async about() {
            return await tauriInvoke("open_about");
        },
    },
};
