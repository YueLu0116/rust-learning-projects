
fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sqr = Rectangle::square(3);

    println!("The area of the rectangle1 is {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square is {:#?}", sqr);
}

// enable Debug output format
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{

    fn square(size:u32)->Rectangle{
        Rectangle{
            width:size,
            height:size,
        }
    }
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, rect:&Rectangle)->bool{
        self.width >= rect.width && self.height >= rect.height
    }
}