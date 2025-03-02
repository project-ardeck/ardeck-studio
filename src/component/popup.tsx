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

import { ReactNode, useState } from "react";

export default function Popup(
    props: {
        children: ReactNode;
        title: string;
        onClose: () => void;
        onOpen: () => void;
        isOpen: boolean;
    }
) {
    const [isOpen, setIsOpen] = useState(false);

    if (!isOpen) return null;

    return (
        <div className="fixed left-0 top-0 z-50 flex h-screen w-screen items-center justify-center bg-black/50 p-4">
            <div className="flex flex-col rounded-md bg-bg-primary p-4">
                <div className="flex items-center justify-between">
                    <h1 className="text-2xl font-bold">{props.title}</h1>
                    <button onClick={props.onClose}>X</button>
                </div>
                <div>{props.children}</div>
            </div>
        </div>
    )
}
