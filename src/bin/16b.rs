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

type Packet = i64;

fn parse_int<'a>(bits: &mut impl Iterator<Item = &'a bool>) -> i64 {
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
    let _version = parse_int(&mut bits.take(3));
    let type_id = parse_int(&mut bits.take(3));
    if type_id == 4 {
        let mut lit = 0;
        while let Some(&h) = bits.next() {
            lit <<= 4;
            lit += parse_int(&mut bits.take(4));
            if !h {
                break;
            }
        }
        lit
    } else {
        let &i = bits.next().unwrap();
        let mut sub_packets = vec![];
        if i {
            let l = parse_int(&mut bits.take(11));
            for _ in 0..l {
                sub_packets.push(parse(bits));
            }
        } else {
            let l = parse_int(&mut bits.take(15)) as usize;
            let mut sub_bits = bits.take(l).peekable();
            while sub_bits.peek().is_some() {
                sub_packets.push(parse(&mut sub_bits));
            }
        }
        match type_id {
            0 => sub_packets.iter().sum(),
            1 => sub_packets.iter().product(),
            2 => *sub_packets.iter().min().unwrap(),
            3 => *sub_packets.iter().max().unwrap(),
            5 => {
                if sub_packets[0] > sub_packets[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if sub_packets[0] < sub_packets[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if sub_packets[0] == sub_packets[1] {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
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
    fn test_eval_1() {
        let input = "C200B40A82";
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_eval_2() {
        let input = "04005AC33890";
        assert_eq!(solve(input), 54);
    }
}
