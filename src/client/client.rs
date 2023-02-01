use laminar::{Packet, Socket};
use std::net::{SocketAddr, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc;

use super::receive_packets;
use super::send_packets;
use super::game_main;

pub fn start_client() {
    let mut socket = Socket::bind("127.0.0.1:1918").unwrap();

    let (msg_out_tx, msg_out_rx) = mpsc::channel::<Packet>();
    let (msg_in_tx, msg_in_rx) = mpsc::channel::<Packet>();

    let packet_sender = socket.get_packet_sender();
    let packet_receiver = socket.get_event_receiver();

    thread::spawn(move || receive_packets::receive_packets(packet_receiver, msg_in_tx));
    thread::spawn(move || send_packets::send_from_channel(packet_sender, msg_out_rx));
    game_main::game_main();

    let poll_loop = thread::spawn(move || socket.start_polling());

    poll_loop.join().unwrap();
}