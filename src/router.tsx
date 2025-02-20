import { BrowserRouter, Route, Routes } from "react-router";
import MainWindow from "./window_main";
import App from "./App";
import ForDev from "./ForDev";
import Config from "./pages/config";

export default function Router() {
    return (
        <BrowserRouter>
            <Routes>
                <Route element={<MainWindow />}>
                    <Route index element={<App />} />
                    <Route path="config" element={<Config />} />
                    <Route path="dev" element={<ForDev />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}
