use std::io;
use rand::Rng;

fn main() {
    let mut rng= rand::thread_rng();
    let to_guess = rng.gen_range(1..50);

    for _x in 0..3 {
        println!("Guess a number between 1-50");
        let mut input_text =  String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read input");

        let trimmed = input_text.trim();
        let mut guess = 0;
        match trimmed.parse::<u32>() {
            Ok(i) => guess = i,
            Err(..) => println!("this was not an integer: {}", trimmed),
        };

        if guess < to_guess {
            println!("Too Low!");
        }
        else if guess > to_guess {
            println!("Too High!");
        }
        else {
            println!("you Win!");
            break;
        }
    }

    println!("you Lose! The number was {}", to_guess);

}

// fn add(num1: u32, num2: u32) -> u32 {
//     num1 + num2
// }