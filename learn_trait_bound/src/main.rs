// fn print_information<T: GetInformation>(item: T) {
//
// }

trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

// 1
fn print_information<T: GetName + GetAge>(item: T) {
    println!("name={}", item.get_name());
    println!("age={}", item.get_age());
}

// 2
fn print_information2<T>(item: T)
    where T: GetAge + GetName
{
    println!("name={}", item.get_name());
    println!("age={}", item.get_age());
}

fn product_item_with_age() -> impl GetAge + GetName {
    Student {
        name: "小李".to_string(),
        age: 24,
    }
}

fn main() {
    let s = Student { name: "xiaoming".to_string(), age: 32 };
    print_information(s);
    // print_information2(&s);
    let s2 = product_item_with_age();
    println!("s2 age is {}",s2.get_age());
    println!("{}",s2.get_name())
}
