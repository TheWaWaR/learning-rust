// cargo-deps: rand

// cargo script dining_philosophers.rs

extern crate rand;
use rand::Rng;                  // Use the Rng trait.

fn main() {
    struct Philosopher {
        name: String,
        age: u32
    }

    impl Philosopher {
        fn new(name: &str) -> Philosopher {
            Philosopher {
                name: name.to_string(),
                age: rand::thread_rng().gen_range(20, 80)
            }
        }
    }
    let p1 = Philosopher::new("Baruch Spinoza");
    let p2 = Philosopher::new("Gilles Deleuze");
    let p3 = Philosopher::new("Karl Marx");
    let p4 = Philosopher::new("Friedrich Nietzsche");
    let p5 = Philosopher::new("Michel Foucault");

    for p in vec![p1, p2, p3, p4, p5] {
        match p {
            Philosopher{ name, age } => {
                println!("name = {}, age = {}", name, age)
            }
        }
    }
}
