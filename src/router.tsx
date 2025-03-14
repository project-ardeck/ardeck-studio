import { BrowserRouter, Route, Routes } from "react-router";
import MainWindow from "./window_main";
import App from "./App";
// import ForDev from "./ForDev";
import Config from "./pages/config";
import About from "./pages/about";
import License from "./pages/about/license";
import Authors from "./pages/about/authors";
import Devices from "./pages/devices";
import DeviceSetting from "./pages/devices/DeviceSetting";
import Mapping from "./pages/mapping";
import MappingSetting from "./pages/mapping/MappingSetting";
import Plugin from "./pages/plugin";
import PluginActions from "./pages/plugin/PluginActions";

export default function Router() {
    return (
        <BrowserRouter>
            <Routes>
                <Route element={<MainWindow />}>
                    <Route element={<App />}>
                        <Route index element={null} />
                        <Route path="config" element={<Config />} />
                        <Route path="devices" element={<Devices />} />
                        <Route path="mapping" element={<Mapping />} />
                        <Route path="plugin" element={<Plugin />} />
                    </Route>
                    <Route
                        path="/devices/:device_id"
                        element={<DeviceSetting />}
                    />
                    <Route
                        path="/mapping/:mapping_id"
                        element={<MappingSetting />}
                    />
                    <Route
                        path="/plugin/:plugin_id"
                        element={<PluginActions />}
                    />
                    {/* <Route path="dev" element={<ForDev />} /> */}
                </Route>
                <Route path="about" element={<About />}>
                    <Route index element={<License />} />
                    <Route path="authors" element={<Authors />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}
