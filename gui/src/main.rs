use gui::{Screen, Draw};

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>
}

impl Draw for SelectBox {
  fn draw(&self) {
    println!("Drawing SelectBox")
  }
}

fn main() {
  let select_box = SelectBox { width: 2, height: 2, options: vec![String::from("Test")]};

  let screen = Screen {
    components: vec!(Box::new(select_box))
  };
  
  screen.run();
}