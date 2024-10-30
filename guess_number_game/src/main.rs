use rand::Rng;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();
    let mut input = String::new();
    let range = 0u32..100;

    // gen random number
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(range.clone());

    loop {
        // read from user
        println!("Enter number from range [0, 100): ");
        lock.read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(number) => {
                if !range.contains(&number) {
                    println!("Number out of range!");
                    continue;
                }

                // check number
                if number == secret_number {
                    println!("You win!");
                    break;
                }

                if number < secret_number {
                    println!("Too small!");
                } else {
                    println!("Too big!");
                }
            }
            Err(_) => {
                println!("Invalid number!");
            }
        }
        input.clear();
    }
}
