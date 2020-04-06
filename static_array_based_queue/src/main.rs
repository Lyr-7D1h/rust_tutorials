// use std::collections::VecDeque;


// struct Queue {
//     data: Vec<u32>,
// }

// impl Default for Queue {
//     fn default() -> Queue { 
//        Queue { data: VecDeque::new() } 
//     }
// }

// impl Queue {
//     fn enqueue(&mut self, value: u32) {
//         self.data.push_back(value);
//         self.end += 1;
//     }

//     fn dequeue(&mut self) -> u32 {
//         self.data.pop_front(self.start)
//     }
// }

// fn main() {
//     let mut queue = Queue{..Default::default()};

//     queue.enqueue(2);
//     queue.dequeue();
// }

