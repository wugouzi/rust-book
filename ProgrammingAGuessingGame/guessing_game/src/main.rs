use std::io;

fn main() {
    // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won't change.
    println!("test");
    let mut guess = String::new();

    // & indicates that this argument is a *reference*
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("Fuck you");
}
