use std::io;

fn main() {
    let mut input = String::new();
    let mut counter = 1;
    loop {
        println!(
            "I am the beginning of the end, and the end of time and space. \
            I am essential to creation, and I surround every place. \
            What am I?"
        );
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "the letter e" {
            println!("Number of trials: {}", counter);
            break;
        }
        input.clear();
        counter += 1;
    }
}
