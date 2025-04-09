fn main() {
    let my_string = "Hello";

    match my_string {
        "Hello" => println!("Anglais"),
        "Bonjour" => println!("Français"),
        "Hola" => println!("Espagnol"),
        "Ciao" => println!("Italien"),
        _ => println!("Autre"),
    }
}