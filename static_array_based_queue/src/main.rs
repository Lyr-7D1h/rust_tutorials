
fn main() {
    const QUEUE: [i32; 4] = [0; 4];
    let mut pointer = 0;

    fn enqueue(value:i32) {
        pointer += 1;
        QUEUE[pointer];
    }

    fn dequeue() {

    }

    enqueue(3);
}

