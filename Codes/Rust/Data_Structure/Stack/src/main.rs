mod stack;
use crate::stack::{Stack,StackInterface};

fn main() {

    let mut stack : Stack = StackInterface::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Hello, world!");
}
