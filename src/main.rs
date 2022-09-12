extern crate teleinfo_nom;
extern crate serialport;
use chrono::{DateTime, Local};
use std::{thread, time};

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
    let serial_port_name = "/tmp/ttyvirtual0";
    let port_builder = serialport::new(serial_port_name, 9600).timeout(time::Duration::from_millis(10));
    let mut stream = port_builder.open().expect(format!("Failed to open port {}", serial_port_name).as_str());
    let _ = stream.clear(serialport::ClearBuffer::All);
    //let mut stream = File::open("stream_standard_complete.txt").unwrap(); // Test version using a local serial dump (needs use std::fs::File; at top)
    let mut remain = "".to_string();
    loop {
        let pending_bytes_count = stream.bytes_to_read().ok();
        /* We work around the infinite loop inside teleinfo_nom::get_message() by making sure we sleep long enough to have sufficient incoming bytes */
        /* We know that the previous call to teleinfo_nom::get_message() only yields at the end of the previous frame, so the remain buffer is empty or nearly */
        if let Some(incoming_bytes_count) = pending_bytes_count {
            let incoming_buf_sz = incoming_bytes_count + u32::try_from(remain.len()).unwrap_or_default();
            if incoming_buf_sz <= 1024 { /* FIXME: 1024 is adapted to standard TIC, not legacy one */
                let delay_bytes = 1024 - incoming_buf_sz;
                let delay_ms = (1000 * u64::from(delay_bytes)) / (9600 / 8);   /* Roughly estimate how many ms to wait for in order to fill-in a total of 128 bytes */
                println!("Waiting {}ms", delay_ms);
                thread::sleep(time::Duration::from_millis(delay_ms));
            }
        }
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
}
