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

import { Link, NavLink, Outlet } from "react-router";

export default function About() {
    return (
        <div className="relative flex h-screen w-screen flex-col gap-2 bg-bg-primary p-2 text-text-primary">
            <div className="flex">
                <img src="/128x128@2x.png" width={128} />
                <div className="flex-1">
                    <h1 className="text-center text-4xl font-bold">
                        Ardeck Studio
                    </h1>
                    <p className="text-center">
                        The ardeck command mapping software.
                    </p>
                </div>
            </div>
            <div className="relative flex flex-1 flex-col">
                <div className="flex gap-2">
                    <NavLink
                        className={({ isActive }) =>
                            isActive
                                ? "rounded-t-md bg-bg-secondary px-2 py-1"
                                : "rounded-t-md px-2 py-1"
                        }
                        to={""}
                    >
                        License
                    </NavLink>
                    <NavLink
                        className={({ isActive }) =>
                            isActive
                                ? "rounded-t-md bg-bg-secondary px-2 py-1"
                                : "rounded-t-md px-2 py-1"
                        }
                        to={"authors"}
                    >
                        Authors
                    </NavLink>
                </div>

                <div className="relative flex-1 rounded-b-md bg-bg-secondary">
                    <div className="absolute bottom-0 left-0 right-0 top-0 h-full overflow-y-auto p-2">
                        <Outlet />
                    </div>
                </div>
            </div>
        </div>
    );
}
