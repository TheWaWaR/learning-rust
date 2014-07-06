
// Rust 告诉你:
//   内存(或Value)是资源, 变量是资源的持有者, 而且一个资源不能被多个变量同时持有.

fn main() {
    // 枚举
    enum Direction {
        North,
        East,
        South,
        West
    }
    // Rust 中的元组(tuple)
    let d = North;
    let directs: (Direction, Direction, Direction, Direction) = (North, East, South, West);

    enum List<T> {
        Cons(T, ~List<T>),
        Nil
    }
    fn prepend<T>(lst: List<T>, value: T) -> (List<T>) {
        Cons(value, ~lst)
    }
    // 构建一个列表
    let mut xs = Nil;
    xs = prepend(xs, 10);
    xs = prepend(xs, 20);
    xs = prepend(xs, 50);
    // ERROR ==> xs = prepend(xs, 50.2); 这得多复杂啊, 类型推断真牛逼...
    xs = prepend(xs, 70);
    println!("xs: {:?}", xs);
    

    // 可修改的变量
    let mut x = 32;
    if x > 30 { x = 64; }
    println!("x = {}", x);

    println!("hello?");
    println!("{:?} ==> {}", d, d as int);
    println!("==========================");
    // 模式匹配
    match directs {
        (d1, d2, d3, d4) => {
            println!("{:?} ==> {}", d1, d1 as int);
            println!("{:?} ==> {}", d2, d2 as int);
            println!("{:?} ==> {}", d3, d3 as int);
            println!("{:?} ==> {}", d4, d4 as int);
        }
    }
}
