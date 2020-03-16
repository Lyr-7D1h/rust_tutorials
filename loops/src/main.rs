fn main() {
    // count_to_thousand()
    liftoff_using_loop();
    liftoff_using_while();
    liftoff_using_for();
}

fn liftoff_using_loop() {
    let mut counter = 3;
    loop {
       println!("{}!", counter);
       counter -= 1; 
       if counter == 0 {
           break
       }
    }
    println!("LIFTOFF")
}

fn liftoff_using_while() {
    let mut counter = 3;
    while counter != 0 {
        println!("{}!", counter);
        counter -= 1;
    }
    println!("LIFTOFF");
}

fn liftoff_using_for() {
    for number in (1..4).rev() {
        println!("{}!", number)
    }
    println!("LIFTOFF")
}

fn count_to_thousand() {
    let mut counter = 0;
    loop {
       counter += 1;
       println!("{}", counter);
       if counter == 1000 {
           break
       }
    }
}