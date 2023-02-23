
pub struct Stack {
    pub items : Vec<u32>
}

pub trait StackInterface {
    fn new() -> Self;

    fn is_empty(&self) -> bool;

    fn push(&mut self,item : u32);

    fn pop(&mut self) -> u32;

    fn size(&self) -> usize;
}

impl StackInterface for Stack {

    /**
     * Function : new
     * @sync
     * About : Initial new stack
     */
    fn new() -> Self {
        Self {items : Vec::new()}
    }

    /**
     * Function : is_empty
     * @sync
     * About : Check stack items is empty
     * Return {Boolean}
     */
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /**
     * Function : push
     * @sync
     * About : Add item to stack items
     * Param : 
     *      - item : {u32}
     */
    fn push(&mut self,item : u32) {
        self.items.push(item);
    }

    /**
     * Function : pop
     * @sync
     * About : Get first item from stack item
     * Return : {u32}
     */
    fn pop(&mut self) -> u32 {
        if self.is_empty() {
            panic!("Stack items has empty!.");
        }

        let last = self.size() - 1;
        let top_item = self.items[last];
        self.items.remove(last);

        top_item
    }

    /**
     * Function : size
     * @sync
     * About : Get size of items
     * Return : {usize}
     */
    fn size(&self) -> usize {
        self.items.len()
    }
}