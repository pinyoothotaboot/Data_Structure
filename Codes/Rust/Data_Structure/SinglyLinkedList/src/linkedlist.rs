


#[derive(Clone)]
enum Node {
    Node(Box<LinkedList>),
    Nil,
}

#[derive(Clone)]
pub struct LinkedList{
    pub data : u32,
    next : Node,
}

pub trait LinkedListInterface {

    fn new() -> Self;

    fn insert(&mut self,element : u32);

    fn delete(&mut self,element : u32);

    fn update(&mut self,index : usize , element : u32);

    fn search(&self,index : u32);

    fn list(&self);

    fn count(&self) -> u32;
}

impl LinkedListInterface for LinkedList{

    /**
     * Function : new
     * @sync
     * About : Initial linked list
     */
    fn new() -> Self {
        Self {data : 0 , next : Node::Nil}
    }

    /**
     * Function : insert
     * @sync
     * About : Add new element to linked list
     * Param :
     *      - element : {u32}
     */
    fn insert(&mut self,element : u32) {

        // Loop next
        match self.next {

            // (2) ---> New node
            Node::Node(ref mut next_node) => {
                next_node.insert(element);
            }

            // (1) ---> Nil
            Node::Nil => {
                let node = Self {
                    data : element,
                    next : Node::Nil
                };

                // (1) ---> (2) ---> Nill
                self.next = Node::Node(Box::new(node));
                println!("Insert {:?} successed.",element);
            }
        }
    }

    /**
     * Function : delete
     * @sync
     * About : Delete element from linked list
     * Param :
     *      - element : {u32}
     */
    fn delete(&mut self,element : u32) {
        // Loop next
        match self.next {
            Node::Node(ref mut next_node) => {
                
                // Check node.value equal element
                if next_node.data == element {
                    // Delete current next node
                    println!("Delete element : {:?} successed",next_node.data);

                    // Copy next node to current node
                    self.next = next_node.next.clone();
                } else {
                    next_node.delete(element);
                }
            }

            Node::Nil => {
                if self.data == element {
                    self.data = 0;
                } else {
                    println!("Element : {:?} does not exist in the linked list",element);
                }
            }
        }
    }

    /**
     * Function : update
     * @sync
     * About : Edit element from index of linked list
     * Param :
     *      - index : {usize}
     *      - element : {u32}
     */
    fn update(&mut self,index : usize , element : u32) {
        let mut idx = 0;
        let mut self_node = self;

        // Check if index is 0
        if idx == index {
            self_node.data = element;
        }

        // Loop from 0 to index
        for idx in 0..index {
            match self_node.next {
                Node::Node(ref mut next_node) => {
                    // current node =  next node
                    self_node = next_node;
                }
                Node::Nil => {}
            }
        }

        self_node.data = element;
        println!("Update element : {:?} to index : {:?} successed.",element,index);
    }

    /**
     * Function : list
     * @sync
     * About : Display data in liked list
     */
    fn list(&self) {
        if self.data ==0 {
            println!("The linked list has empty!.");
        }

        println!("Element is : {:?}",self.data);

        match self.next {
            Node::Node(ref next_node) => {
                next_node.list();
            }
            Node::Nil => {}
        }
    }

    /**
     * Function : search
     * @sync
     * About : Search data in linked list by index of linked list
     * Param : 
     *      - index : {u32}
     */
    fn search(&self,index : u32) {
        if self.count() < index {
            panic!("Index out range!.");
        }

        let mut self_node = self;

        if index == 0 {
            println!("Found element of first is : {:?}",self.data);
        } else {
            for idx in 0..index {
                match self_node.next {
                    Node::Node(ref next_node) => {
                        self_node = next_node;
                    }
                    Node::Nil => {}
                }
            }
            println!("Found element of index : {:?} is {:?}",index,self_node.data);
        }
    }

    /**
     * Function : count
     * @sync
     * About : Get length of linked list
     * Return : {u32}
     */
    fn count(&self) -> u32 {
        match self.next {
            Node::Node(ref new_node) => 1 + new_node.count(),
            Node::Nil => 0
        }
    }
}