fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    println!("tableau : {:?}", v);

    let s = &v[1..3];
    println!("tableau slice : {:?}", s);
}