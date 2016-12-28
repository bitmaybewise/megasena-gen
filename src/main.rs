extern crate rand;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

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

#[cfg(test)]
quickcheck! {
    fn length_equal_6() -> bool {
        random_numbers_ordered().len() == 6
    }
    fn different_values() -> bool {
        let ns = random_numbers_ordered();
        return ns[0] != ns[1] 
            && ns[1] != ns[2]
            && ns[2] != ns[3]
            && ns[3] != ns[4]
            && ns[4] != ns[5];
    }
    fn ordered_values() -> bool {
        let ns = random_numbers_ordered();
        return ns[0] < ns[1] 
            && ns[1] < ns[2]
            && ns[2] < ns[3]
            && ns[3] < ns[4]
            && ns[4] < ns[5];
    }
}