fn read_bit(data: &[u8], position: usize) -> u8 {
    1 & (data[position >> 2] >> (3 - (position & 3)))
}

fn read_bits(data: &[u8], position: usize, count: usize) -> u64 {
    (0..count).fold(0u64, |v, i| (v << 1) | read_bit(data, position + i) as u64)
}

fn read_literal(data: &[u8], position: usize) -> (u64, usize) {
    let mut literal = 0u64;
    let mut index = position;
    loop {
        let nibble = read_bits(data, index, 5);
        index += 5;
        literal = literal << 4 | (nibble & 0xF);

        if (nibble & 0b10000) == 0 {
            break;
        }
    }
    (literal, index)
}

enum Payload {
    Literal(u64),
    Operation(u64, Vec<Packet>),
}

struct Packet {
    version: u64,
    payload: Payload,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match self.payload {
            Payload::Operation(_, ref packets) => {
                packets.iter().map(Packet::version_sum).sum::<u64>() + self.version
            }
            _ => self.version,
        }
    }

    fn eval(&self) -> u64 {
        const SUM: u64 = 0u64;
        const PRODUCT: u64 = 1u64;
        const MINIMUM: u64 = 2u64;
        const MAXIMUM: u64 = 3u64;
        const GREATER_THAN: u64 = 5u64;
        const LESS_THAN: u64 = 6u64;
        const EQUAL_TO: u64 = 7u64;
        match self.payload {
            Payload::Literal(value) => value,
            Payload::Operation(id, ref packets) => {
                let operands = packets.iter().map(Packet::eval);
                match id {
                    SUM => operands.sum::<u64>(),
                    PRODUCT => operands.product(),
                    MINIMUM => operands.min().unwrap_or_default(),
                    MAXIMUM => operands.max().unwrap_or_default(),
                    GREATER_THAN => {
                        let (lhs, rhs) = operands.clone().zip(operands.skip(1)).next().unwrap();
                        if lhs > rhs {
                            1
                        } else {
                            0
                        }
                    }
                    LESS_THAN => {
                        let (lhs, rhs) = operands.clone().zip(operands.skip(1)).next().unwrap();
                        if lhs < rhs {
                            1
                        } else {
                            0
                        }
                    }
                    EQUAL_TO => {
                        let (lhs, rhs) = operands.clone().zip(operands.skip(1)).next().unwrap();
                        if lhs == rhs {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Unknown operator {id}"),
                }
            }
        }
    }
}

fn read_packet(data: &[u8], mut position: usize) -> (usize, Packet) {
    let version = read_bits(data, position, 3);
    let id = read_bits(data, position + 3, 3);
    position += 6;
    match id {
        4 => {
            let (literal, pos) = read_literal(data, position);
            (
                pos,
                Packet {
                    version,
                    payload: Payload::Literal(literal),
                },
            )
        }
        _ => {
            let subtype = read_bit(data, position);
            position += 1;
            if subtype == 0 {
                let length = read_bits(data, position, 15) as usize;
                position += 15;
                let end = position + length;
                let mut subpackets = Vec::new();
                while position < end {
                    let (pos, packet) = read_packet(data, position);
                    position = pos;
                    subpackets.push(packet);
                }
                (
                    position,
                    Packet {
                        version,
                        payload: Payload::Operation(id, subpackets),
                    },
                )
            } else {
                let count = read_bits(data, position, 11) as usize;
                position += 11;
                let mut subpackets = Vec::new();
                while subpackets.len() < count {
                    let (pos, packet) = read_packet(data, position);
                    position = pos;
                    subpackets.push(packet);
                }
                (
                    position,
                    Packet {
                        version,
                        payload: Payload::Operation(id, subpackets),
                    },
                )
            }
        }
    }
}

pub fn run() {
    let input = include_bytes!("../inputs/day16.txt");

    let nibbles: Vec<u8> = input
        .iter()
        .filter_map(|&ch| {
            if ch >= b'A' {
                Some(ch - b'A' + 10)
            } else if ch >= b'0' {
                Some(ch - b'0')
            } else {
                None
            }
        })
        .collect();
    let (_position, packet) = read_packet(&nibbles, 0);
    let version_sum = packet.version_sum();
    let eval = packet.eval();

    println!("Day 16 part 1: {version_sum}");
    println!("Day 16 part 2: {eval}");
}
