

extern crate debug;
use std::owned::Box;

enum List {
    Cons(i32, Box<List>),
    Nil
}


fn preprend(value: i32, lst: List) -> List {
    Cons(value, box lst)
}


fn eq(xs: &List, ys: &List) -> bool {
    match (xs, ys) {
        (&Nil, &Nil) => true,
        (&Cons(x, box ref next_xs), &Cons(y, box ref next_ys))
        if x == y => eq(next_xs, next_ys) ,
        _ => false
    }
}


fn main() {

    let mut lst = Nil;
    lst = preprend(10, lst);
    lst = preprend(20, lst);
    lst = preprend(30, lst);
    lst = preprend(40, lst);
    
    println!("lst: {:?}", lst);
    println!("eq: {:?}", eq(&Cons(10, box Cons(20, box Nil)), &Cons(10, box Cons(20, box Nil))))
    println!("eq: {:?}", eq(&Cons(10, box Cons(20, box Nil)), &Cons(10, box Cons(10, box Nil))))
    println!("eq: {:?}", eq(&Cons(10, box Cons(20, box Nil)), &Nil))
}
