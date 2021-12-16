use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Input {
    input: Vec<u8>,
    bit_position: usize,
}

impl Input {
    fn read_bit(&mut self) -> u64 {
        let nibble = self.input[self.bit_position / 4];
        let bit = (nibble >> (3 - self.bit_position % 4)) % 2;
        self.bit_position += 1;

        bit as u64
    }

    fn read_bits(&mut self, bits: u8) -> u64 {
        assert!(bits <= 64);
        let mut result = 0;
        for _ in 0..bits {
            result = result * 2 + self.read_bit();
        }

        result
    }
}

fn parse_input(input: String) -> Input {
    let input = input
        .trim()
        .chars()
        .map(|char| u8::from_str_radix(&String::from(char), 16).unwrap())
        .collect();

    Input {
        input,
        bit_position: 0,
    }
}

#[derive(Debug, Clone)]
enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl Packet {
    fn total_version(&self) -> u64 {
        match self {
            Packet::Literal(version, _) => *version as u64,
            Packet::Operator(version, _, sub_packets) => {
                let version = *version as u64;
                let sub: u64 = sub_packets
                    .iter()
                    .map(|packet| packet.total_version())
                    .sum();
                version + sub
            }
        }
    }

    fn value(&self) -> u64 {
        match self {
            Packet::Literal(_, value) => *value as u64,
            Packet::Operator(_, type_id, sub_packets) => {
                let mut values = sub_packets.iter().map(|packet| packet.value());

                match type_id {
                    0 => values.sum(),
                    1 => values.product(),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),
                    5 => (values.next().unwrap() > values.next().unwrap()) as u64,
                    6 => (values.next().unwrap() < values.next().unwrap()) as u64,
                    7 => (values.next().unwrap() == values.next().unwrap()) as u64,
                    _ => panic!(),
                }
            }
        }
    }
}

fn parse_packet(input: &mut Input) -> Packet {
    let version = input.read_bits(3);
    let type_id = input.read_bits(3);
    if type_id == 4 {
        let mut number = 0;
        loop {
            let bit = input.read_bit();
            number = number * 16 + input.read_bits(4);
            if bit == 0 {
                break;
            }
        }

        return Packet::Literal(version as u8, number);
    }

    let length_type = input.read_bit();
    if length_type == 0 {
        let length = input.read_bits(15);
        let current_bits = input.bit_position;
        let mut sub_packets = vec![];

        while input.bit_position < current_bits + length as usize {
            sub_packets.push(parse_packet(input));
        }

        assert_eq!(input.bit_position, current_bits + length as usize);

        return Packet::Operator(version as u8, type_id as u8, sub_packets);
    } else {
        let count = input.read_bits(11);
        let mut sub_packets = vec![];

        for _ in 0..count {
            sub_packets.push(parse_packet(input));
        }

        return Packet::Operator(version as u8, type_id as u8, sub_packets);
    }
}

fn task1(input: &Input) -> u64 {
    let packet = parse_packet(&mut input.clone());
    packet.total_version()
}

fn task2(input: &Input) -> u64 {
    let packet = parse_packet(&mut input.clone());
    packet.value()
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input1 = parse_input(String::from("8A004A801A8002F478"));
    let input2 = parse_input(String::from("620080001611562C8802118E34"));
    let input3 = parse_input(String::from("C0015000016115A2E0802F182340"));
    let input4 = parse_input(String::from("A0016C880162017C3686B18A3D4780"));

    assert_eq!(task1(&input1), 16);
    assert_eq!(task1(&input2), 12);
    assert_eq!(task1(&input3), 23);
    assert_eq!(task1(&input4), 31);

    let input1 = parse_input(String::from("C200B40A82"));
    let input2 = parse_input(String::from("04005AC33890"));
    let input3 = parse_input(String::from("880086C3E88112"));
    let input4 = parse_input(String::from("CE00C43D881120"));
    let input5 = parse_input(String::from("D8005AC2A8F0"));
    let input6 = parse_input(String::from("F600BC2D8F"));
    let input7 = parse_input(String::from("9C005AC2F8F0"));
    let input8 = parse_input(String::from("9C0141080250320F1802104A08"));

    assert_eq!(task2(&input1), 3);
    assert_eq!(task2(&input2), 54);
    assert_eq!(task2(&input3), 7);
    assert_eq!(task2(&input4), 9);
    assert_eq!(task2(&input5), 1);
    assert_eq!(task2(&input6), 0);
    assert_eq!(task2(&input7), 0);
    assert_eq!(task2(&input8), 1);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 927);
    assert_eq!(task2(&input), 1725277876501);
}
