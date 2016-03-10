
use std::thread;
use std::time;

fn main() {
    println!("Main Start");
    let a = 3;
    let guard = thread::spawn(move || {
        println!("Start {}", a);
        thread::sleep(time::Duration::from_secs(3));
        println!("End {}", a);
    });
    let _ = guard.join();
    println!("Main End");
}
