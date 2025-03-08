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

import { Link, Router, useLocation, useParams } from "react-router";
import BackToPrev from "../_component/back_to_prev";
import { VscArrowLeft, VscChromeClose } from "react-icons/vsc";
import { useCallback, useEffect, useRef, useState } from "react";
import { ArdeckProfileConfigItem, BaudRateList } from "../../lib/ardeck";
import { invoke } from "../../tauri/invoke";
import Input from "../_component/form/Input";
import Select from "../_component/form/Select";
import Button from "../_component/Button";
import { MappingPresetsJSON } from "../../lib/settings";
import LoadingScreen from "../_component/loading/legacy";

export default function DeviceSetting() {
    let { device_id } = useParams();
    const [deviceSetting, setDeviceSetting] =
        useState<ArdeckProfileConfigItem>();
    const [mappingPresetList, setMappingPresetList] =
        useState<Array<[string, string]>>();

    // const location = useLocation();
    // const prevLocation = useRef(location);

    useEffect(() => {
        const getDeviceSetting = async () => {
            if (device_id) {
                const deviceSetting =
                    await invoke.settings.ardeckPresets.getArdeckProfile(
                        device_id,
                    );
                setDeviceSetting(deviceSetting);
            }
        };
        getDeviceSetting();

        const getMappingPreset = async () => {
            const mappingPresetList =
                await invoke.settings.mappingPresets.getMappingList();
            setMappingPresetList(mappingPresetList);
        };
        getMappingPreset();

        // document.addEventListener("contextmenu", function (event) {
        //     event.preventDefault();
        // });
    }, []);

    const saveDeviceSettingHandler = async () => {
        if (deviceSetting) {
            await invoke.settings.ardeckPresets.saveArdeckProfile(
                deviceSetting,
            );
        }
    };

    // useEffect(() => {
    //     history.pushState(null, document.title, location.pathname);
    //     const popStateHandler = (e: BeforeUnloadEvent) => {
    //         if (location.pathname === prevLocation.current.pathname) {
    //             console.log("popstate");
    //             history.go(1);
    //         }
    //         e.preventDefault();

    //         false
    //     };

    //     // window.addEventListener("contextmenu", (e) => {popStateHandler(e)});

    //     window.addEventListener("popstate", (e) => {popStateHandler(e)});
    //     window.addEventListener("beforeunload", (e) => {popStateHandler(e)});

    //     return () => {
    //         window.removeEventListener("popstate", popStateHandler);
    //         window.removeEventListener("beforeunload", popStateHandler);
    //     };
    // }, [location]);

    return (
        <div className="flex flex-col gap-4 px-8 py-4">
            <LoadingScreen isLoading={!deviceSetting} />
            <BackToPrev className="flex items-center gap-2">
                <VscArrowLeft />
                Back to list
            </BackToPrev>
            {/* <div>Device Setting: {device_id}</div> */}
            <h1 className="text-2xl font-bold">
                Device Setting: {deviceSetting?.deviceName || "New Device"}
            </h1>
            <div className="flex flex-col gap-2">
                <label>
                    <span>Device ID</span>
                    <Input
                        name="device_id"
                        type="text"
                        disabled
                        readOnly
                        value={deviceSetting?.deviceId}
                    />
                </label>
                <label>
                    <span>Device Name</span>
                    <Input
                        name="device_name"
                        type="text"
                        value={deviceSetting?.deviceName}
                        onChange={(e) => {
                            setDeviceSetting(
                                (prev) =>
                                    prev && {
                                        ...prev,
                                        deviceName: e.target.value,
                                    },
                            );
                        }}
                    />
                </label>
                <label>
                    <span>Baud Rate</span>
                    <Select
                        name="baud_rate"
                        value={deviceSetting?.baudRate}
                        onChange={(e) => {
                            setDeviceSetting(
                                (prev) =>
                                    prev && {
                                        ...prev,
                                        baudRate: parseInt(e.target.value),
                                    },
                            );
                        }}
                    >
                        {BaudRateList.map((baudRate) => (
                            <option key={baudRate} value={baudRate}>
                                {baudRate}
                            </option>
                        ))}
                    </Select>
                </label>
                <label>
                    <span>Description</span>
                    <Input
                        name="description"
                        type="text"
                        value={deviceSetting?.description}
                        onChange={(e) => {
                            setDeviceSetting(
                                (prev) =>
                                    prev && {
                                        ...prev,
                                        description: e.target.value,
                                    },
                            );
                        }}
                    />
                </label>
                <label>
                    <span>Mapping Preset</span>
                    <div className="flex gap-2">
                        <Select
                            name="mapping_preset"
                            value={deviceSetting?.mappingPreset}
                            onChange={(e) => {
                                setDeviceSetting(
                                    (prev) =>
                                        prev && {
                                            ...prev,
                                            mappingPreset: e.target.value,
                                        },
                                );
                            }}
                        >
                            {deviceSetting?.mappingPreset ? null : (
                                <option value=""></option>
                            )}
                            {mappingPresetList?.map(([uuid, name]) => (
                                <option key={uuid} value={uuid}>
                                    {name}
                                </option>
                            ))}
                        </Select>
                        {deviceSetting?.mappingPreset ? (
                            <button
                                title="Clear"
                                className="bg-bg-secondary cursor-pointer rounded-md p-2"
                                onClick={() => {
                                    setDeviceSetting(
                                        (prev) =>
                                            prev && {
                                                ...prev,
                                                mappingPreset: "",
                                            },
                                    );
                                }}
                            >
                                <VscChromeClose />
                            </button>
                        ) : null}
                    </div>
                </label>
                <Button
                    onClick={() => {
                        saveDeviceSettingHandler();
                    }}
                    className="mt-8"
                >
                    Save
                </Button>
            </div>
        </div>
    );
}
