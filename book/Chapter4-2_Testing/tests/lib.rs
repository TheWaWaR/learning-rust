
extern crate adder;

use adder::add_two;

#[test]
fn it_works() {
    assert_eq!(4, add_two(2))
}
