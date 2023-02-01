use std::thread;
use std::net::SocketAddr;
use std::sync::mpsc;
use laminar::{Packet, Socket};
use crossbeam_channel::Sender;

use super::receive_packets;
use super::send_packets;
use super::gameserver_main;


pub fn start_server() {
    let mut socket = Socket::bind("127.0.0.1:1919").unwrap();

    let (msg_out_tx, msg_out_rx) = mpsc::channel::<Packet>();
    let (msg_in_tx, msg_in_rx) = mpsc::channel::<Packet>();

    let packet_receiver = socket.get_event_receiver();
    let packet_sender = socket.get_packet_sender();

    thread::spawn(move || receive_packets::receive_packets(packet_receiver, msg_in_tx));
    thread::spawn(move || send_packets::send_from_channel(packet_sender, msg_out_rx));
    thread::spawn(move || gameserver_main::temp_pingpong(msg_in_rx, msg_out_tx));

    let poll_loop = thread::spawn(move || socket.start_polling());
    println!("Setup complete! Server running!");

    poll_loop.join().unwrap();
}