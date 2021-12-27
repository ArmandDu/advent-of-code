use itertools::Itertools;
use year2021::Solution;

pub struct Day16;

#[derive(Debug, Clone)]
pub enum Packet {
    Literal {
        version: usize,
        raw: String,
        value: usize,
    },
    Operator {
        version: usize,
        operation: usize,
        packets: Vec<Packet>,
        raw: String,
    },
}

impl Packet {
    pub fn from_str(packet: &str) -> Self {
        const HEADER_SIZE: usize = 6;
        let version = &packet[0..3];
        let type_id = &packet[3..HEADER_SIZE];
        let body = &packet[HEADER_SIZE..];
        let version = usize::from_str_radix(version, 2).unwrap();
        let type_id = usize::from_str_radix(type_id, 2).unwrap();

        match type_id {
            4 => {
                let mut pointer = 0usize;
                let mut sub_packet = String::new();

                while let Some('1') = &body[pointer..(pointer + 1)].chars().next() {
                    sub_packet += &body[(pointer + 1)..(pointer + 5)];
                    pointer += 5;
                }
                sub_packet += &body[(pointer + 1)..(pointer + 5)];

                Packet::Literal {
                    version,
                    value: usize::from_str_radix(&sub_packet, 2).unwrap(),
                    raw: (&packet[0..(HEADER_SIZE + pointer + 5)]).to_string(),
                }
            }
            operation => match &body[0..1] {
                "0" => {
                    let start = 1;
                    let cursor = start + 15;
                    let sub_packets_length =
                        usize::from_str_radix(&body[start..(cursor)], 2).unwrap();

                    let mut size = 0;
                    let mut sub_packets = Vec::new();

                    while size < sub_packets_length {
                        let packet =
                            Self::from_str(&body[(cursor + size)..(cursor + sub_packets_length)]);

                        size += packet.size();

                        sub_packets.push(packet);
                    }

                    Packet::Operator {
                        version,
                        operation,
                        raw: (&packet[0..(HEADER_SIZE + cursor + sub_packets_length)]).to_string(),
                        packets: sub_packets,
                    }
                }
                "1" => {
                    let start = 1;
                    let cursor = start + 11;
                    let sub_packet_count = usize::from_str_radix(&body[start..cursor], 2).unwrap();

                    let mut sub_packets = Vec::new();
                    let mut size = 0;

                    for _ in 0..sub_packet_count {
                        let packet = Self::from_str(&body[(cursor + size)..]);

                        size += packet.size();

                        sub_packets.push(packet);
                    }

                    Packet::Operator {
                        version,
                        operation,
                        raw: (&packet[0..(HEADER_SIZE + cursor + size)]).to_string(),
                        packets: sub_packets,
                    }
                }
                _ => unreachable!(),
            },
        }
    }

    pub fn decode(&self) -> usize {
        match self {
            Self::Literal { value, .. } => *value,
            Self::Operator {
                operation, packets, ..
            } => match operation {
                0 => packets.iter().map(Self::decode).sum(),
                1 => packets.iter().map(Self::decode).product(),
                2 => packets.iter().map(Self::decode).min().unwrap(),
                3 => packets.iter().map(Self::decode).max().unwrap(),
                5 => (packets[0].decode() > packets[1].decode()) as usize,
                6 => (packets[0].decode() < packets[1].decode()) as usize,
                7 => (packets[0].decode() == packets[1].decode()) as usize,
                _ => 0,
            },
        }
    }

    pub fn version_sum(&self) -> usize {
        match self {
            Self::Literal { version, .. } => *version,
            Self::Operator {
                version, packets, ..
            } => packets.iter().map(Self::version_sum).sum::<usize>() + version,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Literal { raw, .. } => raw.len(),
            Self::Operator { raw, .. } => raw.len(),
        }
    }
}

impl Solution<usize, usize> for Day16 {
    const DAY: u32 = 16;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Packet Decoder";
    type Input = String;

    fn part1(input: &Self::Input) -> Option<usize> {
        let packet = Packet::from_str(input);

        Some(packet.version_sum())
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let packet = Packet::from_str(input);

        Some(packet.decode())
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .trim()
            .chars()
            .map(|c| format!("{:04b}", c.to_digit(16).unwrap_or(0)))
            .join(""))
    }
}
