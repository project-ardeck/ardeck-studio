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
        result += characters.charAt(Math.floor(Math.random() * characters.length));
    }
    return result;
}

export function makeUid(): string {
    const RandomStr = randomStr(0b0110, 64);

    const DATE = Date.now();

    const Uid = `${RandomStr}-${DATE.toString(16)}`;

    return Uid;
}