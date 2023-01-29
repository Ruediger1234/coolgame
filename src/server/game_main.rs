use std::sync::mpsc;
use laminar::Packet;

fn temp_pingpong(in_rx: mpsc::Receiver<Packet>, out_tx: mpsc::Sender<Packet>) {
    for i in in_rx {
        out_tx.send(i).unwrap();
    }
}