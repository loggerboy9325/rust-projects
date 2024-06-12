fn main() {
    let colors = ["red", "green", "blue"];
    let time = std::time::UNIX_EPOCH
        .elapsed()
        .expect("Call the doctor, time went backwards!")
        .as_millis() as usize;

    let actual =  colors[time % colors.len()];
    println!("Welcome to the guessing game!");
    println!("I have chosen a color: red, green or blue, can you guess which?");
    println!("Enter your guess: red, green or blue");
    let input = std::io::stdin()
        .lines()
        .next()
        .expect("No input was read")
        .expect("There was an error when reading the input");
    println!("Your guess was {input}");
    println!("The color I chose was {actual}");

    if input == actual {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
