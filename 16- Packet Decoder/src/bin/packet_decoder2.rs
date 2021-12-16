use packet_decoder::{parse_packet, parse_stdin};

fn main() {
    let bits = parse_stdin();
    println!("Bits: {}", bits);
    let mut remaining_bits = &bits[..];
    let packet = parse_packet(&mut remaining_bits).expect("Packet parsing error!");
    println!("Packet: {:?}", packet);
    println!("Result: {}", packet.value());
}

#[cfg(test)]
mod tests {
    use super::*;
    use packet_decoder::parse_string;

    fn value(hex: &str) -> usize {
        parse_packet(&mut &parse_string(hex)[..]).unwrap().value()
    }

    #[test]
    fn test_sum() {
        assert_eq!(value("C200B40A82"), 3);
    }

    #[test]
    fn test_product() {
        assert_eq!(value("04005AC33890"), 54);
    }

    #[test]
    fn test_minimum() {
        assert_eq!(value("880086C3E88112"), 7);
    }

    #[test]
    fn test_maximum() {
        assert_eq!(value("CE00C43D881120"), 9);
    }

    #[test]
    fn test_greaterthan() {
        assert_eq!(value("D8005AC2A8F0"), 1);
    }

    #[test]
    fn test_lessthan() {
        assert_eq!(value("F600BC2D8F"), 0);
    }

    #[test]
    fn test_equalto() {
        assert_eq!(value("9C005AC2F8F0"), 0);
    }

    #[test]
    fn test_equation() {
        assert_eq!(value("9C0141080250320F1802104A08"), 1);
    }
}
