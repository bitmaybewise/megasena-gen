extern crate rand;

fn main() {
    let mut rng = rand::thread_rng();
    let range = 1..61;
    let mut sample = rand::sample(&mut rng, range, 6);
    sample.sort();
    for n in sample {
        print!("{} ", n);
    }
    println!();
}
