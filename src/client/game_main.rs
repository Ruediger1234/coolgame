use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use laminar::Packet;

pub fn in_temp(in_rx: mpsc::Receiver<Packet>) {
    loop {
        for i in &in_rx {
            println!("Inmcoming: {:?}", i.payload());
        }
    }
}

pub fn out_temp(out_tx: mpsc::Sender<Packet>) {
    let mut counter = 0;

    loop {
        out_tx.send(Packet::reliable_unordered("127.0.0.1:1919".parse().unwrap(), vec![counter])).unwrap();
        counter += 1;
        thread::sleep(Duration::from_secs(1));
    }
}