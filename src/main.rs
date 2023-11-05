extern  crate rand;

use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

fn get_random_number(seed: &u64, index: &u64) -> u64
{
    let mut rng = StdRng::seed_from_u64(*seed);
    for _ in 0..*index {
        rng.gen::<u32>();
    }
    rng.gen_range(0..=100)
}

fn main() {
    println!("Hello, world!");
    for i in 0..=10 {
        let lol = get_random_number(&123456789, &i); // change this ...
        println!("{}", lol);
    }
    
}
