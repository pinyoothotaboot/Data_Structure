mod queue;

use crate::queue::{Queue,QueueInterface};

fn main() {
    let mut queue : Queue = QueueInterface::new(10);

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.enqueue(4);
    queue.enqueue(5);

    let item : u32 = queue.dequeue();
    println!("Item from queue is : {:?}",item);
    
    queue.enqueue(6);
    queue.enqueue(7);
    queue.enqueue(8);
    queue.enqueue(9);
    queue.enqueue(10);
    queue.enqueue(11);
    queue.enqueue(12);
}
