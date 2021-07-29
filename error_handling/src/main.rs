use std::fs::File;
use std::io::Read;
use std::io;


fn main() {
    // let f = File::open("hello.txt");
    // let r = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("error:{:?}",error)
    // };
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello1.txt").expect("Failed to open hello.txt");
    // let r = read_username_from_file();
    // match r {
    //     Ok(s) => println!("{}", s),
    //     Err(error) => println!("{:?}", error),
    // }
    if let Ok(s) = read_username_from_file() {
        println!("{}",s)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(error) => Err(error),
    // }
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}