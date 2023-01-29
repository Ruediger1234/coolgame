use laminar::{Packet, Socket};
use std::thread;
use std::net::SocketAddr;
use crossbeam_channel::Sender;

pub fn start_server() {
    let mut socket = Socket::bind("127.0.0.1:1919").unwrap();
    let packet_recv = socket.get_event_receiver();
    let packet_sender = socket.get_packet_sender();

    let _thread = thread::spawn(move || socket.start_polling());

    println!("Setup complete! Server running!");
    loop {
        let received_event = packet_recv.recv();

        match received_event {
            Ok(event) => {
                match event {
                    laminar::SocketEvent::Connect(addr) => connect(addr),
                    laminar::SocketEvent::Disconnect(addr) => disconnect(addr),
                    laminar::SocketEvent::Packet(packet) => packet_received(packet, &packet_sender),
                    laminar::SocketEvent::Timeout(addr) => timeout(addr),
                }
            },
            Err(error) => {
                println!("An error occured while receiving a package: {:?}", error);
            },
        }
    }
}

fn connect(addr: SocketAddr) {
    println!("User connected: {}", addr.to_string())
}

fn disconnect(addr: SocketAddr) {
    println!("User disconnected: {}", addr.to_string())
}

fn packet_received(packet: Packet, sender: &Sender<Packet>) {
    println!("Packet received from: {}", packet.addr().to_string());
    println!("Packet content: {:?}", packet.payload());
    sender.send(packet).unwrap();
}

fn timeout(addr: SocketAddr) {
    println!("User timed out: {}", addr.to_string())
}