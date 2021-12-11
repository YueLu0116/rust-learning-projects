fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list{
        if item > largest{  // should use trait
            largest = item;
        }
    }

    largest
}

struct MyPoint<T>{
    x: T,
    y: T,
}

impl<T>MyPoint<T>{
    fn x(&self)->&T{
        &self.x
    }
}

impl MyPoint<f32>{
    fn distance(&self)->f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![1,4,2,3,2];
    let ret = largest(&number_list);
    println!("The largest number is {}", ret);
    let char_list = vec!['a', 'e', 'i', 'o', 'u'];
    let ret = largest(&char_list);
    println!("The largest char is {}", ret);

    let point = MyPoint{x: 5, y: 9};
    println!("x is {}", point.x());
}
