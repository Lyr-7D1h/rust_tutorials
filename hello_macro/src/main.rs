use hello_macro::HelloMacro;

struct Test;

impl HelloMacro for Test {
  fn hello_macro() {
    println!("test");
  }
  
}

fn main() {
  Test::hello_macro();
}