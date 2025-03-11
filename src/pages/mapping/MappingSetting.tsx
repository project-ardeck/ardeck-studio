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
import { defaultActionMap, SwitchType } from "../../lib/ardeck";
import { cloneDeep } from "lodash";
import { ModalWindowContainer, useModal } from "../_component/ModalWindow";

export default function MappingSetting() {
    const { mapping_id } = useParams();
    const [mappingPreset, setMappingPreset] = useState<MappingPreset | null>();
    // const configModalForPluginIdAndActionId = useModal(
    //     (close) => (
    //         <ModalWindowContainer>
    //             Modal test
    //             <button onClick={close}>Close modal</button>
    //         </ModalWindowContainer>
    //     )
    // );

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

    return (
        <div className="flex flex-col gap-4 px-8 py-4">
            <LoadingScreen isLoading={!mappingPreset} />
            {/* {configModalForPluginIdAndActionId.modal}
            <button onClick={configModalForPluginIdAndActionId.open}>Open modal</button> */}
            <BackToPrev className="flex items-center gap-2">
                <VscArrowLeft />
                Back to list
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
                            <div className="flex w-full gap-2">
                                <label>
                                    <span>Switch Type</span>
                                    <Select
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
                                        // className="w-16"
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
                                <label className="flex-1">
                                    <span>Plugin ID</span>
                                    <Input
                                        type="text"
                                        value={mapping.pluginId}
                                        onChange={(e) => {
                                            setMappingPreset((prev) => {
                                                if (prev) {
                                                    const m = cloneDeep(prev);
                                                    m.mapping[i] = {
                                                        ...m.mapping[i],
                                                        pluginId:
                                                            e.target.value,
                                                    };
                                                    return m;
                                                }
                                            });
                                        }}
                                    />
                                </label>
                                <label className="flex-1">
                                    <span>Action ID</span>
                                    <Input
                                        type="text"
                                        className="flex-1"
                                        value={mapping.actionId}
                                        onChange={(e) => {
                                            setMappingPreset((prev) => {
                                                if (prev) {
                                                    const m = cloneDeep(prev);
                                                    m.mapping[i] = {
                                                        ...m.mapping[i],
                                                        actionId:
                                                            e.target.value,
                                                    };
                                                    return m;
                                                }
                                            });
                                        }}
                                    />
                                </label>

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
        </div>
    );
}
