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

import { ReactNode } from "react";
import { Link, useNavigate } from "react-router";

export default function BackToPrev(props: {
    children: ReactNode;
    className?: string;
}) {
    const navigate = useNavigate();
    return (
        <button className={`cursor-pointer ${props.className}`} onClick={() => navigate(-1)}>
            {props.children}
        </button>
    );
}
