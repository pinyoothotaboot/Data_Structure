
// Struct of queue
pub struct Queue {
    pub limit : usize,
    pub items : Vec<u32>
}

// Interface of queue
pub trait QueueInterface {
    fn new(limit : usize) -> Self;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;

    fn peek(&self) -> u32;

    fn enqueue(&mut self,item : u32);

    fn dequeue(&mut self) -> u32;
}

// Implement queue
impl QueueInterface for Queue {

    /**
     * Function : new
     * @sync
     * About : Initial queue
     * Param :
     *      - limit : {usize}
     * Return {Queue}
     */
    fn new(limit : usize) -> Self {
        Self {limit : limit , items : Vec::new()}
    }

    /**
     * Function : is_empty
     * @sync
     * About : Check if the queue is empty
     * Return : {Boolean}
     */
    fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    /**
     * Function : is_full
     * @sync
     * About : Check if the queue is full
     * Return : {Boolean}
     */
    fn is_full(&self) -> bool {
        self.items.len() == self.limit
    }

    /**
     * Function : peek
     * @sync
     * About : Get the value of the front of the queue without removing it
     * Return : {u32}
     */
    fn peek(&self) -> u32 {
        if self.is_empty() {
            panic!("Queue is empty!.");
        }

        self.items[0]
    }

    /**
     * Function : enqueue
     * @sync
     * About : Add an element to the end of the queue
     * Param :
     *      - item : {u32}
     */
    fn enqueue(&mut self,item : u32) {
        if self.is_full() {
            panic!("Queue has fulled!.,Need dequeue..");
        }

        self.items.push(item);
    }

    /**
     * Function : dequeue
     * @sync
     * About : Remove an element from the front of the queue
     * Return : {u32}
     */
    fn dequeue(&mut self) -> u32 {
        if self.is_empty() {
            panic!("Queue is empty!.");
        }

        let first = 0;
        let front = self.items[first];
        self.items.remove(first);
        front
    }
}