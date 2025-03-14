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

import {
    Component,
    FunctionComponent,
    ReactNode,
    RefCallback,
    useEffect,
    useRef,
    useState,
} from "react";
import { invoke } from "../tauri/invoke";
import { ActionMap, defaultActionMap, SwitchType } from "../lib/ardeck";
import {
    defaultMappingPreset,
    MappingPreset,
    MappingPresetsJSON,
} from "../lib/settings";
import { cloneDeep, get } from "lodash";

type ActionMapKey = "switchType" | "switchId" | "pluginId" | "actionId";
type MappingList = Array<[string, string]>; // [uuid, presetName]

export default function ActionMappingForm() {
    const isInit = useRef(false);

    /** ファイルから取得されたマッピングの一覧 */
    const [mappingList, setMappingList] = useState<MappingList>([]);

    /** 編集するプリセットのUUID */
    const [editTarget, setEditTarget] = useState<string>("");

    /** 編集中のプリセットの一時保存 */
    const [presetTmp, setPresetTmp] =
        useState<MappingPreset>(defaultMappingPreset);

    /** 追加中のマッピングの一時保存 */
    const [newMappingTmp, setNewMappingTmp] =
        useState<ActionMap>(defaultActionMap);

    const getMappingList = async () => {
        return await invoke.settings.mappingPresets.getMappingList();
    };

    /** 選択されたプリセットのみを取り出し、編集用の変数に移す */
    const changeEditTarget = async (uuid: string) => {
        const mappingPreset =
            await invoke.settings.mappingPresets.getMappingPreset(uuid);
        // console.log("mappingPreset: ", mappingPreset);

        setEditTarget(uuid);
        setPresetTmp(mappingPreset ?? defaultMappingPreset);
        setNewMappingTmp(defaultActionMap); // reset
    };

    /** マッピングの項目に空白がなければtrue */
    const checkMappingComplete = (map: ActionMap): boolean => {
        return (
            // map.switchType !== SwitchType.Digital ||
            // map.switchId !== 0 ||
            map.pluginId !== "" && map.actionId !== ""
        );
    };

    /** プリセット名が空白でなければtrue */
    const checkPresetComplete = (preset: MappingPreset): boolean => {
        return /*preset.uuid !== "" && */ preset.presetName !== "";
    };

    // debug
    // useEffect(() => {
    //     console.log(`%c[Rerender] presetTmp`, "color: red");
    // }, [presetTmp]);

    /** プリセットにマッピングを追加 */
    const addNewMapToPresetTmp = (map: ActionMap /* newMappingTmp */) => {
        if (presetTmp) {
            if (!checkMappingComplete(map)) return;

            setPresetTmp((prev) => {
                if (!prev) return prev;
                return {
                    ...prev,
                    mapping: [...prev.mapping, map],
                };
            });

            // 入力欄をクリアする
            setNewMappingTmp(defaultActionMap);

            console.log("newMappingTmp: ", newMappingTmp);
        } else {
            console.log("presetTmp is null");
        }
    };

    /** 変更されたプリセットを保存 */
    const savePreset = async (preset: MappingPreset /* presetTmp */) => {
        if (presetTmp) {
            if (!checkPresetComplete(preset)) {
                return;
            }

            // console.log("applyPreset", preset, checkPresetComplete(preset));
            const savedPreset =
                await invoke.settings.mappingPresets.saveMappingPreset(preset);
            // await

            setEditTarget(savedPreset.uuid);
            setPresetTmp(savedPreset);
            setNewMappingTmp(defaultActionMap); // reset
            setMappingList(await getMappingList()); // refresh
        }
    };

    // const saveNewPreset = (preset: MappingPreset /*newMappingTmp*/) => {}

    // console.log(
    //     `%c[${reRender.current}] render`,
    //     "color: red; font-weight: bold; font-size: 20px",
    //     presetTmp,
    //     newMappingTmp,
    // );

    useEffect(() => {
        if (!isInit.current) {
            isInit.current = true;

            // ファイルからマッピング一覧を取得
            const init = async () => {
                const mappingList = await getMappingList();
                setMappingList(mappingList);
                // console.log("mappingList: ", mappingList);
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
                    // console.log("setPresetTmp", presetTmp);
                }}
                value={editTarget}
            >
                <option value="">[new preset]</option>
                {mappingList.map((a) => {
                    return (
                        <option key={a[0]} value={a[0]}>
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
                    // console.log(
                    //     `${i}${isNew ? "[new]" : ""}: presetTmp`,
                    //     a,
                    //     checkMappingComplete(a),
                    // );

                    return (
                        <div key={i} className="flex w-full gap-1">
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
                                    // console.log(
                                    //     "--------------------------------------------",
                                    // );
                                    // console.log("onClick: add/remove", a);
                                    if (presetTmp) {
                                        // console.log("if (presetTmp)");
                                        if (isNew) {
                                            // console.log("if (isNew)");
                                            addNewMapToPresetTmp(newMappingTmp);
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
