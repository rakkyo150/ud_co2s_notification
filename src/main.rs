use serde_json::{self, json};
use serial::SerialPort;
use std::fs::File;
use std::io::{self, prelude::*};
use std::io::BufReader;
use dotenv::dotenv;
use reqwest::blocking::Client;
use chrono::prelude::*;

mod image;
mod log;

// 以下は基本的に変更する必要はありません
const DEVICE_PATH: &str = "/dev/ttyACM0";

struct UDCO2S {
    dev: String,
}

impl UDCO2S {
    pub fn new(dev: &str) -> Self {
        UDCO2S {
            dev: dev.into(),
        }
    }
    
    pub fn start_logging(&self, log_file: &str) -> io::Result<Vec<log::Log>> {
        let regex = regex::Regex::new(r"CO2=(?P<co2>\d+),HUM=(?P<hum>\d+\.\d+),TMP=(?P<tmp>-?\d+\.\d+)").unwrap();
        
        let mut port = serial::open(&self.dev).unwrap();

        let option_func = &|settings: &mut dyn serial::SerialPortSettings|{
            _ = settings.set_baud_rate(serial::Baud115200);
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        };

        _ = port.reconfigure(option_func);
        _ = port.set_timeout(std::time::Duration::from_secs(6));
        
        write!(&mut port, "STA\r\n")?;
        print!("{}", read_until(&mut port, '\n')?); // Print the first line
        
        let mut all_data: Vec<log::Log> = vec![];
        
        if let Ok(line) = read_until(&mut port, '\n') {
            if let Some(m) = regex.captures(&line) {

                let obj=log::Log{
                    time: Local::now(),
                    status: log::UDCO2SStat::new(
                        m["co2"].parse::<i64>().unwrap(),
                        m["hum"].parse::<f32>().unwrap(),
                        m["tmp"].parse::<f32>().unwrap()
                )};

                println!("{}", m["co2"].parse::<i32>().unwrap());
                
                let file = File::open(log_file)?;
                let reader = BufReader::new(&file);
                let response = serde_json::from_reader(reader);
                all_data = if let Ok(response_) = response{
                    response_
                }else if let Err(e) = response{
                    println!("Error: {}",e);
                    vec![]
                }else{
                    println!("Weird Error");
                    vec![]
                };

                all_data.push(obj);
                let now: DateTime<Local> = Local::now(); // 現在の日時を取得
                if now.hour() == 0 {
                    all_data = vec![];
                }

                let mut file = File::create(log_file)?;

                file.write_all(serde_json::to_string(&all_data).unwrap().as_bytes())?;
            }
        }
        
        write!(&mut port, "STP\r\n")?;

        Ok(all_data)
    }
}

fn read_until(port: &mut dyn serial::SerialPort, del: char) -> io::Result<String> {
    let mut res = String::new();
    loop {
        let mut buf = [0u8; 1];
        match port.read_exact(&mut buf) {
            Ok(_) => {
                let ch = char::from(buf[0]);
                if ch == del {
                    return Ok(res);
                } else {
                    res.push(ch);
                }
            }
            Err(e) => match e.kind() {
                io::ErrorKind::TimedOut => return Ok(res),
                _ => return Err(e.into()),
            },
        }
    }
}

fn send_message(webhook_url: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = json!({
        "content": message,
    });

    let client = Client::new();
    let response = client
        .post(webhook_url)
        .json(&json)
        .send()?;

    println!("send_message: {}",response.status());

    Ok(())
}

fn send_image(webhook_url: &str, image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let file = reqwest::blocking::multipart::Form::new().file("key", image_path)?;      

    let response = reqwest::blocking::Client::new()
    .post(webhook_url)
    .multipart(file)
    .send()
    .unwrap(); 

    println!("send_image: {}",response.status());

    Ok(())
}


fn main() {
    dotenv().ok();

    let data_file_path = std::env::var("DATA_FILE_PATH").expect("DATABASE_URL must be set");
    let webhook_url = std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");

    let sensor = UDCO2S::new(DEVICE_PATH);
    let all_data_result = sensor.start_logging(&data_file_path);
    let mut all_data: Vec<log::Log> = vec![];
    match all_data_result { 
        Ok(response) => all_data = response,
        Err(e) => println!("{}", e)
    };
    println!("{}",all_data.last().unwrap().status.co2ppm);
    let now_ppm = format!("{}{}",all_data.last().unwrap().status.co2ppm.to_string(), "ppm");
    let _ = image::generate(&data_file_path);

    let _ = send_message(&webhook_url,&now_ppm);
    let _ = send_image(&webhook_url, "./output.png");
}