use std::io::{self, Read};

pub fn get_stdin() -> io::Result<String> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    lock.read_to_string(&mut input)?;
    Ok(input.trim().into())
}
