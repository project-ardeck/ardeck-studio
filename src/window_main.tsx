import { Outlet } from "react-router";
import TitleBar from "./component/TitleBar";

export default function MainWindow() {
    return (
        <div className="flex h-full w-full flex-col overflow-hidden">
            <header>
                <TitleBar />
            </header>
            <main className="scrollbar-theme bg-bg-primary text-text-primary relative flex-1 overflow-auto">
                <Outlet />
            </main>
        </div>
    );
}
