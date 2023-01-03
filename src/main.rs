fn main() {
    let mut name = "Nisvet";
    say_hey(name);

    name = "John";
    say_hey(name);
}

fn say_hey(name: &str) {
    println!("Hey, hey! My name is {name}.");
}
