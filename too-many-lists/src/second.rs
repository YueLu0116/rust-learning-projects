// reference:
// https://rust-unofficial.github.io/too-many-lists/second.html

type Link<T> = Option<Box<Node<T>>>;

pub struct IntoIter<T>(List<T>);

// TIL: add lifetimes only in function and type signatures
pub struct Iter<'a, T>{
    next : Option<&'a Node<T>>,
}

pub struct IterMut<'a, T>{
    next : Option<&'a mut Node<T>>,
}

pub struct List<T>{
    head : Link<T>,
}

struct Node<T>{
    elem: T,
    next: Link<T>,
}

impl<T> List<T>{
    pub fn new() -> Self{
        List {head : None}
    }

    pub fn push(&mut self, elem : T){
        let new_node = Box::new(Node{
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node|{  // TIL!
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self)->Option<&T>{
        // TIL!
        // reference:
        // https://doc.rust-lang.org/std/option/enum.Option.html
        // > as_ref convert &Option<T> to Option<&T>, preserving the original
        // > The map method takes the self argument by value, consuming the original
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }

    pub fn peek_mut(&mut self)->Option<&mut T>{
        self.head.as_mut().map(|node|{
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T>{
        // TIL!
        // reference
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref
        // > as_deref: Converts from Option<T> (or &Option<T>) to Option<&T::Target>
        Iter {next: self.head.as_deref()}
    }

    pub fn iter_mut(& mut self) -> IterMut<T>{
        IterMut{next : self.head.as_deref_mut()}
    }
}

impl<T> Drop for List<T>{
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link{
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>{
        // "0" here is similiar with cpp's std::get<0>
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node|{
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>{
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    #[test]
    fn basics(){
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek(){
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }


}

// review points:
// 1. Use Option's take method to replace mem::replace
// 2. map: match option { None => None, Some(x) => Some(y) }
// 3. Basic generics
// 4. as_ref, as_deref and as_mut methods of Options
// 5. associated types in trait
// 6. tuple structs
// 7. lifetimes, when should I add them?
// 8. Iter, IntoIter, and IterMut
// 9. Copy: i32 is Copy, Option<T> is Copy, &mut is not Copy...