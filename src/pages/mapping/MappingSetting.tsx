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
import { VscArrowLeft, VscTrash } from "react-icons/vsc";
import { useEffect, useState } from "react";
import { defaultMappingPreset, MappingPreset } from "../../lib/settings";
import { invoke } from "../../tauri/invoke";
import LoadingScreen from "../_component/loading/legacy";
import Input from "../_component/form/Input";
import Button from "../_component/Button";
import Select from "../_component/form/Select";
import { ActionMap, defaultActionMap, SwitchType } from "../../lib/ardeck";
import { cloneDeep } from "lodash";
import { ModalWindow, ModalWindowContainer } from "../_component/ModalWindow";
import { PluginActionList, PluginManifestJSON } from "../../lib/plugin";

interface ModalParams {
    presetIndex: number;
    preset: ActionMap;
}

interface PluginActions {
    manifest: PluginManifestJSON;
    actions: PluginActionList;
}

export default function MappingSetting() {
    const { mapping_id } = useParams();
    const [mappingPreset, setMappingPreset] = useState<MappingPreset | null>();

    // モーダル表示用データ
    const [modalParams, setModalParams] = useState<ModalParams | null>(null);

    // プラグインとそれぞれのアクションの一覧
    const [plugins, setPlugins] = useState<PluginActions[]>([]);

    const closeModal = () => setModalParams(null);

    // 設定を適用して閉じる
    const applyAndCloseModal = () => {
        if (modalParams) {
            setMappingPreset((prev) => {
                if (prev) {
                    const p = cloneDeep(mappingPreset);
                    if (!p) return prev;
                    p.mapping[modalParams.presetIndex] = modalParams.preset;
                    return p;
                }
            });
        }

        closeModal();
    };

    // 選択したプリセットを編集
    const openModal = (param: ModalParams) => {
        setModalParams(param);
    };

    const addNewMap = () => {
        setMappingPreset((prev) => {
            if (prev) {
                return {
                    ...prev,
                    mapping: [...prev.mapping, defaultActionMap],
                };
            }
            return prev;
        });
    };

    const saveMappingPreset = async () => {
        if (mappingPreset) {
            await invoke.settings.mappingPresets.saveMappingPreset(
                mappingPreset,
            );
        }
    };

    useEffect(() => {
        const getMappingPreset = async () => {
            const mapping =
                await invoke.settings.mappingPresets.getMappingPreset(
                    mapping_id!,
                );
            setMappingPreset(mapping ?? defaultMappingPreset);
        };

        if (mapping_id === "new") {
            setMappingPreset(defaultMappingPreset);
            addNewMap();
        } else {
            getMappingPreset();
        }
    }, [mapping_id]);

    // プラグイン一覧を取得
    useEffect(() => {
        const pluginActions: PluginActions[] = [];

        const getPluginManifestList = async () => {
            const list = await invoke.plugin.getPluginManifests();
        };

        const getPluginActions = async (plugin_id: string) => {
            const list = await invoke.plugin.getPluginActions(plugin_id!);
        };
        const initPluginActions = async () => {
            const list = await invoke.plugin.getPluginManifests();
            for (const manifest of list) {
                const actions = await invoke.plugin.getPluginActions(
                    manifest.id,
                );
                pluginActions.push({ manifest, actions });
            }
            setPlugins(pluginActions);
        };
        initPluginActions();
    }, []);

    return (
        <div className="flex flex-col gap-4 px-8 py-4">
            <BackToPrev>
                <div className="flex items-center gap-2">
                    <VscArrowLeft />
                    Back to list
                </div>
            </BackToPrev>
            {/* <div>Device Setting: {device_id}</div> */}
            {/* <pre>{JSON.stringify(mappingPreset, null, "\t")}</pre> */}
            <h1 className="text-2xl font-bold">
                Mapping Setting: {mappingPreset?.presetName || "New Mapping"}
                {/* Device Setting: {deviceSetting?.deviceName || "New Device"} */}
            </h1>
            <div className="flex flex-col gap-2">
                <label>
                    <span>Preset Name</span>
                    <Input
                        value={mappingPreset?.presetName ?? ""}
                        onChange={(e) => {
                            setMappingPreset({
                                ...mappingPreset!,
                                presetName: e.target.value,
                            });
                        }}
                    />
                </label>
                <div className="flex flex-col gap-2 py-4">
                    {mappingPreset?.mapping.map((mapping, i) => {
                        return (
                            <div key={i} className="flex w-full gap-2">
                                <label>
                                    <span>Switch Type</span>
                                    <Select
                                        className="py-2"
                                        value={mapping.switchType ?? ""}
                                        onChange={(e) => {
                                            setMappingPreset((prev) => {
                                                if (prev) {
                                                    const newValue = e.target
                                                        .value as SwitchType;
                                                    const m = cloneDeep(prev);
                                                    m.mapping[i] = {
                                                        ...m.mapping[i],
                                                        switchType: newValue,
                                                    };
                                                    return m;
                                                }
                                            });
                                        }}
                                    >
                                        <option value={SwitchType.Digital}>
                                            {SwitchType.Digital}
                                        </option>
                                        <option value={SwitchType.Analog}>
                                            {SwitchType.Analog}
                                        </option>
                                    </Select>
                                </label>
                                <label className="w-24">
                                    <span>Switch ID</span>
                                    <Input
                                        className="py-2"
                                        type="number"
                                        value={mapping.switchId}
                                        onChange={(e) => {
                                            setMappingPreset((prev) => {
                                                if (prev) {
                                                    const newValue = Number(
                                                        e.target.value,
                                                    );
                                                    const m = cloneDeep(prev);
                                                    m.mapping[i] = {
                                                        ...m.mapping[i],
                                                        switchId: newValue,
                                                    };
                                                    return m;
                                                }
                                            });
                                        }}
                                        min={0}
                                    />
                                </label>
                                <div className="flex flex-1 flex-col">
                                    <div className="flex gap-2 *:flex-1">
                                        <span>Plugin ID</span>
                                        <span>Action ID</span>
                                    </div>
                                    <Button
                                        className="bg-bg-tertiary flex cursor-pointer gap-2 rounded-md px-1 *:pointer-events-none *:flex-1 hover:opacity-50"
                                        onClick={() =>
                                            openModal({
                                                presetIndex: i,
                                                preset: mapping,
                                            })
                                        }
                                    >
                                        <Input
                                            className="rounded-sm"
                                            type="text"
                                            value={mapping.pluginId}
                                            readOnly
                                        />
                                        <Input
                                            className="rounded-sm"
                                            type="text"
                                            value={mapping.actionId}
                                            readOnly
                                        />
                                    </Button>
                                </div>

                                <Button
                                    className="hover:bg-accent-negative mt-6 h-auto w-auto px-2 py-2"
                                    onClick={() => {
                                        setMappingPreset((prev) => {
                                            if (prev) {
                                                const m = cloneDeep(prev);
                                                m.mapping.splice(i, 1);
                                                return m;
                                            }
                                        });
                                    }}
                                >
                                    <VscTrash />
                                </Button>
                            </div>
                        );
                    })}
                    <Button onClick={addNewMap} className="mt-4">
                        Add Preset
                    </Button>
                </div>
                <Button className="mt-4" onClick={saveMappingPreset}>
                    Save
                </Button>
            </div>

            {/* plugin idとaction idを、ロードされているプラグインから選ぶか直接入力するためのモーダル */}
            <ModalWindow isOpen={modalParams !== null}>
                {modalParams && (
                    <ModalWindowContainer className="flex flex-col gap-4">
                        <div className="flex w-full gap-2">
                            <Button
                                className="w-full flex-1"
                                onClick={applyAndCloseModal}
                            >
                                apply
                            </Button>
                            <Button
                                className="hover:bg-accent-negative w-16"
                                onClick={closeModal}
                            >
                                cancel
                            </Button>
                        </div>
                        <div className="flex w-full gap-2">
                            <label className="flex-1">
                                <span>Plugin ID</span>
                                <Input
                                    type="text"
                                    value={modalParams.preset.pluginId}
                                    onChange={(e) => {
                                        setModalParams((prev) => {
                                            if (prev) {
                                                const m = cloneDeep(prev);
                                                m.preset = {
                                                    ...m.preset,
                                                    pluginId: e.target.value,
                                                };
                                                return m;
                                            }

                                            return prev;
                                        });
                                    }}
                                />
                            </label>
                            <label className="flex-1">
                                <span>Action ID</span>
                                <Input
                                    type="text"
                                    className="flex-1"
                                    value={modalParams.preset.actionId}
                                    onChange={(e) => {
                                        setModalParams((prev) => {
                                            if (prev) {
                                                const m = cloneDeep(prev);
                                                m.preset = {
                                                    ...m.preset,
                                                    actionId: e.target.value,
                                                };
                                                return m;
                                            }

                                            return prev;
                                        });
                                    }}
                                />
                            </label>
                        </div>
                        <div className="mt-8 flex w-full flex-1 flex-col gap-2 overflow-auto">
                            <span>Action List</span>
                            {plugins.map((plugin) => {
                                return plugin.actions.map((action) => {
                                    return (
                                        <Button
                                            onClick={() =>
                                                setModalParams((prev) => {
                                                    if (prev) {
                                                        const m =
                                                            cloneDeep(prev);
                                                        m.preset = {
                                                            ...m.preset,
                                                            pluginId:
                                                                plugin.manifest
                                                                    .id,
                                                            actionId: action.id,
                                                        };
                                                        return m;
                                                    }
                                                    return prev;
                                                })
                                            }
                                        >
                                            {plugin.manifest.name} : {action.name}
                                        </Button>
                                    );
                                });
                            })}
                        </div>
                    </ModalWindowContainer>
                )}
            </ModalWindow>
        </div>
    );
}
