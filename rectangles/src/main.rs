#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        &self.width >= &rect.width && &self.height >= &rect.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = (30, 50);
    let rect1 = Rectangle { width: 30, height: 50 };
    
   println!("The area is: {}", area1(30, 50));
   println!("The area is: {}", area2(rect));
   println!("The area is: {}", area3(&rect1));
   println!("The area is: {}", rect1.area());
   println!("Rect1 can hold rect2: {}", rect1.can_hold(&Rectangle { width: 20, height: 25}));
   println!("This is an square: {:#?}", Rectangle::square(10));

   println!("Rectangle Object: {:#?}", rect1)
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangel: &Rectangle) -> u32 {
    rectangel.width * rectangel.height
}