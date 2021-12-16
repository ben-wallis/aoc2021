use aoc2021::utils::read_lines;
use std::borrow::Cow;
use std::collections::HashSet;

fn main() {
    let input = include_str!("C:\\rust\\aoc2021\\day16_input.txt");
    let binary = input
        .chars()
        .map(|char| format!("{:04b}", u8::from_str_radix(&char.to_string(), 16).unwrap()))
        .collect();

    println!("{}", binary);

    // Didn't commit for part 1
    // println!("Part 1: {:?}", part2);

    let part2 = solve(binary);
    println!("Part 2: {:?}", part2);
}

fn solve(input: String) -> u64 {
    let mut offset = 0;
    process_packet(&input, &mut offset)
}

fn process_packet(data: &str, offset: &mut usize) -> u64 {
    let packet_version = &data[*offset..*offset + 3];
    let packet_version_int = u64::from_str_radix(packet_version, 2).unwrap();
    let packet_type = &data[*offset + 3..*offset + 6];
    let packet_type_int = u8::from_str_radix(packet_type, 2).unwrap();
    println!(
        "Offset: {} Packet Version: {} ({}) Type: {} ({})",
        offset, packet_version, packet_version_int, packet_type, packet_type_int
    );

    *offset = *offset + 6;

    if packet_type_int == 4 {
        process_literal_value_packet(data, offset)
    } else {
        process_operator_packet(data, offset, packet_type_int)
    }
}

fn process_literal_value_packet(data: &str, offset: &mut usize) -> u64 {
    let mut literal_binary: String = "".to_string();

    loop {
        let more = &data[*offset..=*offset] == "1";
        literal_binary.push_str(&data[*offset + 1..*offset + 5]);
        *offset = *offset + 5;
        if !more {
            break;
        }
    }

    u64::from_str_radix(&literal_binary, 2).unwrap()
}

fn process_operator_packet(data: &str, offset: &mut usize, packet_type: u8) -> u64 {
    let length_type_id = &data[*offset..=*offset];

    *offset = *offset + 1;
    let mut values: Vec<u64> = vec![];
    if length_type_id == "0" {
        let packets_length = usize::from_str_radix(&data[*offset..*offset + 15], 2).unwrap();
        *offset = *offset + 15;
        let end = *offset + packets_length;
        while *offset < end {
            values.push(process_packet(data, offset));
        }
    } else if length_type_id == "1" {
        let sub_packets = u32::from_str_radix(&data[*offset..*offset + 11], 2).unwrap();
        *offset = *offset + 11;
        values = (0..sub_packets)
            .map(|_| process_packet(data, offset))
            .collect();
    } else {
        panic!("Unknown length_type_id: {}", length_type_id);
    }

    #[rustfmt::skip]
    match packet_type {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => { if values[0] > values[1] { 1 } else { 0 } }
        6 => { if values[0] < values[1] { 1 } else { 0 } }
        7 => { if values[0] == values[1] { 1 } else { 0 } }
        _ => unimplemented!(),
    }
}
