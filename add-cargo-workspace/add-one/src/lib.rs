use rand;

pub fn add_one(x: i32) -> i32{
    x + 1
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn is_work(){
        assert_eq!(3, add_one(2));
    }
}