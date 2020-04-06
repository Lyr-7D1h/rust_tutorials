struct Node<'a> {
    value: i32,
    next: &'a Node<'a>
}

struct SinglyLinkedList<'a> {
    head: &'a Node<'a>,
    tail: &'a Node<'a>
}

impl SinglyLinkedList<'_> {
    fn push(&self, node: Node) {

    }
    fn new() {
        SinglyLinkedList {  }
    }
}



fn main() {
    
}
