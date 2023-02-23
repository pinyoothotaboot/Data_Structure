
mod linkedlist;
use crate::linkedlist::{LinkedList,LinkedListInterface};

fn main() {
    let mut linkedlist : LinkedList  = LinkedListInterface::new();

    linkedlist.insert(1);
    linkedlist.insert(2);
    linkedlist.insert(3);
    linkedlist.insert(4);

    let count = linkedlist.count();
    linkedlist.list();
    println!("Linkedlist Count : {:?}",count);

    linkedlist.delete(3);
    let count = linkedlist.count();
    println!("Linkedlist After delete Count : {:?}",count);
    linkedlist.list();

    linkedlist.update(1,99);
    linkedlist.list();

    linkedlist.search(3);
}
