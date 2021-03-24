fn main() {
    let rect1 = Rect::build_rect(30, 50);
    println!(
        "The area od the rectangle is {} square pixels.", rect1.area()
    );
    let mut b = rect1;
    b.width = 10;
    println!(
        "The area od the rectangle is {} square pixels.", b.area()
    );
    println!("{:#?}\n{:#?}", rect1,b)
}


// struct Rect(i32,i32);

#[derive(Debug,Copy,Clone)]
struct Rect {
    width: i32,
    height: i32,
}



impl Rect {
    fn area(self) -> i32 {
        self.width * self.height
    }

    fn build_rect(width: i32, height: i32) -> Rect {
        Rect {
            width,
            height,
        }
    }
}