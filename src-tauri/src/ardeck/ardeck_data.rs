use std::vec;

use ::serde::{Deserialize, Serialize};
use chrono::{Local, TimeZone, Utc};

// #[derive(Clone, Copy)]
// pub enum SwitchType {
//     Unknown,
//     Digital,
//     Analog,
// }

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwitchData {
    switch_type: i8, // -1: Unknown, 0: Digital, 1: Analog
    id: u8,
    state: u16,
    raw_data: Vec<u8>,
    timestamp: i64,
}

impl SwitchData {
    pub fn new() -> SwitchData {
        SwitchData {
            switch_type: -1,
            id: 0,
            state: 0,
            raw_data: Vec::new(),
            timestamp: 0,
        }
    }

    pub fn set_switch_type(&mut self, switch_type: i8) {
        self.switch_type = switch_type;
    }

    pub fn switch_type(&self) -> i8 {
        self.switch_type
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn set_state(&mut self, state: u16) {
        self.state = state;
    }

    pub fn state(&self) -> u16 {
        self.state
    }

    pub fn set_raw_data(&mut self, raw_data: Vec<u8>) {
        self.raw_data = raw_data;
    }

    pub fn raw_data(&self) -> Vec<u8> {
        self.raw_data.clone()
    }

    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

pub struct ArdeckData {
    header_buf: String,
    is_reading: bool,
    read_count: u8,
    has_collect: bool,
    on_correct_handler: Box<dyn Fn(SwitchData) + Send>,
    pub protocol_version: String,
    complete_count: u128,
    switch_data_buf: SwitchData,
}

impl ArdeckData {
    const HEADER: &'static str = "ADEC";
    const HEADER_LEN: usize = 4;
    const BODY_SIZE: usize = 2;

    pub fn new() -> ArdeckData {
        let header_len = Self::HEADER.len();
        ArdeckData {
            header_buf: String::new(),
            is_reading: false,
            read_count: 0,
            has_collect: false,
            on_correct_handler: Box::new(|data| {}),
            protocol_version: "2024-06-17".to_string(), // TODO: デフォルトバージョンは最新
            complete_count: 0,
            switch_data_buf: SwitchData::new(),
        }
    }

    fn countup_read(&mut self) {
        self.read_count += 1;
        print!("\tCountup-Read: {}", self.read_count);
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
        self.switch_data_buf = SwitchData::new();
    }

    fn get_time_millis() -> i64 {
        Utc::now().timestamp_millis()
    }

    fn format_switch_data(&mut self) {
        let switch_type = self.switch_data_buf.switch_type();
        let raw_data = self.switch_data_buf.raw_data();
        println!("{:08b}", raw_data[0]);
        let mut id: u8;
        let mut state: u16;
        if switch_type == 0 {
            id = (raw_data[0] & 0b01111110) >> 1;
            state = (raw_data[0] & 0b00000001) as u16;
        } else {
            id = (raw_data[0] & 0b01111100) >> 2;
            state = ((raw_data[0] & 0b00000011) as u16) << 8 | raw_data[1] as u16;
        }

        self.switch_data_buf.set_id(id);
        self.switch_data_buf.set_state(state);

        let time = Self::get_time_millis();
        self.switch_data_buf.set_timestamp(time);
    }

    fn put_challenge(&mut self, _data: u8) -> bool {
        print!("{}", self.read_count);
        print!("\t{:08b}", &_data);

        let buf_len = self.header_buf.len();
        let if_str = String::from_utf8(vec![_data]).unwrap_or("".to_string());
        let msg = if_str.clone();
        // match if_str.clone() {
        //     Ok(msg) => {
                print!("\t{}", msg);

                /* TODO:
                // A, D, E, C, [DATA] だとCの時にデータが切れたときにめちゃくちゃなデータが来ることがある
                // A, D, [DATA], E, C にする？
                 */
                // ADECのヘッダーの頭であるAが来たら、読み取り開始
                if msg == "A" && !self.is_reading && self.read_count == 0 {
                    self.clear_flag_count(); // 念のためリセット
                    self.clear_buf();
                    self.is_reading = true;
                    self.header_buf.push('A');

                    self.countup_read();

                    print!("\tCollect-A, Start-Read");
                    println!("");

                    return false;
                }

                // 2個目にDが来たら、ヘッダーを読み取る
                if msg == "D" && self.is_reading && self.read_count == 1 {
                    self.header_buf.push('D');

                    self.countup_read();

                    print!("\tCollect-D");
                    println!("");

                    return false;
                }

                // 3個目にデータが来たら、データを読み取る
                if self.is_reading && self.read_count == 2 {
                    // self.data_buf[] = _data.clone();

                    let switch_type = self.switch_data_buf.switch_type();

                    print!("Switch-Type: {}", switch_type);

                    match switch_type {
                        -1 => {
                            // let check = self.check_switch_type(_data);
                            let check = (_data & 0b10000000) >> 7;
                            self.switch_data_buf.set_switch_type(check as i8);

                            print!("\tCheck: {}", check);

                            match check {
                                0 => {
                                    let mut raw_data = self.switch_data_buf.raw_data();
                                    raw_data.push(_data);
                                    self.switch_data_buf.set_raw_data(raw_data);

                                    self.countup_read();

                                    print!("\tCollect-Data-Digital");
                                }
                                1 => {
                                    let mut raw_data = self.switch_data_buf.raw_data();
                                    raw_data.push(_data);
                                    self.switch_data_buf.set_raw_data(raw_data);

                                    print!("\tCollect-Data-Analog-0");
                                }
                                _ => {
                                    print!("\tCollect-data-Unknown-0");
                                }
                            }
                        }
                        1 => {
                            // self.data_buf.push(_data);
                            let mut raw_data = self.switch_data_buf.raw_data();
                            raw_data.push(_data);
                            self.switch_data_buf.set_raw_data(raw_data);

                            self.countup_read();

                            print!("\tCollect-Data-Analog-1");
                        }
                        _ => {
                            print!("\tCollect-Data-Unknown-1");
                        }
                    }

                    println!("");
                    return false;
                }

                // データの後ろにE, Cが来たら、ヘッダーを読み取る
                if msg == "E" && self.is_reading && self.read_count == 3 {
                    self.header_buf.push('E');

                    self.countup_read();

                    print!("\tCollect-E");
                    println!("");

                    return false;
                }

                if msg == "C" && self.is_reading && self.read_count == 4 {
                    self.header_buf.push('C');

                    self.countup_read();

                    print!("\tCollect-C");
                    println!("");
                }

                // ヘッダーが４つ揃ったら、溜めたデータをチェックする
                if self.read_count >= 5 {
                    // 前回までに溜めたデータがADECだったら、今回のデータを正式なデータとして扱う
                    if self.header_buf == Self::HEADER {
                        self.clear_flag_count();
                        print!("\tComplete-Data");
                        println!("");

                        self.format_switch_data();

                        self.on_correct_emit(self.switch_data_buf.clone());

                        return true;
                    } else {
                        // ヘッダーがADECじゃなかったら、リセット
                        self.clear_flag_count();
                        print!("\tCollect-Reset");
                        println!("------------------------------------------------");
                        println!("");

                        return false;
                    }
                } else {
                    println!("\twtfff");
                    return false;
                }
            }
            // Err(_) => {
            //     println!("\twtf char");

            //     return false;
            // }
        // }
    // }

    pub fn on_data(&mut self, data: Vec<u8>) {
        // println!("aaaaa");

        let pc = self.put_challenge(data.clone()[0]);

        // if pc {
        //     // (self.on_correct_handler)();
        //     let time = Self::get_time_millis();
        //     self.countup_complete();
        //     self.on_correct_handler.as_mut()(self.switch_data_buf.clone());
        //     println!("On Correct! {}", Local.timestamp_millis_opt(time).unwrap());
        //     println!("------------------------------------------------");
        // }

        // if Self::HEADER == self.header_buf {
        //     println("Header Complete!");
        //     // bufをクリア
        // }
    }

    pub fn on_complete<F: Fn(SwitchData) + Send + 'static>(&mut self, handler: F) {
        self.on_correct_handler = Box::new(handler);
    }

    fn on_correct_emit(&mut self, data: SwitchData) {
        let time = Self::get_time_millis();
        println!("Switch data: {:?}", data);
        self.countup_complete();
        self.on_correct_handler.as_mut()(data);
        println!("On Correct! {}", Local.timestamp_millis_opt(time).unwrap());
        println!("------------------------------------------------");
    }
}
