use core::time;
use std::vec;

use chrono::{Local, TimeZone, Utc};

#[derive(Clone, Copy)]
pub enum SwitchType {
    Unknown,
    Digital,
    Analog,
}

#[derive(Clone, Copy)]
pub struct SwitchData {
    switch_type: SwitchType,
    id: u8,
    state: u16,
    raw_data: u16,
}

impl SwitchData {
    pub fn new() -> SwitchData {
        SwitchData {
            switch_type: SwitchType::Unknown,
            id: 0,
            state: 0,
            raw_data: 0,
        }
    }

    pub fn set_switch_type(&mut self, switch_type: SwitchType) {
        self.switch_type = switch_type;
    }

    pub fn switch_type(&self) -> SwitchType {
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

    pub fn set_raw_data(&mut self, raw_data: u16) {
        self.raw_data = raw_data;
    }

    pub fn raw_data(&self) -> u16 {
        self.raw_data
    }
}

pub struct ArdeckData {
    header: String,
    bodysize: usize, // [byte]
    header_buf: String,
    data_buf: Vec<u8>,
    is_reading: bool,
    read_count: u8,
    has_collect: bool,
    on_correct_handler: Box<dyn Fn(u8, i64) + Send>,
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
            header: Self::HEADER.to_string(),
            bodysize: Self::BODY_SIZE,
            header_buf: String::new(),
            data_buf: Vec::new(),
            is_reading: false,
            read_count: 0,
            has_collect: false,
            on_correct_handler: Box::new(|x, timestamp| {}),
            protocol_version: "2024-06-17".to_string(), // TODO: デフォルトバージョンは最新
            complete_count: 0,
            switch_data_buf: SwitchData::new(),
        }
    }

    fn countup_read(&mut self) {
        self.read_count += 1;
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
        self.data_buf.clear();
        self.switch_data_buf = SwitchData::new();
    }

    fn get_time_millis() -> i64 {
        Utc::now().timestamp_millis()
    }

    fn check_switch_type(&mut self, data: u8) -> SwitchType {
        let switch_type = data & 0b10000000 >> 7;

        if switch_type == 0 {
            self.switch_data_buf.set_switch_type(SwitchType::Digital);
        } else if switch_type == 1 {
            self.switch_data_buf.set_switch_type(SwitchType::Analog);
        }

        self.switch_data_buf.switch_type()
    }

    fn put_challenge(&mut self, _data: u8) -> bool {
        print!("{:08b}", &_data);

        let buf_len = self.header_buf.len();
        let if_str = String::from_utf8(vec![_data]);
        match if_str.clone() {
            Ok(msg) => {
                print!("\t{}", msg);

                /* TODO:
                // A, D, E, C, [DATA] だとCの時にデータが切れたときにめちゃくちゃなデータが来ることがある
                // A, D, [DATA], E, C にする？
                 */

                if self.protocol_version == "2014-06-03".to_string() {
                    // A, D, E, C, [DATA]

                    // ADECのヘッダーの頭であるAが来たら、読み取り開始
                    if msg == "A" && !self.is_reading && self.read_count == 0 {
                        self.clear_flag_count(); // 念のためリセット
                        self.clear_buf();
                        self.is_reading = true;
                        self.header_buf.push('A');

                        print!("\tCollect-A, Start-Read");
                        println!("");
                    }

                    // D, E, C が順番に来たら、ヘッダーを読み取る
                    if msg == "D" && self.is_reading && self.read_count == 1 {
                        self.header_buf.push('D');

                        print!("\tCollect-D");
                        println!("");
                    }

                    if msg == "E" && self.is_reading && self.read_count == 2 {
                        self.header_buf.push('E');

                        print!("\tCollect-E");
                        println!("");
                    }

                    if msg == "C" && self.is_reading && self.read_count == 3 {
                        self.header_buf.push('C');

                        print!("\tCollect-C");
                        println!("");
                    }

                    // ヘッダーが４つ揃ったら、溜めたデータをチェックする
                    if self.read_count >= 4 {
                        // 前回までに溜めたデータがADECだったら、今回のデータを正式なデータとして扱う
                        if self.header_buf == Self::HEADER {
                            self.data_buf[0] = _data;
                            self.clear_flag_count();
                            print!("\tCollect-Data");
                            println!("");

                            return true;
                        } else {
                            // ヘッダーがADECじゃなかったら、リセット
                            self.clear_flag_count();
                            print!("\tCollect-Reset");
                            println!("");

                            return false;
                        }
                    } else {
                        // ヘッダーが４つ揃ってなかったら、読み取り中ならばカウントアップ
                        if self.is_reading {
                            self.countup_read();
                        }
                        print!("\t{}", self.read_count);
                        println!("");
                        return false;
                    }
                } else if self.protocol_version == "2024-06-17".to_string() {
                    // ADECのヘッダーの頭であるAが来たら、読み取り開始
                    if msg == "A" && !self.is_reading && self.read_count == 0 {
                        self.clear_flag_count(); // 念のためリセット
                        self.clear_buf();
                        self.is_reading = true;
                        self.header_buf.push('A');

                        self.countup_read();

                        print!("\tCollect-A, Start-Read");
                        println!("");
                    }

                    // 2個目にDが来たら、ヘッダーを読み取る
                    if msg == "D" && self.is_reading && self.read_count == 1 {
                        self.header_buf.push('D');

                        self.countup_read();

                        print!("\tCollect-D");
                        println!("");
                    }

                    // 3個目にデータが来たら、データを読み取る
                    if self.is_reading && self.read_count == 2 {
                        // self.data_buf[] = _data.clone();

                        let switch_type = self.switch_data_buf.switch_type();

                        match switch_type {
                            SwitchType::Unknown => {
                                let check = self.check_switch_type(_data);

                                match check {
                                    SwitchType::Digital => {
                                        self.data_buf = vec![0; 1];
                                        self.data_buf[0] = _data;
                                    },
                                    SwitchType::Analog => {
                                        self.data_buf = vec![0; 2];
                                        self.data_buf[0] = _data;
                                    },
                                    _ => {}
                                }

                            }
                            _ => {
                                self.data_buf[1] = _data;
                                self.countup_read();
                            }
                        }

                        println!("\tCollect-Data");
                    }

                    // データの後ろにE, Cが来たら、ヘッダーを読み取る
                    if msg == "E" && self.is_reading && self.read_count == 3 {
                        self.header_buf.push('E');

                        self.countup_read();

                        print!("\tCollect-E");
                        println!("");
                    }

                    if msg == "C" && self.is_reading && self.read_count == 4 {
                        self.header_buf.push('C');

                        self.countup_read();

                        print!("\tCollect-C");
                        println!("");
                    }

                    // ヘッダーが４つ揃ったら、溜めたデータをチェックする
                    if self.read_count >= 4 {
                        // 前回までに溜めたデータがADECだったら、今回のデータを正式なデータとして扱う
                        if self.header_buf == Self::HEADER {
                            self.clear_flag_count();
                            print!("\tCollect-Data");
                            println!("");

                            return true;
                        } else {
                            // ヘッダーがADECじゃなかったら、リセット
                            self.clear_flag_count();
                            print!("\tCollect-Reset");
                            println!("");

                            return false;
                        }
                    } else {
                        // ヘッダーが４つ揃ってなかったら、読み取り中ならばカウントアップ
                        print!("\t{}", self.read_count);
                        println!("");
                        return false;
                    }
                } else {
                    return false;
                }
            }
            Err(_) => {
                println!("\t");

                return false;
            }
        }
    }

    pub fn on_data(&mut self, data: Vec<u8>) {
        // println!("aaaaa");

        let pc = self.put_challenge(data.clone()[0]);

        if pc {
            // (self.on_correct_handler)();
            let time = Self::get_time_millis();
            self.countup_complete();
            self.on_correct_handler.as_mut()(self.data_buf[0], time);
            println!("On Correct! {}", Local.timestamp_millis_opt(time).unwrap());
            println!("------------------------------------------------");
        }

        // if Self::HEADER == self.header_buf {
        //     println("Header Complete!");
        //     // bufをクリア
        // }
    }

    pub fn on_complete<F: Fn(u8, i64) + Send + 'static>(&mut self, handler: F) {
        self.on_correct_handler = Box::new(handler);
    }
}
