use near_sdk::log;
use rand::Rng;

const SIZE: i32 = 22;
const HALF_SIZE: i32 = SIZE / 2;
const ONE: i32 = 1;
const SCHEMA: [&str; 5] = ["🟣", "🟡️️", "⚫️", "⭕️", "🔘"];

pub fn generate(seed: i32) -> String {
    let mut output = String::new();

    let a = if seed == 0 {
        let random = random_num();
        log!("\n\n\nCall claim_my_design with the seed number {random} to claim it.\n");
        random
    } else {
        seed
    };

    let modulus = (a % 11) + 5;

    for i in 0..SIZE {
        let mut y = 2 * (i - HALF_SIZE) + 1;

        y = if a % 3 == 1 {
            -y
        } else {
            y.abs()
        };

        y *= a;

        for j in 0..SIZE {
            let mut x = 2 * (j - HALF_SIZE) + 1;

            if a % 2 == 1 {
                x = x.abs()
            }

            x *= a;

            let v = (((x * y) / ONE) as i32).abs() % modulus;

            let value = if v < 5 {
                SCHEMA.get(v as usize).unwrap().to_string()
            } else {
                String::from("⚪️")
            };

            output.push_str(&value);
        }

        output.push('\n');
    }

    output
}

fn random_num() -> i32 {
    let mut rng = rand::thread_rng();

    rng.gen()
}
