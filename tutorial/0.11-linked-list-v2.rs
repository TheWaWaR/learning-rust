

extern crate debug;
use std::owned::Box;


enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}


fn preprend<T>(value: T, lst: List<T>) -> List<T> {
    // 参数中的 lst 是shadow copy的, 返回值也是 shadow copy的,
    // shadow copy 具体是什么含意有待认识
    // >> However, this will cause both lists to be moved into the function
    // 说明参数传递会传递 Ownership .
    Cons(value, box lst)
}


fn eq<T: PartialEq>(xs: &List<T>, ys: &List<T>) -> bool {
    // let xl = *xs;  ==> 这样是不合法的,因为 *xs 需要获得 xs 的 Ownership, 但是在这里没有这个能力.
    match (xs, ys) {
        (&Nil, &Nil) => true,
        (&Cons(ref a, box ref next_xs), &Cons(ref b, box ref next_ys))
        if a == b => eq(next_xs, next_ys),
        _ => false
    }
}

// 实现 List<T> 自己的 PartiaEq
impl <T: PartialEq> PartialEq for List<T> {
    // 此处 <T: PartialEq> 表示我们需要 T 类型实现了PartialEq 接口
    // impl .. PartialEq for List<T> .. 表示我们当前在实现 List<T> 的 PartialEq 接口
    
    fn eq(&self, ys: &List<T>) -> bool {
        // self 表示一个比较时在左边的 List<T> instance
        match(self, ys) {
            (&Nil, &Nil) => true,
            (&Cons(ref a, box ref next_xs), &Cons(ref b, box ref next_ys))
            if a == b => next_xs == next_ys,
            _ => false
        }
    }
}


fn main() {
    let mut lst: List<f64> = Nil;
    lst = preprend(0.1, lst);
    lst = preprend(1.2, lst);
    lst = preprend(2.3, lst);
    lst = preprend(3.4, lst);
    
    let mut lsta: List<f64> = Nil;
    lsta = preprend(0.2, lsta);
    
    let mut lstb: List<f64> = Nil;
    lstb = preprend(0.1, lstb);
    lstb = preprend(1.2, lstb);
    lstb = preprend(2.3, lstb);
    lstb = preprend(3.4, lstb);

    println!("lst: {:?}", lst);
    println!("eq(&lst, &lsta): {:?}", eq(&lst, &lsta));
    println!("lst == lsta: {:?}", lst == lsta);
    println!("eq(&lst, &lstb): {:?}", eq(&lst, &lstb));
    println!("lst == lstb: {:?}", lst == lstb);
    
    let xs = Cons('c', box Cons('a', box Cons('t', box Nil)));
    let ys = Cons('c', box Cons('a', box Cons('t', box Nil)));
    println!("eq(&xs, &ys): {:?}", eq(&xs, &ys));
    
    println!("eq(&Cons(10i32, box Cons(20, box Nil)), &Cons(10, box Cons(20, box Nil))): {:?}", eq(&Cons(10i32, box Cons(20, box Nil)), &Cons(10, box Cons(20, box Nil))));
    println!("eq(&Cons(10i32, box Cons(20, box Nil)), &Cons(10, box Cons(10, box Nil))): {:?}", eq(&Cons(10i32, box Cons(20, box Nil)), &Cons(10, box Cons(10, box Nil))));
    println!("Cons(10i32, box Cons(20, box Nil)) == Cons(10, box Cons(10, box Nil)): {:?}", Cons(10i32, box Cons(20, box Nil)) == Cons(10, box Cons(10, box Nil)));
    println!("eq(&Cons(10i32, box Cons(20, box Nil)), &Nil): {:?}", eq(&Cons(10i32, box Cons(20, box Nil)), &Nil))
}
