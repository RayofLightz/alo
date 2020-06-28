pub use rand::prelude::*;

macro_rules! errcall {
    ($res:expr, $parent:expr, $input:expr) => {
        match $res {
            Ok(string) => string.to_string(),
            Err(_err) => $parent($input)
        }
    };
}

macro_rules! bytes {
    ($string:expr) => {&mut $string.as_bytes().to_vec()};
}

macro_rules! random_index {
    ($indexable:expr) => {rand::random::<usize>() % $indexable.len()}
}

macro_rules! bytes_to_string {
    ($bytes:expr) => {std::str::from_utf8($bytes)};
}

pub fn shiftright(input: &str) -> String {
    let byts = bytes!(input);
    let index = random_index!(byts);
    let shift = rand::random::<u8>() % 4;
    byts[index] = byts[index] >> shift;
    let res = bytes_to_string!(byts);
    let ret = errcall!(res, shiftright, input);
    return ret
}

pub fn shiftleft(input: &str) -> String {
    let byts = bytes!(input);
    let index = random_index!(byts);
    let shift = rand::random::<u8>() % 4;
    byts[index] = byts[index] << shift;
    let res =  bytes_to_string!(byts);
    let ret = errcall!(res, shiftleft, input);
    return ret
}

pub fn random_insert(input: &str) -> String {
    let byts = bytes!(input);
    let index = random_index!(byts);
    let rand_char = rand::random::<u8>();
    byts.insert(index, rand_char);
    let res = bytes_to_string!(byts);
    let ret = errcall!(res, random_insert, input);
    return ret
}

pub fn bitflip(input: &str) -> String {
    let byts = bytes!(input);
    let index = random_index!(byts);
    let bit = 1 << (rand::random::<u8>() % 7);
    byts[index] = byts[index] ^ bit;
    let res = bytes_to_string!(byts);
    let ret = errcall!(res, bitflip, input);
    return ret
}

pub fn generate_mutation(input: &str) -> String {
    let choice = rand::random::<u8>() % 4;
    let round = match choice {
        0 => shiftleft(input),
        1 => shiftright(input),
        2 => random_insert(input),
        3 => bitflip(input),
        _ => generate_mutation(input)
    };
    match rand::random::<u8>() % 2 {
        1 => generate_mutation(&round),
        _ => return round
    }
}
