use std::net::SocketAddr;
use std::sync::mpsc;
use laminar::{Packet, SocketEvent};
use crossbeam_channel::Receiver;

pub fn receive_packets(receiver: Receiver<SocketEvent>, in_tx: mpsc::Sender<Packet>) {
    loop {
        let message = receiver.recv();
        match message {
            Ok(message) => {
                match message {
                    laminar::SocketEvent::Connect(addr) => connect(addr),
                    laminar::SocketEvent::Disconnect(addr) => disconnect(addr),
                    laminar::SocketEvent::Packet(packet) => packet_received(packet, &in_tx),
                    laminar::SocketEvent::Timeout(addr) => timeout(addr),
                }
            },
            Err(err) => println!("Error occured while receiving a packet: {:?}", err),
        }
    }
}

fn connect(addr: SocketAddr) {
    println!("User connected: {}", addr.to_string())
}

fn disconnect(addr: SocketAddr) {
    println!("User disconnected: {}", addr.to_string())
}

fn packet_received(packet: Packet, in_tx: &mpsc::Sender<Packet>) {
    in_tx.send(packet).unwrap();
}

fn timeout(addr: SocketAddr) {
    println!("Connection timed out: {}", addr.to_string());
}