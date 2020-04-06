use std::io;
use std::collections::HashMap;

fn main() {
    mean_test();
    manage_comp_io();
}

fn manage_comp_io() {
    let mut departments : HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();


    while !input.eq("e") {
        input.clear();

        println!("'a (name) (department)' to add someone\n'l' to list all employees\n'e' to exit");
        
        io::stdin().read_line(&mut input).expect("Failed to read stdin");
        input.truncate(input.len() - 1);

        println!("");
        match input[0..1].as_ref() {
            "l" => {
                println!("Employees: {:?}", departments)
            }
            "a" => {
                // let i : String = input.deref();
                let mut arguments:Vec<&str> = input.split(" ").collect();
               
                arguments.retain(|&arg| arg != "");

                if arguments.len() != 3 {
                    println!("No 2 arguments given");
                    continue
                }

                let employees = departments.entry(String::from(arguments[2])).or_insert(vec![]);

                employees.push(String::from(arguments[1]));

                println!("Adding {} to {}", arguments[1], arguments[2]);
            }
            "e" => {
                println!("Exiting Program");
                continue
            }
            _ => {
                println!("Unrecognized Command {}", input);
            }
        }
        println!("");
    }
}


fn mean_test() {
   let numbers1 = vec![4, 5, 6, 2, 5, 3];
   let numbers2: Vec<i32> = vec![-2, 23, 1, -50];
   let numbers3 = vec![100, 2, 4, 0, 1, 200];

   println!("Average: {}", calculate_mean(&numbers1));
   println!("Average: {}", calculate_mean(&numbers2));
   println!("Average: {}", calculate_mean(&numbers3));
}

fn calculate_mean(list: &Vec<i32>) -> i32 {
    let mut total = 0;
    for val in list {
        total += val;
    }
    total/list.len() as i32
}