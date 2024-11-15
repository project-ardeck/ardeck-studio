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

export function randomStr(type: number, length: number): string {
    const upperCase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // 1000
    const lowerCase = "abcdefghijklmnopqrstuvwxyz"; // 0100
    const numbers = "0123456789"; // 0010
    const specialCharacters = "!@#$%^&*()_+~`|}{[]\\:;?><,./-="; // 0001

    let characters: string = "";

    if (type & 0b1000) characters += upperCase;
    if (type & 0b0100) characters += lowerCase;
    if (type & 0b0010) characters += numbers;
    if (type & 0b0001) characters += specialCharacters;

    let result: string = "";

    for (let i = 0; i < length; i++) {
        result += characters.charAt(
            Math.floor(Math.random() * characters.length),
        );
    }
    return result;
}

export function makeUid(): string {
    const RandomStr = randomStr(0b0110, 64);

    const DATE = Date.now();

    const Uid = `${RandomStr}-${DATE.toString(16)}`;

    return Uid;
}
