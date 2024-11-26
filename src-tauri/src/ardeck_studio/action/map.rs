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

use serde::{Deserialize, Serialize};

use super::{SwitchId, SwitchType};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActionMap {
    // スイッチの種類 (デジタルスイッチか、アナログスイッチか)
    pub switch_type: SwitchType,
    // スイッチのピン番号
    pub switch_id: SwitchId,
    // プラグインのID
    pub plugin_id: String,
    // アクションのID
    pub action_id: String,
}

struct Mapping {}
