import React, { Children } from "react";

const themeList = [
    "default-dark",
    "default-light",
    "fk-neon",
    "kawaii-light",
] as const;
declare global {
    type ThemeList = typeof themeList[number];
}

type Theme = {
    id: string;
    name?: string;
    author?: string;

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

    get themeList() {
        return themeList;
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