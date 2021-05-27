// Solving Attempt for:
// https://www.hackerearth.com/practice/data-structures/advanced-data-structures/fenwick-binary-indexed-trees/tutorial/
use std::io;

#[derive(Debug)]
struct FTree {
    bit: Vec<i32>
}

impl FTree {
    fn new (data: &[i32]) -> FTree {
        let mut tree = FTree { bit: vec![0; data.len()] };
        for (index, value) in data.iter().enumerate() {
            tree.update(index + 1, value.clone());
        }
        tree
    }

    // index is 1 based
    fn update(&mut self, index: usize, value: i32) {
        let mut index = index;

        while index <= self.bit.len() {
            // println!("Updating index: {} value: {}", index, value);
            self.bit[index - 1] += value;
            index += {
                let index : i32 = index as i32;
                (index & -index) as usize
            }
        }
    }

    fn query(&self, index: usize) -> i32 {
        let mut index = index;
        let mut sum = 0;
        while index > 0 {
            sum += &self.bit[index - 1];
            index -= {
                let index : i32 = index as i32;
                (index & -index) as usize
            }
        }
        sum
    }
}

fn main() {

    input_handler();

    assert_eq!(gcd(5, 5), 5);
    assert_eq!(gcd(4, 5), 1);
    assert_eq!(gcd(3, 5), 1);
    assert_eq!(gcd(2, 5), 1);
    assert_eq!(gcd(1, 5), 1);

    assert_eq!(f(3), 5);
    assert_eq!(f(4), 8);
    assert_eq!(f(5), 9);

    let mut example_data : Vec<i32> = vec![3, 4, 3].into_iter().map(|x| f(x)).collect();

    let mut tree = FTree::new(&example_data);
    // println!("Data: {:?} | Tree: {:?}", example_data, tree);

    assert_eq!(compute(1, 2, &tree), 13);
    assert_eq!(compute(1, 3, &tree), 18);
    assert_eq!(compute(3, 3, &tree), 5);
    tree = update(&mut example_data, 1, f(4));
    // println!("Data: {:?} | Tree: {:?}", example_data, tree);
    assert_eq!(compute(1, 3, &tree), 21);
    assert_eq!(compute(1, 2, &tree), 16);
}

fn input_handler() {
    let data_size = get_input();
    let data_size: i32 = data_size.parse().unwrap();
    
    let data = get_input();
    let mut data : Vec<i32> = data
        .split_ascii_whitespace()
        .map(|item| f(item.parse().unwrap()))
        .collect();

    let mut tree = FTree::new(&data);

    let queries_size = get_input();
    let queries_size: i32 = queries_size.parse().unwrap();

    for _ in 0..queries_size {
        let query = get_input();
        let mut query = query.split_ascii_whitespace();
        let query_command = query.next().unwrap();

        match query_command {
            "U" => {
                let query_arg1 : usize = query.next().unwrap().parse().unwrap();
                let query_arg2 : i32 = query.next().unwrap().parse().unwrap();
                tree = update(&mut data, query_arg1, f(query_arg2));
            },
            "C" => {
                let query_arg1 : usize = query.next().unwrap().parse().unwrap();
                let query_arg2 : usize = query.next().unwrap().parse().unwrap();
                println!("{}", compute(query_arg1, query_arg2, &tree))
            },
            _ => println!("invalid command")
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();          
    input.pop();
    input
}

fn update(data: &mut [i32], index: usize, value: i32) -> FTree {
    data[index - 1] = value;
    FTree::new(data)
}

fn compute(x: usize, y: usize, tree: &FTree) -> i32 {
    let mut sum =  tree.query(y);

    if x == y {
        sum -= tree.query(y - 1);
    } else if x > 1 {
        sum -= tree.query(x);
    }
    // println!("Sum {}", sum);
    sum % (10 ^ 9 + 7)
}

fn gcd(u: i32, v: i32) -> i32 {
    if u == v {
        return u;
    }

    if u == 0 {
        return v;
    }

    if v == 0 {
        return u;
    }

    if u % 2 == 0 { // u is even
        if v % 2 != 0 { // v is odd
            return gcd(u >> 1, v);
        } else {
            return gcd(u >> 1, v >> 1) << 1;
        }
    } 
    
    if v % 2 == 0 { // u is odd, v is even
        return gcd(u, v >> 1);
    }

    if u > v {
        return gcd((u - v) >> 1, v);
    }

    return gcd((v - u) >> 1, u);
}

fn f(x: i32) -> i32 {
    let mut result = 0;
    for i in 1..x + 1 {
        result += gcd(i, x);
    }

    result
}