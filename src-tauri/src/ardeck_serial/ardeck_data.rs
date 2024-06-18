pub struct SwitchData {
    id : u8,
    state : u8,
}

pub struct ArdeckData {
    header: String,
    bodysize: u16, // [byte]
    header_buf: String,
    is_reading: bool,
    on_correct_handler: Box<dyn Fn(u8) + Send>,
    data_count: u128,
}

impl ArdeckData {
    const HEADER: &'static str = "ADEC";

    pub fn new() -> ArdeckData {
        let header_len = Self::HEADER.len();
        let header_buf = String::new();
        ArdeckData {
            header: Self::HEADER.to_string(),
            bodysize: 1,
            header_buf,
            is_reading: false,
            on_correct_handler: Box::new(|x| {}),
            data_count: 0,
        }
    }

    fn put_challenge(&mut self, _data: Vec<u8>) -> bool {
        print!("{:0<8b}", &_data[0]);

        let buf_len = self.header_buf.len();
        let if_str = String::from_utf8(_data);
        match if_str.clone() {
            Ok(msg) => {
                print!("\t{}", msg);

                /* TODO:
                // A, D, E, C, [DATA] だとCの時にデータが切れたときにめちゃくちゃなデータが来ることがある
                // A, D, [DATA], E, C にする？
                */

                if msg == "A" && self.is_reading == false {
                    self.header_buf = "".to_string();
                    self.is_reading = true;
                    self.header_buf.push('A');

                    print!("\tCollect-A, Start-Read");
                    println!("");

                    return false;
                }
                if msg == "D" && self.is_reading == true {
                    self.header_buf.push('D');

                    print!("\tCollect-D");
                    println!("");

                    return false;
                }
                if msg == "E" && self.is_reading == true {
                    self.header_buf.push('E');

                    print!("\tCollect-E");
                    println!("");

                    return false;
                }
                if msg == "C" && self.is_reading == true {
                    self.header_buf.push('C');

                    print!("\tCollect-C");
                    println!("");

                    return false;
                }

                if Self::HEADER.to_string().len() > 4 {
                    self.is_reading = false;

                    print!("\tErr, Over Header");
                    println!("");

                    return false;
                }

                if Self::HEADER.to_string() == self.header_buf
                    && Self::HEADER.to_string().len() == self.header_buf.len()
                {
                    self.is_reading = false;

                    print!("\t All Collect!!");
                    println!("");

                    return true;
                } else {
                    self.is_reading = false;

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
            println!("------------------------ On Collect ------------------------");
            // (self.on_correct_handler)();
            self.on_correct_handler.as_mut()(data[0]);
            self.countup();
        }

        // if Self::HEADER == self.header_buf {
        //     println("Header Complete!");
        //     // bufをクリア
        // }
    }

    fn countup(&mut self) {
        self.data_count += 1;
    }

    pub fn on_collect<F: Fn(u8) + Send + 'static>(&mut self, handler: F)
    {
        self.on_correct_handler = Box::new(handler);
        
    }
}
