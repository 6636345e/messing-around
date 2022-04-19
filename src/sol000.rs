use std::io;

pub fn solve000() {
    println!("Program 000");
    let mut guess = String::new();
    println!("Enter a number?");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    if guess % 2 == 0 && guess != 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}
