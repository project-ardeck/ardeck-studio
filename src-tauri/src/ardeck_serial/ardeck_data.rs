pub struct SwitchData {
    id : u8,
    state : u8,
}

pub struct ArdeckData {
    header: String,
    bodysize: u16, // [byte]
    header_buf: String,
    data_buf: Vec<u8>,
    is_reading: bool,
    read_count: u8,
    on_correct_handler: Box<dyn Fn(u8) + Send>,
    pub protocol_version: String,
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
            data_buf: vec![0; 1],
            is_reading: false,
            read_count: 0,
            on_correct_handler: Box::new(|x| {}),
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
    }

    fn put_challenge(&mut self, _data: Vec<u8>) -> bool {
        print!("{:0<8b}", &_data[0]);

        let buf_len = self.header_buf.len();
        let if_str = String::from_utf8(_data.clone());
        match if_str.clone() {
            Ok(msg) => {
                print!("\t{}", msg);

                /* TODO:
                // A, D, E, C, [DATA] だとCの時にデータが切れたときにめちゃくちゃなデータが来ることがある
                // A, D, [DATA], E, C にする？
                */

                if self.protocol_version == "2014-06-03".to_string() {
                    if msg == "A" && self.is_reading == false && self.read_count == 0 {
                        self.param_reset();
                        self.is_reading = true;
                        self.header_buf.push('A');
    
                        print!("\tCollect-A, Start-Read");
                        println!("");
                    }
                    if msg == "D" && self.is_reading == true && self.read_count == 1 {
                        self.header_buf.push('D');
    
                        print!("\tCollect-D");
                        println!("");
                    }
                    if msg == "E" && self.is_reading == true && self.read_count == 2 {
                        self.header_buf.push('E');
    
                        print!("\tCollect-E");
                        println!("");
                    }
                    if msg == "C" && self.is_reading == true && self.read_count == 3 {
                        self.header_buf.push('C');
    
                        print!("\tCollect-C");
                        println!("");
                    }
    
                    if Self::HEADER.to_string() == self.header_buf
                        && Self::HEADER.to_string().len() == self.header_buf.len()
                        && self.is_reading == true
                        && self.read_count == 4
                    {
                        // self.countup();
                        self.data_buf = _data;
    
                        print!("\t All Collect!!");
                        println!("");
    
                        return true;
                    }

                    if self.is_reading == true && self.read_count == 4 {
                        self.param_reset();
    
                        println!("");
    
                        return false;
                    }

                    if self.is_reading {
                        self.countup();
                    }
                    return false;
                } else if self.protocol_version == "2024-06-17".to_string() {
                    println!("{}", self.read_count);
                    if msg == "A" && self.is_reading == false && self.read_count == 0 {
                        self.param_reset();
                        self.is_reading = true;
                        self.header_buf.push('A');
    
                        print!("\tCollect-A, Start-Read");
                        println!("");
                    }

                    if msg == "D" && self.is_reading == true && self.read_count == 1 {
                        self.header_buf.push('D');
    
                        print!("\tCollect-D");
                        println!("");
                    }

                    if self.is_reading == true
                        && self.header_buf == "AD".to_string()
                        && self.read_count == 2
                    {
                        print!("\tCollect-DataSize");
                        println!("");
                    }

                    if msg == "E" && self.is_reading == true && self.read_count == 3 {
                        self.header_buf.push('E');
    
                        print!("\tCollect-E");
                        println!("");
                    }

                    if msg == "C" && self.is_reading == true && self.read_count == 4 {
                        self.header_buf.push('C');
    
                        print!("\tCollect-C");
                        println!("");
                    }

                    if Self::HEADER.to_string() == self.header_buf
                        && Self::HEADER.to_string().len() == self.header_buf.len()
                        && self.is_reading == true
                        && self.read_count == 4
                    {
                        // self.countup();
                        self.data_buf = _data;
    
                        print!("\t All Collect!!");
                        println!("");
    
                        return true;
                    }

                    if self.is_reading == true && self.read_count == 4 {
                        self.param_reset();
    
                        println!("");
    
                        return false;
                    }
                    
                    if self.is_reading {
                        self.countup();
                    }
                    return false;
                } else {
                    return false
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
            self.on_correct_handler.as_mut()(self.data_buf[0]);
        }

        // if Self::HEADER == self.header_buf {
        //     println("Header Complete!");
        //     // bufをクリア
        // }
    }

    pub fn on_collect<F: Fn(u8) + Send + 'static>(&mut self, handler: F)
    {
        self.on_correct_handler = Box::new(handler);
        
    }
}
