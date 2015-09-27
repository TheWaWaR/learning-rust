// cargo-deps: rand

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;                  // Use the Rng trait.

/// ================================================================================
/// str                 是不可变的字符串；
/// std::String         是可变的字符串；
/// std::path::Path     用来表示路径，方法和普通字符串不一样，当然独立出来；
/// std::path::PathBuf  这是Path的可变版本；
/// std::ffi::CStr      用于表示由C分配、rust借用的C字符串；
/// std::ffi::CString   用于表示由rust分配、可以传递给C函数使用的C字符串；
/// std::ffi::OsStr     平台相关的字符串，具体看: https://github.com/rust-lang/rust/blob/master/src/libstd/ffi/os_str.rs
/// std::ffi::OsString  这个是上面的可变版本；
/// ================================================================================
/// 总之普通字符串就用str和String，路径就用Path和PathBuf，其他是ffi才需要用到的。算是挺清晰的设计。

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Input your guess:");
        let mut guess = String::new();
        let size = io::stdin().read_line(&mut guess).ok().expect("Faild read input!");
        println!("Size: {}", size);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error paser: {}", e);
                continue
            }
        };
        // println!("Your guess: [{}], secret number: [{}]", guess, secret_number);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("You win!!! secret number={}", secret_number);
                break
            }
        }
    }
}
