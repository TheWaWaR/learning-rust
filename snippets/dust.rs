// cargo-deps: num_cpus="*"

extern crate num_cpus;

fn test_cpu() {
    println!("num cpus: {}", ::num_cpus::get());
}

fn main() {
    test_cpu();
}
