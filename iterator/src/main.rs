fn main() {
    println!("Hello, world!");
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();
    // for val in v1_iter {
    //     println!("{}", val)
    // }
    if let Some(v) = v1_iter.next() {
        println!("{}",v);
    }
    if let Some(v) = v1_iter.next() {
        println!("{}",v);
    }
    if let Some(v) = v1_iter.next() {
        println!("{}",v);
    }
    if let Some(v) = v1_iter.next() {
        println!("{}",v);
    }else {
        println!("At end");
    }
    let v2 = vec![2,3,6];
    let v2_iter = v2.iter();
    let total:i32 = v2_iter.sum();
    println!("{}",total);

    let mut v1 = vec![1,2,3,4];
    let mut v1_iter = v1.iter_mut();
    if let Some(v) = v1_iter.next() {
        *v = 3;
    }
    println!("{:?}",v1);
    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("{:?}",v2);

    let v1 = vec![1,2,3,7,6];
    let v2: Vec<_> = v1.into_iter().filter(|x | *x > 5).collect();
    println!("{:?}",v2);


}
