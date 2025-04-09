fn main() {
    println!("le plus grand nombre est {}", get_bigger(1, 2));
}

/* fn add(a: i32, b: i32) -> i32 {
    a + b
} */

fn get_bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
