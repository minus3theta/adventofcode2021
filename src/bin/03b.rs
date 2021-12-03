use adventofcode2021::get_stdin;
use itertools::Itertools;

fn count_bit(nums: &[Vec<char>], index: usize) -> (usize, usize) {
    let mut zeros = 0;
    let mut ones = 0;
    for num in nums {
        if num[index] == '0' {
            zeros += 1;
        } else {
            ones += 1;
        }
    }
    (zeros, ones)
}

fn filter(nums: Vec<Vec<char>>, index: usize, choice: char) -> Vec<Vec<char>> {
    nums.into_iter().filter(|v| v[index] == choice).collect()
}

fn oxygen(mut nums: Vec<Vec<char>>) -> Vec<char> {
    for index in 0.. {
        if nums.len() == 1 {
            return nums.pop().unwrap();
        }
        let (z, o) = count_bit(&nums, index);
        let choice = if o >= z { '1' } else { '0' };
        nums = filter(nums, index, choice);
    }
    unreachable!()
}

fn co2(mut nums: Vec<Vec<char>>) -> Vec<char> {
    for index in 0.. {
        if nums.len() == 1 {
            return nums.pop().unwrap();
        }
        let (z, o) = count_bit(&nums, index);
        let choice = if z <= o { '0' } else { '1' };
        nums = filter(nums, index, choice);
    }
    unreachable!()
}

fn parse_binary(num: &[char]) -> i32 {
    let mut ret = 0;
    for d in num {
        ret <<= 1;
        if d == &'1' {
            ret += 1;
        }
    }
    ret
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let input = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

    let ox = parse_binary(&oxygen(input.clone()));
    let co2 = parse_binary(&co2(input));
    println!("{}", ox * co2);

    Ok(())
}
