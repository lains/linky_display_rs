use std::fs::File;
extern crate teleinfo_nom;

pub struct TeleinfoCachedPower {
    power: i32,
    new: bool,
}

impl TeleinfoCachedPower {
    pub fn set(&mut self, power: i32) {
        self.power = power;
        self.new = true;
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
            inst_power: TeleinfoCachedPower {
                power: 0,
                new: false,
            },
            max_power: TeleinfoCachedPower {
                power: 0,
                new: false
            },
        }
    }
}

fn main() {
    let mut tic_cache = TeleinfoCache::new();
    // Could be a serial port with serialport crate
    let mut stream = File::open("stream_standard_complete.txt").unwrap();
    //let remain;
    //let msg;
    //let parse_done=false;
    //while ()
    let (_remain, msg1) = teleinfo_nom::get_message(&mut stream, "".to_string()).unwrap();
    let current_indices = msg1.get_billing_indices();
    let current_values = msg1.get_values(current_indices);
    for (index,value) in current_values.into_iter() {
        match value {
            Some(val) => println!("store {}: {} in database", index, val),
            None => (),
        }
    }
    if let Some(power) = msg1.get_value("SINSTS".to_string()) {
        match power.value.parse::<i32>() {
            Ok(power) => tic_cache.set_inst_power(power),
            Err(_e) => { /* Parse errors (SINSTS does not convert to i32) are ignored */},
        };
        println!("SINSTS={}!", power.value);
    }
    //let (remain, msg2) = teleinfo_nom::get_message(&mut stream, remain).unwrap()
}
