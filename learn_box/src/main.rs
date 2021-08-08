use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
     &self.0
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    use List::Cons;
    // let list = Cons(1,Cons(2,Cons(3,List::Nil)));

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(List::Nil))))));
    println!("{:?}", list);
    let x = 5;
    let y = &x;
    let aa = assert_eq!(5, x);
    let bb = assert_eq!(5, *y);

    let z = Box::new(x);
    assert_eq!(5, *z);

    let xx = 5;
    let yy = MyBox::new(xx);
    assert_eq!(5, *yy);
    assert_eq!(5, xx)
}
