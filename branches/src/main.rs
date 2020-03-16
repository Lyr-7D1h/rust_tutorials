fn main() {
    let mut number = 3;

    if number < 5 {
        println!("true")
    } else {
        println!("false")
    }

    number = if 3 == 2 {
        5
    } else {
        3
    };

    println!("{}", number)
}
