

extern crate debug;
use std::owned::Box;


enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}


fn preprend<T>(value: T, lst: List<T>) -> List<T> {
    // 参数中的 lst 是shadow copy的, 返回值也是 shadow copy的,
    // shadow copy 具体是什么含意有待认识
    Cons(value, box lst)
}


fn main() {
    let mut lst: List<f64> = Nil;
    lst = preprend(0.1, lst);
    lst = preprend(1.2, lst);
    lst = preprend(2.3, lst);
    lst = preprend(3.4, lst);
    
    println!("lst: {:?}", lst);
}
