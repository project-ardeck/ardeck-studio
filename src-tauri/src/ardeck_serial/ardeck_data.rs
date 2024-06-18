use core::time;

use chrono::{Local, TimeZone, Utc};

pub struct SwitchData {
    id: u8,
    state: u8,
}

pub struct ArdeckData {
    header: String,
    bodysize: u16, // [byte]
    header_buf: String,
    data_buf: Vec<u8>,
    is_reading: bool,
    read_count: u8,
    has_collect: bool,
    on_correct_handler: Box<dyn Fn(u8, i64) + Send>,
    pub protocol_version: String,
}

impl ArdeckData {
    const HEADER: &'static str = "ADEC";
    const HEADER_LEN: usize = 4;
    const BODY_SIZE: u16 = 1;

    pub fn new() -> ArdeckData {
        let header_len = Self::HEADER.len();
        ArdeckData {
            header: Self::HEADER.to_string(),
            bodysize: Self::BODY_SIZE,
            header_buf: String::new(),
            data_buf: vec![0; Self::BODY_SIZE as usize],
            is_reading: false,
            read_count: 0,
            has_collect: false,
            on_correct_handler: Box::new(|x, timestamp| {}),
            protocol_version: "2024-06-17".to_string(), // TODO: デフォルトバージョンは最新
        }
    }

    fn countup(&mut self) {
        self.read_count += 1;
    }

    fn param_reset(&mut self) {
        self.header_buf = "".to_string();
        self.is_reading = false;
        self.read_count = 0;
        self.has_collect = false;
    }

    fn get_time_millis() -> i64 {
        Utc::now().timestamp_millis()
    }

    fn put_challenge(&mut self, _data: Vec<u8>) -> bool {
        print!("{:08b}", &_data[0]);

        let buf_len = self.header_buf.len();
        let if_str = String::from_utf8(_data.clone());
        match if_str.clone() {
            Ok(msg) => {
                print!("\t{}", msg);

                /* TODO:
                // A, D, E, C, [DATA] だとCの時にデータが切れたときにめちゃくちゃなデータが来ることがある
                // A, D, [DATA], E, C にする？
                 */

                if self.protocol_version == "2014-06-03".to_string() || true {
                    // A, D, E, C, [DATA]

                    // ADECのヘッダーの頭であるAが来たら、読み取り開始
                    if msg == "A" && !self.is_reading && self.read_count == 0 {
                        self.param_reset(); // 念のためリセット
                        self.is_reading = true;
                        self.header_buf.push('A');

                        print!("\tCollect-A, Start-Read");
                        println!("");
                    }

                    // D, E, C が順番に来たら、ヘッダーを読み取る
                    if  msg == "D" && self.is_reading && self.read_count == 1 {
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
                            self.data_buf = _data;
                            self.param_reset();
                            print!("\tCollect-Data");
                            println!("");

                            return true;
                        } else {
                            // ヘッダーがADECじゃなかったら、リセット
                            self.param_reset();
                            print!("\tCollect-Reset");
                            println!("");

                            return false;
                        }
                    } else {
                        // ヘッダーが４つ揃ってなかったら、読み取り中ならばカウントアップ
                        if self.is_reading {
                            self.countup();
                        }
                        return false;
                    }
                } else if self.protocol_version == "2024-06-17".to_string() {
                    false
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

        let pc = self.put_challenge(data.clone());

        if pc {
            // (self.on_correct_handler)();
            let time = Self::get_time_millis();
            self.on_correct_handler.as_mut()(self.data_buf[0], time);
            println!("On Correct! {}", Local.timestamp_millis_opt(time).unwrap());
            println!("------------------------------------------------");
        }

        // if Self::HEADER == self.header_buf {
        //     println("Header Complete!");
        //     // bufをクリア
        // }
    }

    pub fn on_collect<F: Fn(u8, i64) + Send + 'static>(&mut self, handler: F) {
        self.on_correct_handler = Box::new(handler);
    }
}
