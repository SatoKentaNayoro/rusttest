// 定义特征
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

trait SchoolName {
    fn get_school_name(&self) -> String {
        // ("清华大学".to_string())
        String::from("清华大学")
    }
}

// 实现trait
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl SchoolName for Student {}

impl SchoolName for Teacher {
    fn get_school_name(&self) -> String {
        String:: from("北京大学")
    }
}


pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl GetInformation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
    // fn get_subject(&self) -> &String {
    //     &self.subject
    // }
}

fn print_information(item: impl GetInformation) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

fn main() {
    let s = Student { name: String::from("小明"), age: 11 };
    let t = Teacher { name: String::from("老王"), age: 30, subject: "语文".to_string() };
    // println!("student's name is {},age is {}", s.get_name(), s.get_age());
    // println!("teacher's name is {},age is {},subject is {}", t.get_name(), t.get_age(), t.subject);
    // println!("Hello, world!");

    let s_school_name = s.get_school_name();
    let t_school_name = t.get_school_name();
    print_information(s);
    print_information(t);
    println!("student's school is {}", s_school_name);
    println!("student's school is {}", t_school_name);
}
