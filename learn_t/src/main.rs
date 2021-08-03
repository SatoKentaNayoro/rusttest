//fn largest_i33(list: &[i32]) -> i32 {
//    let mut largest = list[0];
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}
//
//fn largest_char(list: &[char]) -> char {
//    let mut largest = list[0];
//    for &item in list.iter() {
//        if item > largest {
//            largest = item
//        }
//    }
//    largest
//
//}
// 使用泛型
//fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//    let mut largest = list[0];
//    for &item in list.iter() {
//        if item > largest {
//            largest = item
//        }
//    }
//    largest
//}
//
//
//fn main() {
//    let number_list = vec![1,2,23,34,8,100];
//    let max_number = largest(&number_list);
//    println!("max_number = {}",max_number);
//    let char_list = vec!['a','y','b'];
//    let max_char = largest(&char_list);
//    println!("max_char = {}",max_char);
//
//}
// 结构体中使用泛型
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// fn main() {
//     let integer = Point{x: 1,y: 2};
//     println!("{:#?}",integer);
//     let float = Point{x: 1.1,y: 2.2};
//     println!("{:?}",float);
//     let chars = Point{x: "a",y: "b"};
//     println!("{:?}",chars);
// }
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &U {
        &self.y
    }

    fn create_new<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:#?}", p.get_x());
    println!("{:#?}", p.get_y());
    let p = Point2 { x: 1, y: 2.2 };
    println!("{:#?}", p.get_x());
    println!("{:#?}", p.get_y());
    let p2 = Point2 { x: 3, y: "冷森森快打完" };
    let r = p.create_new(p2);
    println!("{:#?}", r)
}