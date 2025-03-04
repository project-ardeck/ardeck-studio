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
import { VscArrowLeft } from "react-icons/vsc";

export default function DeviceSetting() {
    let { device } = useParams();

    return (
        <div>
            <BackToPrev className="flex items-center gap-2"><VscArrowLeft />Back to list</BackToPrev>
            <div>Device Setting: {device}</div>
        </div>
    );
}
