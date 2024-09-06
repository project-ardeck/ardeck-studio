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


import React, { Children } from "react";

const themeList = [
    "default-dark",
    "default-light",
    "fk-neon",
    // "kawaii-blue",
    "d1sc0rd-dark"
] as const;

declare global {
    type ThemeList = typeof themeList[number];

    type ThemeInfo = {
        id: string;
        name?: string;
        author?: string;
    }
    
    type Theme = ThemeInfo & {
        base: "dark" | "light";
    
        colors: {
            "bg-titlebar": number[];
    
            "bg-primary": number[];
            "bg-secondary"?: number[];
            "bg-tertiary"?: number[];
            "bg-quaternary"?: number[];
    
            "text-primary": number[];
            "text-reverse"?: number[];
    
            "accent-primary": number[];
            "accent-secondary"?: number[];
    
            "accent-positive": number[];
            "accent-caution": number[];
            "accent-negative": number[];
            "accent-link"?: number[];
        }
    };
}



class themeConfigs {
    static themeFormatting(theme: Theme): Theme {
        const overFormat = (color: number[]) => {
            return color.map((value) => {
                if (value < 0) {
                    return 0;
                } else if (value > 255) {
                    return 255;
                } else {
                    return value;
                }
            });
        }

        if (!theme.colors["bg-secondary"]) {
            theme.colors["bg-secondary"] = overFormat(
                theme.colors["bg-primary"].map((color) => color + 10)
            );
        }

        // TODO : Add more formatting

        return theme;
    }

    static async getTheme(theme: ThemeList): Promise<Theme> {
        const res = await fetch(`/theme/${theme}.json`);
        const json: Theme = await res.json();

        console.log(json);

        return json;
    }
}

declare global {
    interface Window {
        windowTheme: WindowTheme;
    }
}

type WindowThemeProps = {
    children: React.ReactNode;
};

export default class WindowTheme extends React.Component<WindowThemeProps> {
    private nowTheme: ThemeList = "default-dark";

    get theme() {
        return this.nowTheme;
    }

    async themeList() {
        console.log("getting list...");

        let list: ThemeInfo[] = [];
        
        for (let i = 0; i < themeList.length; i ++) {
            const themeConfig = await themeConfigs.getTheme(themeList[i]);
            
            const tmp = {
                id: themeConfig.id,
                name: themeConfig.name,
                author: themeConfig.author,
            }
            
            list.push(tmp);
        }
        
        return list;
    }
    
    

    constructor(props: WindowThemeProps) {
        super(props);

        window.windowTheme = this;
    }

    async setTheme(theme: ThemeList) {
        const themeConfig = await themeConfigs.getTheme(theme);
        const formattedTheme = themeConfigs.themeFormatting(themeConfig);

        const root = document.documentElement;

        Object.entries(formattedTheme.colors).forEach(([key, value]) => {
            root.style.setProperty(`--${key}`, value.join(" "));
        });

        this.nowTheme = theme;
    }

    render(): React.ReactNode {
        return <>{this.props.children}</>;
    }
}