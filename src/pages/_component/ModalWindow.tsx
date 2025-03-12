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

import { EventCallback } from "@tauri-apps/api/event";
import { Attributes, HTMLAttributes, ReactNode, useState } from "react";

export function ModalWindow(props: {
    isOpen: boolean;
    onOpen?: () => void;
    onClose?: () => void;
    children: ReactNode;
}): ReactNode {
    if (props.onOpen && props.isOpen) props.onOpen();
    if (props.onClose && !props.isOpen) props.onClose();

    return props.isOpen ? props.children : null;
}

export function ModalWindowContainer(
    props: {
        children: ReactNode;
        backdropClassName?: string;
    } & HTMLAttributes<HTMLDivElement>,
): ReactNode {
    return (
        <>
            <div className={`modal-backdrop ${props.backdropClassName}`} />
            <div {...props} className={`modal ${props.className}`} />
        </>
    );
}
