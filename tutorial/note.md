
# Meta information
* Version: 0.11

# List

* 使用 macro 用 *!*, 也叫语法扩展 (Syntax extension)
    let numbers = vec![1i, 2i, 3i];

* 声明本地变量用 *let*
    let numbers = vec![1i, 2i, 3i];

* 有用的 `if`, `else if`... 赋值

    let price =
      if item == "salad" {
         3.50
      } else if item == "muffin" {
         2.25
      } else {
         2.00
      };


* `char` 是4字节的Unicode

* 模式匹配(Pattern matching)

    let my_number = 1;
    match my_number {
      0     => println!("zero"),
      1 | 2 => println!("one or two"),
      3..10 => println!("three to ten"),
      _     => println!("something else")
    }


* 绑定匹配项使用 *@*

    match age {
        a @ 0..20 => println!("{} years old", a),
        _ => println!("older than 21")
    }


* 复杂的枚举, 声明了一个 *Shape* 类型(type), 和两个定义形状的函数(functions)
    enum Shape {
        Circle(Point, f64),
        Rectangle(Point, Point)
    }

    
* 在 Rust 中*()* 意味着 *unit* 或 *void*

* 用 *box* 关键字来声明指向堆(Heap)中的变量

* 当 *task* 失败时 Rust 会调用其中所有 object 的 *destructor* 释放所有占
  用的资源。当一个变量超出了其作用域(*scope*) 时，它的 *destructor* 也
  会被调用。

* Ownership: 一个 object 的 owner 是一个 variable, ownership 包括两部
  分概念 -> 生命周期管理(end lifetime by call destructor) + 可变性管理
  (mutable)。object 和 variable 之间的 Ownership 是递归继承的。

  
