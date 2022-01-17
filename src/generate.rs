use near_sdk::env;
use random::Source;

use crate::log;

pub const SIZE: u64 = 22;
pub const HALF_SIZE: u64 = SIZE / 2;
pub const ONE: u64  = 1;

pub const MAX_SEED: i32 = 144_500_000;


pub fn schema(id: usize) -> String{
    return match id{
        0 => String::from("🟣"),
        1 => String::from("🟡️️"),
        2 => String::from("⚫️"),
        3 => String::from("⭕️"),
        4 => String::from("🔘"),
        x => {panic!("Invalid id for schema ({})", x)},
    }
}

pub fn generate(seed: i32) -> String {
    let mut output: String = String::from("");

    let a: i64;

    if seed > MAX_SEED {
        log(&format!("\n\n\tA value larger than {} will cause an overflow error. Generating a seed instead.\n\n", MAX_SEED));
    }

    // TODO move this to index
    if (seed == 0) || (seed > MAX_SEED) {    
        a = randomNum() as i64;
        log(&format!("\n\n\tCall claimMyDesign with the seed number {} to claim it.\n", a));
    } else {
        a = seed as i64;
        log(&format!("\n\n\tCall claimMyDesign with the seed number {} to claim it.\n", a));
    }

    let (mut x, mut y, mut v): (i64, i64, i64);
    let mut value: String;
    let modulus = (a % 11) + 5;

    for i in 0..(SIZE as i64) {
        y = 2 * (i - HALF_SIZE as i64) + 1;
        if a % 3 == 1 {
            y = -y;
        } else if a % 3 == 2 {
            y = abs(y) as i64;
        }
        y = y * a;
        for j in 0..(SIZE as i64) {
            x = 2 * (j - HALF_SIZE as i64) + 1;
            if a % 2 == 1 {
                x = abs(x) as i64;
            }
            x = x * a;
            v = abs(x * y / ONE as i64) % modulus as i64;

            if v < 5{
                value = schema(v as usize);
            } else {
                value = String::from("⚪️")
            }
            output = format!("{}{}", output, value);
        }
        output= format!("{}\n", output);
    }

    output
}


pub fn abs(n: i64) -> i64 {
    if n >= 0 { return n; };

    -n
}

#[allow(non_snake_case)]
pub fn randomNum() -> u32 {
    // Using crate random for generating random values
    let first_seed: u64 = env::block_index();

    // Use block index as random value
    // Use the same value with inverted bits as the second seed
    let first_seed_inverted: u64 = first_seed.reverse_bits();
    
    // Create the struct with both seeds, then get a random value between 0 and 1
    let rng: f64 = random::Default::new()
        .seed([first_seed, first_seed_inverted])
        .read_f64();
    
    // The random value will be a proportion between 0 and MAX_SEED
    let value: u32 = (MAX_SEED as f64 * rng) as u32;

    value
}