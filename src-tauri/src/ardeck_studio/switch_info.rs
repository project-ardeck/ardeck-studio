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

pub mod compare;

use compare::ActionCompare;
use ::serde::{Deserialize, Serialize};
use chrono::Utc;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SwitchType {
    Unknown = -1,
    Digital = 0,
    Analog = 1,
}

#[derive(Clone, Debug)]
enum BodyLen {
    Unknown = 0,
    Digital = 1,
    Analog = 2,
}

pub type SwitchId = u8;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwitchInfo {
    pub switch_type: SwitchType, // -1: Unknown, 0: Digital, 1: Analog
    pub switch_id: SwitchId,
    pub switch_state: u16,
    pub timestamp: i64,
}

impl SwitchInfo {
    pub fn new() -> Self {
        Self {
            switch_type: SwitchType::Unknown,
            switch_id: 0,
            switch_state: 0,
            timestamp: 0,
        }
    }

    pub fn set_switch_type(&mut self, switch_type: SwitchType) {
        self.switch_type = switch_type;
    }

    pub fn get_switch_type(&self) -> SwitchType {
        self.switch_type
    }

    pub fn set_switch_id(&mut self, id: u8) {
        self.switch_id = id;
    }

    pub fn get_switch_id(&self) -> u8 {
        self.switch_id
    }

    pub fn set_switch_state(&mut self, state: u16) {
        self.switch_state = state;
    }

    pub fn get_switch_state(&self) -> u16 {
        self.switch_state
    }

    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}

pub struct ActionDataParser {
    header_buf: String,
    is_reading: bool,
    read_count: u8,
    header_len: usize,
    data_len: usize, // Digital: 4, Analog: 5
    body_len: BodyLen,
    has_collect: bool,
    on_correct_handler: Vec<Box<dyn Fn(SwitchInfo) + Send + 'static>>,
    complete_count: u128,
    action_buf: SwitchInfo,
    action_raw_buf: Vec<u8>,
    compare: ActionCompare,
}

impl ActionDataParser {
    const HEADER: &'static str = "ADEC";
    const HEADER_LEN: usize = Self::HEADER.len();
    const BODY_SIZE: usize = 2;

    pub fn new() -> Self {
        let header_len = Self::HEADER.len();
        Self {
            header_buf: String::new(),
            is_reading: false,
            read_count: 0,
            header_len: Self::HEADER_LEN,
            data_len: 0,
            body_len: BodyLen::Unknown,
            has_collect: false,
            on_correct_handler: Vec::new(),
            complete_count: 0,
            action_buf: SwitchInfo::new(),
            action_raw_buf: Vec::new(),
            compare: ActionCompare::new(),
        }
    }

    fn data_of_digital_switch(&mut self) {
        self.body_len = BodyLen::Digital;
        self.data_len = Self::HEADER_LEN + BodyLen::Digital as usize;
    }

    fn data_of_analog_switch(&mut self) {
        self.body_len = BodyLen::Analog;
        self.data_len = Self::HEADER_LEN + BodyLen::Analog as usize;
    }

    fn countup_read(&mut self) {
        self.read_count += 1;
        // print!("\tCountup-Read: {}", self.read_count);
    }

    fn countup_complete(&mut self) {
        self.complete_count += 1;
    }

    fn clear_flag_count(&mut self) {
        self.is_reading = false;
        self.read_count = 0;
        self.has_collect = false;
    }

    fn clear_buf(&mut self) {
        self.header_buf.clear();
        self.action_buf = SwitchInfo::new();
        self.action_raw_buf.clear();
    }

    fn get_time_millis() -> i64 {
        Utc::now().timestamp_millis()
    }

    fn format_switch_data(&mut self) {
        let switch_type = self.action_buf.get_switch_type();
        let raw_data = &self.action_raw_buf;
        // println!("{:08b}", raw_data[0]);
        let id: u8;
        let state: u16;
        match switch_type {
            SwitchType::Digital => {
                id = (raw_data[0] & 0b01111110) >> 1;
                state = (raw_data[0] & 0b00000001) as u16;
            }
            SwitchType::Analog => {
                id = (raw_data[0] & 0b01111100) >> 2;
                state = ((raw_data[0] & 0b00000011) as u16) << 8 | raw_data[1] as u16;
            }
            _ => {
                id = 0;
                state = 0;
            }
        }

        self.action_buf.set_switch_id(id);
        self.action_buf.set_switch_state(state);

        let time = Self::get_time_millis();
        self.action_buf.set_timestamp(time);
    }

    fn put_challenge(&mut self, _data: u8) -> bool {
        // print!("count: {}", self.read_count);
        // print!("\t{:08b}", &_data);

        let if_str = String::from_utf8(vec![_data]).unwrap_or("".to_string());
        let msg = if_str.clone();
        // print!("\t{}", msg);
        // ADECのヘッダーの頭であるAが来たら、読み取り開始
        if msg == "A" && !self.is_reading
        /* && self.read_count == 0 */
        {
            self.clear_flag_count(); // 念のためリセット
            self.clear_buf();
            self.is_reading = true;
            self.header_buf.push('A');

            // print!("\tCollect-A, Start-Read");
        }

        // 2個目にDが来たら、ヘッダーを読み取る
        if msg == "D" && self.is_reading
        /* && self.read_count == 1 */
        {
            self.header_buf.push('D');

            // print!("\tCollect-D");
        }

        // 3個目にデータが来たら、データを読み取る
        if self.is_reading && self.read_count == 2 || self.read_count == 3 {
            // self.data_buf[] = _data.clone();

            let switch_type = self.action_buf.get_switch_type();

            // print!("Switch-Type: {:?}", switch_type);
            

            match switch_type {
                SwitchType::Unknown => {
                    // let check = self.check_switch_type(_data);
                    let check = match (_data & 0b10000000) >> 7 {
                        0 => SwitchType::Digital,
                        1 => SwitchType::Analog,
                        _ => SwitchType::Unknown,
                    };
                    self.action_buf.set_switch_type(check);

                    // print!("\tCheck: {:?}", check);

                    match check {
                        SwitchType::Digital => {
                            self.data_of_digital_switch();

                            self.action_raw_buf.push(_data);

                            // print!("\tCollect-Data-Digital");
                            
                            // println!("{:08b}", _data);
                        }
                        SwitchType::Analog => {
                            self.data_of_analog_switch();

                            self.action_raw_buf.push(_data);

                            // print!("\tCollect-Data-Analog-0");
                            // println!("{:08b}", _data);
                        }
                        _ => {
                            // print!("\tCollect-data-Unknown-0");
                        }
                    }
                }
                SwitchType::Analog => {
                    self.action_raw_buf.push(_data);

                    // print!("\tCollect-Data-Analog-1");
                    // println!("{:08b}", _data);
                }
                SwitchType::Digital => {}
            }
        }

        // データの後ろにE, Cが来たら、ヘッダーを読み取る
        if msg == "E" && self.is_reading
        /* && self.read_count as i8 == 3 + self.switch_data_buf.switch_type() */
        {
            self.header_buf.push('E');

            // print!("\tCollect-E");
        }

        if msg == "C" && self.is_reading
        /* && self.read_count as i8 == 4 + self.switch_data_buf.switch_type() */
        {
            self.header_buf.push('C');

            // print!("\tCollect-C");
        }

        // ヘッダーが４つ揃ったら、溜めたデータをチェックする
        if self.header_buf.len() == Self::HEADER_LEN as usize || self.read_count > 5 {
            self.countup_read();

            // 前回までに溜めたデータがADECだったら、今回のデータを正式なデータとして扱う
            if self.header_buf == Self::HEADER && self.read_count as i8 == self.data_len as i8 {
                self.clear_flag_count();
                // print!("\tComplete-Data");
                // println!("");

                self.format_switch_data();

                self.on_complete_emit_all(self.action_buf.clone());

                return true;
            } else {
                // ヘッダーがADECじゃなかったら、リセット
                self.clear_flag_count();
                // println!("\tCollect-Reset");
                // println!("------------------------------------------------");
                // println!("");

                return false;
            }
        } else {
            if self.is_reading {
                self.countup_read();
            }
            // println!("");

            return false;
        }
    }

    pub fn put_data(&mut self, data: Vec<u8>) {
        self.put_challenge(data.clone()[0]);
    }

    pub fn on_complete<F: Fn(SwitchInfo) + Send + 'static>(&mut self, callback: F) {
        self.on_correct_handler.push(Box::new(callback));
    }

    pub fn on_change_action<F: Fn(SwitchInfo) + Send + 'static>(&mut self, callback: F) {
        self.compare.on_change_action(callback);
    }

    fn on_complete_emit_all(&mut self, action: SwitchInfo) {
        self.countup_complete();
        self.compare.put_action(action.clone()); // on change actionのために
        // println!("{:?}", action);

        for h in self.on_correct_handler.iter() {
            
            let time = Self::get_time_millis();
            // println!("Switch data: {:?}", data);
            // *h.as_mut()(data.clone());
            h(action.clone());
            // println!("On Correct! {}", Local.timestamp_millis_opt(time).unwrap());
            // println!("------------------------------------------------");
        }
    }
}
