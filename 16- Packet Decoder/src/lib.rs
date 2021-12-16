use std::io::{self, BufRead};

use bitvec::prelude::*;

type Bits = BitSlice<Msb0, u8>;

#[derive(Debug)]
pub enum Payload {
    Literal(usize),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl Payload {
    fn value(&self) -> usize {
        match self {
            Payload::Literal(value) => *value,
            Payload::Sum(packets) => packets.iter().map(Packet::value).sum(),
            Payload::Product(packets) => packets.iter().map(Packet::value).product(),
            Payload::Minimum(packets) => packets.iter().map(Packet::value).min().unwrap(),
            Payload::Maximum(packets) => packets.iter().map(Packet::value).max().unwrap(),
            Payload::GreaterThan(packets) => if packets[0].value() > packets[1].value() { 1 } else { 0 },
            Payload::LessThan(packets) => if packets[0].value() < packets[1].value() { 1 } else { 0 },
            Payload::EqualTo(packets) => if packets[0].value() == packets[1].value() { 1 } else { 0 },
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    pub version: u8,
    pub tag: u8,
    pub payload: Payload,
}

impl Packet {
    pub fn value(&self) -> usize {
        self.payload.value()
    }
}

fn consume<'a, 'b>(bits: &'a mut &'b Bits, len: usize) -> Result<&'b Bits, ()> {
    let ret = bits.get(0..len).ok_or(())?;
    *bits = &bits[len..];
    Ok(ret)
}

fn parse_literal(mut bits: &mut &Bits) -> Result<usize, ()> {
    eprintln!("Parsing literal...");
    let mut value: BitVec<Msb0, usize> = BitVec::new();
    while {
        let cur = consume(&mut bits, 5)?;
        let next = cur[0];
        value.extend(&cur[1..5]);
        next
    } {}
    Ok(value.load_be())
}

fn parse_subpackets(mut bits: &mut &Bits) -> Result<Vec<Packet>, ()> {
    let mut packets = Vec::new();

    let length_tid = consume(&mut bits, 1)?[0];
    match length_tid {
        false => {
            let total_length = consume(&mut bits, 15)?.load_be::<usize>();
            eprintln!("Parsing subpackets with size {}...", total_length);

            let mut subbits = consume(&mut bits, total_length)?;
            while subbits.len() > 0 {
                packets.push(parse_packet(&mut subbits)?);
            }
        },
        true => {
            let packet_count = consume(&mut bits, 11)?.load_be::<usize>();
            eprintln!("Parsing {} subpackets...", packet_count);

            for _ in 0..packet_count {
                packets.push(parse_packet(&mut bits)?);
            }
        },
    }

    Ok(packets)
}

pub fn parse_packet(mut bits: &mut &Bits) -> Result<Packet, ()> {
    let version = consume(&mut bits, 3)?.load_be::<u8>();
    let tag = consume(&mut bits, 3)?.load_be::<u8>();
    eprintln!("Parsing packet with version {}, tag {}...", version, tag);

    let payload = match tag {
        4 => Payload::Literal(parse_literal(&mut bits)?),
        _ => {
            let subpackets = parse_subpackets(&mut bits)?;
            match tag {
                0 => Payload::Sum(subpackets),
                1 => Payload::Product(subpackets),
                2 => Payload::Minimum(subpackets),
                3 => Payload::Maximum(subpackets),
                5 => Payload::GreaterThan(subpackets),
                6 => Payload::LessThan(subpackets),
                7 => Payload::EqualTo(subpackets),
                _ => panic!("Internal error!"),
            }
        },
    };

    Ok(Packet { version, tag, payload, })
}

pub fn parse_stdin() -> BitVec<Msb0, u8> {
    BitVec::from_vec(hex::decode(io::stdin().lock().lines().flatten().collect::<Vec<String>>().join("")).expect("Invalid hex string!"))
}

pub fn parse_string(data: &str) -> BitVec<Msb0, u8> {
    BitVec::from_vec(hex::decode(data).expect("Invalid hex string!"))
}
