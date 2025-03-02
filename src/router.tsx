import { BrowserRouter, Route, Routes } from "react-router";
import MainWindow from "./window_main";
import App from "./App";
import ForDev from "./ForDev";
import Config from "./pages/config";
import About from "./pages/about";
import License from "./pages/about/license";
import Authors from "./pages/about/authors";
import Devices from "./pages/devices";

export default function Router() {
    return (
        <BrowserRouter>
            <Routes>
                <Route element={<MainWindow />}>
                    <Route index element={<App />} />
                    <Route path="config" element={<Config />} />
                    <Route path="devices" element={<Devices />} />
                    <Route path="dev" element={<ForDev />} />
                </Route>
                <Route path="about" element={<About />}>
                    <Route index element={<License />} />
                    <Route path="authors" element={<Authors />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}
