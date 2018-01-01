use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::{thread, time};
extern crate serial;

fn main() {
    let mut port = serial::open(&env::args().nth(1).unwrap()).unwrap();
    port.write(&[0x00,0x80,0x81]).unwrap();

    let mut void = vec![0;1024];
    while true {
        println!("ping");
        port.write(&[0x00,0x01]).unwrap();
        thread::sleep(time::Duration::from_millis(2000));
    }
}
