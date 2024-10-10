import { Component, RefCallback, useEffect, useRef, useState } from "react"
import { makeUid, randomStr } from "../util/props";
import { ActionMap, ActionMapConfig, SwitchType } from "../types/ardeck";

type UID = string;
type ActionMapWithUID = {
    uid: UID
} & ActionMap
type ActionMapWithUIDList = ActionMapWithUID[];
type ActionMapKey = "switchType" | "switchId" | "pluginId" | "actionId";


export default function ActionMappingForm(props: {
    onChange?: (e: ActionMapWithUIDList) => {}
    onSubmit?: (e: ActionMapWithUIDList) => {}
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
            <div className="flex gap-1 w-full">
                <select
                    id={`action-map-option-switch-type-${isNew ? temporaryUid : action?.uid}`}
                    value={action?.switchType}
                    className="rounded-sm bg-bg-quaternary text-text-primary px-4 py-2"
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
                    placeholder="switchId"
                    value={action?.switchId}
                    min={0}
                    className="rounded-sm bg-bg-quaternary text-text-primary px-4 py-2 w-32"
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
                    className="rounded-sm bg-bg-quaternary text-text-primary px-4 py-2 w-full flex-1"
                />
                <input
                    id={`action-map-option-action-id-${isNew ? temporaryUid : action?.uid}`}
                    type="text"
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
                    className="rounded-sm bg-bg-quaternary text-text-primary px-4 py-2 w-full flex-1"
                />
                <button
                    id={`action-map-${isNew ? "add" : "remove"}-action-map-${isNew ? temporaryUid : action?.uid}`}
                    onClick={() => {
                        if (isNew) {
                            const switchTypeElm = document.getElementById(`action-map-option-switch-type-${temporaryUid}`) as HTMLSelectElement;
                            const switchIdElm = document.getElementById(`action-map-option-switch-id-${temporaryUid}`) as HTMLInputElement;
                            const pluginIdElm = document.getElementById(`action-map-option-plugin-id-${temporaryUid}`) as HTMLInputElement;
                            const actionIdElm = document.getElementById(`action-map-option-action-id-${temporaryUid}`) as HTMLInputElement;

                            if (!(switchTypeElm.value && switchIdElm.value && pluginIdElm.value && actionIdElm.value)) {
                                return;
                            }

                            let switchType;
                            switch (Number(switchTypeElm.value)) {
                                case SwitchType.Digital:
                                    switchType = SwitchType.Digital;
                                    break;
                                case SwitchType.Analog:
                                    switchType = SwitchType.Analog;
                                    break;
                                default:
                                    return
                            }
                            const map: ActionMap = {
                                switchType,
                                switchId: Number(switchIdElm.value),
                                pluginId: pluginIdElm.value,
                                actionId: actionIdElm.value,
                            }

                            addItem(map);

                            switchTypeElm.value = "0";
                            switchIdElm.value = "";
                            pluginIdElm.value = "";
                            actionIdElm.value = "";
                        } else {
                            removeItem(action!.uid);
                        }
                    }}
                    className="rounded-sm bg-bg-quaternary text-text-primary px-4 py-2">
                    {buttonValue}
                </button>
            </div>
        )
    };

    useEffect(() => {
        if (!isInit.current) {
            isInit.current = true;
        }
    }, []);

    const onSubmitHandler = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
    };

    return (
        <div className="w-full">

            {/* <form onSubmit={(e) => {onSubmitHandler(e)}}> */}
            <div className="flex flex-col gap-2 w-full">
                <div className="flex flex-col gap-1 w-full">
                    {Array.from(item).map((e, i) => {
                        return (<div key={`${e.uid}-container`}>{form(e)}</div>)
                    })}
                </div>
                <div className="w-full">
                    {form()}
                </div>
                <div className="mt-2">
                    <button className="rounded-sm bg-bg-quaternary text-text-primary px-4 py-2 w-full">
                        save
                    </button>
                </div>
            </div>
            {/* </form> */}
        </div>
    );

}
