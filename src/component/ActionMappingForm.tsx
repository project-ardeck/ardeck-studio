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
    defaultActionMap,
    SwitchType,
} from "../types/ardeck";
import {
    defaultMappingPreset,
    MappingPreset,
    MappingPresetsJSON,
} from "../types/settings";
import { cloneDeep } from "lodash";

type ActionMapKey = "switchType" | "switchId" | "pluginId" | "actionId";
type MappingList = Array<[string, string]>; // [uuid, presetName]

export default function ActionMappingForm(props: {
    // actionMapPresets?: ActionMapPreset[];
    // onSubmit: (e: ActionMapPreset) => void;
}) {
    const isInit = useRef(false);
    const reRender = useRef(0);

    const mappingList = useRef<MappingList>([]);

    const [editTarget, setEditTarget] = useState<string>("");
    const [presetTmp, setPresetTmp] =
        useState<MappingPreset>(defaultMappingPreset);
    const [newMappingTmp, setNewMappingTmp] =
        useState<ActionMap>(defaultActionMap);

    const changeEditTarget = async (uuid: string) => {
        const mappingPreset =
            await invoke.settings.mappingPresets.getMappingPreset(uuid);
        console.log("mappingPreset: ", mappingPreset);

        setEditTarget(uuid);
        setPresetTmp(mappingPreset ?? defaultMappingPreset);
        setNewMappingTmp(defaultActionMap); // reset
    };

    const checkMappingComplete = (map: ActionMap): boolean => {
        return (
            // map.switchType !== SwitchType.Digital ||
            // map.switchId !== 0 ||
            map.pluginId !== "" && map.actionId !== ""
        );
    };

    const checkPresetComplete = (preset: MappingPreset): boolean => {
        return /*preset.uuid !== "" && */ preset.presetName !== "";
    };

    const commitToPresetTmp = (map: ActionMap /* newMappingTmp */) => {
        if (presetTmp) {
            if (!checkMappingComplete(map)) return;

            setPresetTmp((prev) => {
                if (!prev) return prev;
                return {
                    ...prev,
                    mapping: [...prev.mapping, map],
                };
            });

            setNewMappingTmp(defaultActionMap);
        }
    };

    const savePreset = async (preset: MappingPreset /* presetTmp */) => {
        if (presetTmp) {
            if (!checkPresetComplete(preset)) {
                return;
            }

            console.log("applyPreset", preset, checkPresetComplete(preset));
            const savedPreset =
                await invoke.settings.mappingPresets.saveMappingPreset(preset);
            // await

            setEditTarget(savedPreset.uuid);
            setPresetTmp(savedPreset);
            setNewMappingTmp(defaultActionMap); // reset
        }
    };

    // const saveNewPreset = (preset: MappingPreset /*newMappingTmp*/) => {}

    reRender.current += 1;
    console.log(
        `%c[${reRender.current}] render`,
        "color: red; font-weight: bold; font-size: 20px",
        presetTmp,
        newMappingTmp,
    );

    useEffect(() => {
        if (!isInit.current) {
            isInit.current = true;

            const init = async () => {
                mappingList.current =
                    await invoke.settings.mappingPresets.getMappingList();
                console.log("mappingList: ", mappingList.current);
            };

            init();
        }
    }, []);

    const onSubmit = () => {
        // props.onSubmit(presetTmp!);
    };

    return (
        <div className="flex w-full flex-col gap-4">
            <select
                className="w-full rounded-md bg-bg-quaternary px-4 py-2 text-text-primary"
                onChange={(e) => {
                    changeEditTarget(e.target.value);
                    console.log("setPresetTmp", presetTmp);
                }}
                value={editTarget}
            >
                <option selected value="">
                    [new preset]
                </option>
                {mappingList.current.map((a) => {
                    return (
                        <option value={a[0]}>
                            <span>{a[1]}</span>
                        </option>
                    );
                })}
            </select>
            <div className="flex w-full flex-col gap-2">
                {/*
                    設定済みのデータと新しく追加するデータの一時保存を結合する
                    配列の一番最後は必ず未追加の仮データ
                */}
                {presetTmp?.mapping.concat(newMappingTmp).map((a, i) => {
                    const isNew = presetTmp.mapping.length <= i;
                    console.log(
                        `${i}${isNew ? "[new]" : ""}: presetTmp`,
                        a,
                        checkMappingComplete(a),
                    );

                    return (
                        <div className="flex w-full gap-1">
                            <select
                                className="rounded-md bg-bg-quaternary px-4 py-2 text-text-primary"
                                value={
                                    isNew
                                        ? newMappingTmp.switchType
                                        : a.switchType
                                }
                                onChange={(e) => {
                                    const newValue = e.target
                                        .value as SwitchType;

                                    setPresetTmp((prev) => {
                                        if (!prev) return prev;
                                        if (isNew) {
                                            setNewMappingTmp((prev) => {
                                                return {
                                                    ...prev,
                                                    switchType: newValue,
                                                };
                                            });

                                            return prev;
                                        } else {
                                            const mapping = prev.mapping ?? [];
                                            mapping[i] = {
                                                ...mapping[i],
                                                switchType: newValue,
                                            };
                                            return { ...prev, mapping };
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
                            </select>
                            <input
                                type="number"
                                min={0}
                                placeholder="switch id"
                                className="w-24 rounded-md bg-bg-quaternary px-4 py-2 text-text-primary"
                                value={
                                    isNew ? newMappingTmp.switchId : a.switchId
                                }
                                onChange={(e) => {
                                    const newValue = parseInt(e.target.value);
                                    setPresetTmp((prev) => {
                                        if (!prev) return prev;
                                        if (isNew) {
                                            setNewMappingTmp((prev) => {
                                                return {
                                                    ...prev,
                                                    switchId: newValue,
                                                };
                                            });

                                            return prev;
                                        } else {
                                            const mapping = prev.mapping ?? [];
                                            mapping[i] = {
                                                ...mapping[i],
                                                switchId: newValue,
                                            };
                                            return { ...prev, mapping };
                                        }
                                    });
                                }}
                            />
                            <input
                                type="text"
                                placeholder="plugin id"
                                className="w-full rounded-md bg-bg-quaternary px-4 py-2 text-text-primary"
                                value={a.pluginId}
                                onChange={(e) => {
                                    const newValue = e.target.value;
                                    setPresetTmp((prev) => {
                                        if (!prev) return prev;
                                        if (isNew) {
                                            setNewMappingTmp((prev) => {
                                                return {
                                                    ...prev,
                                                    pluginId: newValue,
                                                };
                                            });

                                            return prev;
                                        } else {
                                            const mapping = prev.mapping ?? [];
                                            mapping[i] = {
                                                ...mapping[i],
                                                pluginId: newValue,
                                            };
                                            return { ...prev, mapping };
                                        }
                                    });
                                }}
                            />
                            <input
                                type="text"
                                placeholder="action id"
                                className="w-full rounded-md bg-bg-quaternary px-4 py-2 text-text-primary"
                                value={a.actionId}
                                onChange={(e) => {
                                    const newValue = e.target.value;
                                    setPresetTmp((prev) => {
                                        if (!prev) return prev;
                                        if (isNew) {
                                            setNewMappingTmp((prev) => {
                                                return {
                                                    ...prev,
                                                    actionId: newValue,
                                                };
                                            });

                                            return prev;
                                        } else {
                                            const mapping = prev.mapping ?? [];
                                            mapping[i] = {
                                                ...mapping[i],
                                                actionId: newValue,
                                            };
                                            return { ...prev, mapping };
                                        }
                                    });
                                }}
                            />
                            <input
                                type="button"
                                disabled={!checkMappingComplete(a)}
                                onClick={() => {
                                    if (presetTmp) {
                                        if (isNew) {
                                            setPresetTmp((prev) => {
                                                commitToPresetTmp(
                                                    newMappingTmp,
                                                );

                                                return prev;
                                            });
                                        } else {
                                            setPresetTmp((prev) => {
                                                if (!prev) return prev;
                                                return {
                                                    ...prev,
                                                    mapping:
                                                        prev.mapping.filter(
                                                            (a, b) => b != i,
                                                        ),
                                                };
                                            });
                                        }
                                    }
                                }}
                                value={isNew ? "+" : "-"}
                                className="cursor-pointer rounded-md bg-bg-quaternary px-4 py-2 text-text-primary disabled:opacity-50"
                            />
                        </div>
                    );
                })}
            </div>
            <div className="flex w-full gap-1">
                <input
                    type="text"
                    className="w-full rounded-md bg-bg-quaternary px-4 py-2 text-text-primary"
                    placeholder="preset name"
                    value={presetTmp ? presetTmp.presetName : ""}
                    onChange={(e) => {
                        const newValue = e.target.value;
                        setPresetTmp((prev) => {
                            if (!prev) return prev;
                            return { ...prev, presetName: newValue };
                        });
                    }}
                />
            </div>
            <div>
                <input
                    type="button"
                    className="w-full cursor-pointer rounded-md bg-bg-quaternary px-4 py-2 text-text-primary"
                    value="button"
                    onClick={() => {
                        // onSubmit(presetTmp!);
                        if (presetTmp) {
                            savePreset(presetTmp);
                        }
                    }}
                />
            </div>

            <div>
                <pre>{JSON.stringify(presetTmp, null, 2)}</pre>
            </div>
        </div>
    );
}
