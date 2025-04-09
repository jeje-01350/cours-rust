fn main() {
    let age: i32 = 20;

    if age >= 18 {
        println!("Vous êtes majeur");
    } else {
        println!("Vous êtes mineur");
    }

    if age >= 18 && (age == 20 || age == 21) {
        println!("ok");
    } 
}