use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); 

    // Since we don't want the function to 
    // end if we enter the wrong input,
    // we would use a loop

    loop {
        println!("Enter a guess");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Now the input we recieved is string,
        // But we need it to be an integer to 
        // compare it later
        // So we are going to change the type

        // The program might panic if we enter
        // something that is not a number
        // to handle that, we can use the match keyword
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // What we did here is called shadowing
        // That is creating a new varible with the 
        // same name so it changes the type
    
        // And since the compiler won't know
        // what to parse the string to, we 
        // had to specify the type to u32
    
        println!("You Guessed {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                // Break is used to get out of
                // the loop once we win.
                break;
            },
            Ordering::Greater => println!("{}", "Too big!".red())
        }
    }    
}
