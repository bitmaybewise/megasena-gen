extern crate rand;

fn random_numbers_ordered() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let range = 1..61;
    let mut sample = rand::sample(&mut rng, range, 6);
    sample.sort();
    return sample;
}

fn main() {
    let numbers = random_numbers_ordered();
    for n in numbers {
        print!("{} ", n);
    }
    println!();
}
