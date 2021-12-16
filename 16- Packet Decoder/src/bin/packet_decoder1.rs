use packet_decoder::{Packet, Payload, parse_packet, parse_stdin};

fn version_sum(packet: &Packet) -> u32 {
    match &packet.payload {
        Payload::Literal(_) => packet.version as u32,
        Payload::Sum(subpackets) |
        Payload::Product(subpackets) |
        Payload::Minimum(subpackets) |
        Payload::Maximum(subpackets) |
        Payload::GreaterThan(subpackets) |
        Payload::LessThan(subpackets) |
        Payload::EqualTo(subpackets) => {
            packet.version as u32 + subpackets.iter().map(version_sum).sum::<u32>()
        },
    }
}

fn main() {
    let bits = parse_stdin();
    println!("Bits: {}", bits);
    let mut remaining_bits = &bits[..];
    let packet = parse_packet(&mut remaining_bits).expect("Packet parsing error!");
    println!("Packet: {:?}", packet);
    println!("Version sum: {}", version_sum(&packet));
}

#[cfg(test)]
mod tests {
    use super::*;
    use packet_decoder::parse_string;

    fn sum(hex: &str) -> u32 {
        version_sum(&parse_packet(&mut &parse_string(hex)[..]).unwrap())
    }

    #[test]
    fn test_version_sum_16() {
        assert_eq!(sum("8A004A801A8002F478"), 16);
    }

    #[test]
    fn test_version_sum_12() {
        assert_eq!(sum("620080001611562C8802118E34"), 12);
    }

    #[test]
    fn test_version_sum_23() {
        assert_eq!(sum("C0015000016115A2E0802F182340"), 23);
    }

    #[test]
    fn test_version_sum_31() {
        assert_eq!(sum("A0016C880162017C3686B18A3D4780"), 31);
    }
}
