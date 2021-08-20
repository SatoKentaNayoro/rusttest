fn main() {
    let a = [1, 2, 3];
    for info in a.iter().skip(1) {
        println!("{}", info)
    }
}
