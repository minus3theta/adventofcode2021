use adventofcode2021::get_stdin;

fn decode_one(hex: char) -> Vec<bool> {
    let x = match hex {
        '0'..='9' => hex as u8 - b'0',
        'A'..='F' => hex as u8 - b'A' + 10,
        _ => unreachable!(),
    };
    (0..4).rev().map(|shift| x & (1 << shift) != 0).collect()
}

fn decode(hex: &str) -> Vec<bool> {
    hex.chars()
        .flat_map(|c| decode_one(c).into_iter())
        .collect()
}

type Packet = i32;

fn parse_int<'a>(bits: &mut impl Iterator<Item = &'a bool>) -> i32 {
    let mut ret = 0;
    for &b in bits {
        ret <<= 1;
        if b {
            ret += 1;
        }
    }
    ret
}

fn parse(bits: &mut dyn Iterator<Item = &bool>) -> Packet {
    let version = parse_int(&mut bits.take(3));
    let type_id = parse_int(&mut bits.take(3));
    if type_id == 4 {
        while let Some(&h) = bits.next() {
            let _ = parse_int(&mut bits.take(4));
            if !h {
                break;
            }
        }
        version
    } else {
        let &i = bits.next().unwrap();
        let mut packet = 0;
        if i {
            let l = parse_int(&mut bits.take(11));
            for _ in 0..l {
                let sub_packet = parse(bits);
                packet += sub_packet;
            }
        } else {
            let l = parse_int(&mut bits.take(15)) as usize;
            let mut sub_bits = bits.take(l).peekable();
            while sub_bits.peek().is_some() {
                let sub_packet = parse(&mut sub_bits);
                packet += sub_packet;
            }
        }
        version + packet
    }
}

fn solve(input: &str) -> Packet {
    let input = decode(input);
    parse(&mut input.iter())
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    println!("{}", solve(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits_from_str(bits: &str) -> Vec<bool> {
        bits.chars().map(|c| c == '1').collect()
    }

    #[test]
    fn test_decode() {
        let input = "D2FE28";
        let expected = bits_from_str("110100101111111000101000");

        assert_eq!(decode(input), expected);
    }

    #[test]
    fn test_parse_lit() {
        let input = bits_from_str("11010001111");
        let mut iter = input.iter();
        assert_eq!(parse(&mut iter), 6);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_version_1() {
        let input = "8A004A801A8002F478";
        assert_eq!(solve(input), 16);
    }

    #[test]
    fn test_version_2() {
        let input = "620080001611562C8802118E34";
        assert_eq!(solve(input), 12);
    }

    #[test]
    fn test_version_3() {
        let input = "C0015000016115A2E0802F182340";
        assert_eq!(solve(input), 23);
    }

    #[test]
    fn test_version_4() {
        let input = "A0016C880162017C3686B18A3D4780";
        assert_eq!(solve(input), 31);
    }
}
