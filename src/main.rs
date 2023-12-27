use std::io;

fn main() {
    println!("Guess the number: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let apples: i8; //still immutable
    apples = 8;
    //apples = 10; //forbidden because it is immutable
    println!("The apples are {apples}!");
}
