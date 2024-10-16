import { Component, FunctionComponent, ReactNode, RefCallback, useEffect, useRef, useState } from "react"
import { makeUid, randomStr } from "../util/props";
import { ActionMap, ActionMapConfig, ActionMapPreset, SwitchType } from "../types/ardeck";

type UID = string;
type ActionMapWithUID = {
    uid: UID
} & ActionMap
type ActionMapWithUIDList = ActionMapWithUID[];
type ActionMapKey = "switchType" | "switchId" | "pluginId" | "actionId";


export default function ActionMappingForm(props: {
    actionMapPreset?: ActionMapPreset,
    onSubmit: (e: ActionMapPreset) => void
}) {
    const isInit = useRef(false);
    const [item, setItem] = useState<ActionMapWithUIDList>([]);

    const addItem = (_item: ActionMap): UID => {
        let uid: UID = makeUid();

        setItem(list => {
            let find = list.findIndex(e => e.uid == uid);
            if (find != -1) {
                // list.rev
            }

            return [...list, { uid, ..._item }];
        })

        console.log("ActionMappingForm.item: ", item);

        return uid;
    };

    const removeItem = (_key: UID) => {
        setItem(e => e.filter(f => f.uid != _key));
    };

    const editItem = (_key: UID, editcallback: (e: ActionMapWithUID) => ActionMapWithUID) => {
        setItem(e => {
            let index = e.findIndex(f => f.uid == _key);
            if (index == -1) {
                console.log(`Not found: uid-${_key}`);
                return e;
            }
            let data = item[index];
            if (data) {
                let edit = editcallback(data);
                return [...e.filter(n => n.uid != _key), edit];
            } else {
                return e;
            }
        })
        console.log("ActionMappingForm.item: ", item);
    };

    // console.log("ActionMappingForm.item: ", item);

    const onSubmitHandler = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const target: HTMLFormElement = e.currentTarget;
        const switchTypeVal = target.switchtype.value;
        const switchIdVal = target.switchid.value;
        const pluginIdVal = target.pluginid.value;
        const actionIdVal = target.actionid.value;
        console.log(
            "<submit>\n",
            `\tswitchTypeVal: ${switchTypeVal}\n`,
            `\tswitchIdVal: ${switchIdVal}\n`,
            `\tpluginIdVal: ${pluginIdVal}\n`,
            `\tactionIdVal: ${actionIdVal}\n`,
        );
        if (!(switchTypeVal && switchIdVal && pluginIdVal && actionIdVal)) {
            return;
        }
        let switchType;
        switch (Number(switchTypeVal)) {
            case SwitchType.Digital:
                switchType = SwitchType.Digital;
                break;
            case SwitchType.Analog:
                switchType = SwitchType.Analog;
                break;
            default:
                console.error("fawef");
                return
        }
        const map: ActionMap = {
            switchType,
            switchId: Number(switchIdVal),
            pluginId: pluginIdVal,
            actionId: actionIdVal,
        }
        addItem(map);
        console.log("OK");
        target.reset();
    };

    const NewOptionForm: FunctionComponent<{ children: ReactNode, isNew: boolean }> = (props) => {
        if (props.isNew) {
            return (
                <form onSubmit={(e) => { onSubmitHandler(e) }}>
                    {props.children}
                </form>
            );
        } else {
            return (
                <>{props.children}</>
            );
        }
    }


    const form = (action?: ActionMapWithUID) => {

        const isNew = action ? false : true; // actionがなければnew
        console.log("isNew", isNew, action?.actionId);

        let buttonValue = isNew ? "+" : "-";

        const temporaryUid = `new-${Date.now().toString(16)}`;

        // TODO: 簡潔にする
        let isDigital = false;
        if (action?.switchType == 0) {
            isDigital = true;
        } else if (action?.switchType == 1) {
            isDigital = false;
        }
        return (
            <NewOptionForm isNew={isNew}>
                <div className="flex gap-1 w-full">
                    <select
                        id={`action-map-option-switch-type-${isNew ? temporaryUid : action?.uid}`}
                        name="switchtype"
                        value={action?.switchType}
                        className="rounded-l-md bg-bg-quaternary text-text-primary px-4 py-2"
                        onChange={(e) => {
                            if (action) {
                                editItem(action?.uid, action => {
                                    // action.switchType = ;
                                    switch (Number(e.target.value)) {
                                        case SwitchType.Digital:
                                            action.switchType = SwitchType.Digital;
                                            break;
                                        case SwitchType.Analog:
                                            action.switchType = SwitchType.Analog;
                                            break;
                                        default:
                                            break;
                                    }

                                    // console.log("onChange.switchType", Number(e.target.value));

                                    return action;
                                });
                            }
                        }}
                    >
                        <option value={0}>Digital</option>
                        <option value={1}>Analog</option>
                    </select>
                    <input
                        id={`action-map-option-switch-id-${isNew ? temporaryUid : action?.uid}`}
                        type="number"
                        name="switchid"
                        placeholder="switchId"
                        value={action?.switchId}
                        min={0}
                        className="bg-bg-quaternary text-text-primary px-4 py-2 w-32"
                        onChange={(e) => {
                            if (action) {
                                editItem(action?.uid, (oldItem) => {
                                    oldItem.switchId = Number(e.target.value)

                                    return oldItem;
                                });
                            }
                        }}
                    />
                    <input
                        id={`action-map-option-plugin-id-${isNew ? temporaryUid : action?.uid}`}
                        type="text"
                        name="pluginid"
                        placeholder="pluginId"
                        value={action?.pluginId}
                        onChange={(e) => {
                            if (action) {
                                editItem(action?.uid, (oldItem) => {
                                    oldItem.pluginId = e.target.value;

                                    return oldItem;
                                })
                            }
                        }}
                        className="bg-bg-quaternary text-text-primary px-4 py-2 w-full flex-1"
                    />
                    <input
                        id={`action-map-option-action-id-${isNew ? temporaryUid : action?.uid}`}
                        type="text"
                        name="actionid"
                        placeholder="actionId"
                        value={action?.actionId!}
                        onChange={(e) => {
                            if (action) {
                                editItem(action?.uid, (oldItem) => {
                                    oldItem.actionId = e.target.value;

                                    return oldItem;
                                })
                            }
                        }}
                        className="bg-bg-quaternary text-text-primary px-4 py-2 w-full flex-1"
                    />
                    <input
                        type={isNew ? "submit" : "button"}
                        id={`action-map-${isNew ? "add" : "remove"}-action-map-${isNew ? temporaryUid : action?.uid}`}
                        onClick={() => {
                            if (!isNew) {
                                removeItem(action!.uid);
                            }
                        }}
                        value={buttonValue}
                        className="rounded-r-md bg-bg-quaternary text-text-primary px-4 py-2 cursor-pointer"
                    />
                </div>
            </NewOptionForm>
        )
    };

    useEffect(() => {
        if (!isInit.current) {
            isInit.current = true;
        }
    }, []);

    const onSave = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const target: HTMLFormElement = e.currentTarget;
        const presetIdVal = target.presetid.value;
        const presetNameVal = target.presetname.value;

        if (!(presetIdVal && presetNameVal)) {
            return;
        }

        const actionMapList: ActionMap[] = item.map(e => {
            const actionMap: ActionMap  = {
                actionId: e.actionId,
                pluginId: e.pluginId,
                switchId: e.switchId,
                switchType: e.switchType
            }

            return actionMap;
        })

        console.log(actionMapList);
        const actionMapPreset: ActionMapPreset = {
            presetId: presetIdVal,
            presetName: presetNameVal,
            mapping: actionMapList
        }

        props.onSubmit(actionMapPreset);
    }

    return (
        <div className="w-full">
            <div className="flex flex-col gap-2 w-full">
                <div className="flex flex-col gap-1 w-full">
                    {Array.from(item).map((e, i) => {
                        return (<div key={`${e.uid}-container`}>{form(e)}</div>)
                    })}
                </div>
                <div className="w-full">
                    {form()}
                </div>
                <form onSubmit={(e) => { onSave(e) }}>
                    <div className="flex gap-1 mt-2">
                        <input
                            type="text"
                            name="presetid"
                            placeholder="preset id"
                            className="rounded-l-md bg-bg-quaternary text-text-primary px-4 py-2"
                        />
                        <input
                            type="text"
                            name="presetname"
                            placeholder="preset name"
                            className="bg-bg-quaternary text-text-primary px-4 py-2"
                        />
                        <input
                            type="submit"
                            name="savepreset"
                            value="save"
                            className="rounded-r-md bg-bg-quaternary text-text-primary px-4 py-2 w-full cursor-pointer"
                        />
                    </div>
                </form>
            </div>
        </div>
    );

}
