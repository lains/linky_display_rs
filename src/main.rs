extern crate teleinfo_nom;
extern crate serialport;
use chrono::{DateTime, Local};

pub struct TeleinfoCachedPower {
    power: i32,
    refreshed: bool,
    changed: bool,
}

impl TeleinfoCachedPower {
    pub fn set(&mut self, power: i32) {
        self.refreshed = true;
        if self.power != power {
            self.changed = true;
        }
        self.power = power;
    }
    pub fn new() -> TeleinfoCachedPower {
        TeleinfoCachedPower {
            power: 0,
            refreshed: false,
            changed: false,
        }
    }
}

pub struct TeleinfoCache {
    inst_power: TeleinfoCachedPower,
    max_power: TeleinfoCachedPower,
}

impl TeleinfoCache {
    pub fn set_inst_power(&mut self, inst_power: i32) {
        self.inst_power.set(inst_power);
        if self.max_power.power < inst_power {
            self.max_power.power = inst_power;
        }
    }
    pub fn new() -> TeleinfoCache {
        TeleinfoCache { 
            inst_power: TeleinfoCachedPower::new(),
            max_power: TeleinfoCachedPower::new(),
        }
    }
}

fn main() {
    let mut tic_cache = TeleinfoCache::new();
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    let serial_port_name = "/dev/ttyvirtual0";
    let port_builder = serialport::new(serial_port_name, 9600);
    let mut stream = port_builder.open().expect(format!("Failed to open port {}", serial_port_name).as_str());
    //let mut stream = File::open("stream_standard_complete.txt").unwrap(); // Test version using a local serial dump (needs use std::fs::File; at top)
    //let msg;
    //let parse_done=false;
    let mut remain = "".to_string();
    loop {
        let msg1;
        (remain, msg1) = teleinfo_nom::get_message(&mut stream, remain).unwrap();
        if let Some(power) = msg1.get_value("SINSTS".to_string()) {
            match power.value.parse::<i32>() {
                Ok(power) => {
                    let now: DateTime<Local> = Local::now();
                    tic_cache.set_inst_power(power);
                    println!("At {}, SINSTS={}W", now.format("%H:%M:%S"), power);
                },
                Err(e) => { println!("While parsing SINST: {}", e) },
            };
        }
    }
    //let (remain, msg2) = teleinfo_nom::get_message(&mut stream, remain).unwrap()
}
