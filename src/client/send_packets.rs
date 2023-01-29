use std::sync::mpsc;

use crossbeam_channel::Sender;
use laminar::Packet;

pub fn send_from_channel(sender: Sender<Packet>, out_rx: mpsc::Receiver<Packet>) {
    loop {
        for i in &out_rx {
            send_packet(&sender, i);
        }
    }
}

fn send_packet(sender: &Sender<Packet>, packet: Packet) {
    sender.send(packet).unwrap();
}