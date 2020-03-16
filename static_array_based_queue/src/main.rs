
fn main() {
    const QUEUE: [i32; 4] = [0; 4];
    let mut pointer = 0;
    const example: [i32; 4] = [
        1,
        2,
        3,
        4
    ];
    for num in example.iter() {
        println!("{}", num)
    }
}


fn enqueue(value:i32) {
    println!("{}", QUEUE)

}

fn dequeue() {

}