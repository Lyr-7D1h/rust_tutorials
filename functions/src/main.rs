fn main() {
    println!("Hello, world!");
    another_function();
    println!("{}", five());
}

fn another_function() {
    println!("Another function.")
}

fn five() -> i32 {
    5
}