use std::error::Error;
use std::time::Duration;
use rppal::uart::{Parity, Uart};

const MAX_PACKET_SIZE: usize = 64;

enum PacketType {
    Address,
    Data,
    Ack,
    Nack,
    Error,
}

fn send(address: u8, data: &[u8]) {

}

fn receive(address: u8, bytes: usize) {

}

pub fn initialize_mascp() -> (impl FnMut(u8, &[u8]) -> u8, impl FnMut(u8, &[u8]) -> u8) {
    (|a: u8, b: &[u8]| {
        10
    },
    |a: u8, b: &[u8]| {
        10
    })
}