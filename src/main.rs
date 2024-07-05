use std::io; //User input library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100); //thread_rng runs on this thread, range 1 to 100 inclusive


    loop {

        println!("Please guess a number");

        let mut guess = String::new(); //mutable empty string so we can change it later
    
        io::stdin() //stdin processor
            .read_line(&mut guess) //Read line and save to guess. .read_line reads the line. &mut means mutable and argument reference rather than value
            .expect("Failed to read line"); //Excepption
    
        //io::stdin().read_line(&mut guess).expect("Failed to read line"); an alternative to the above line
    
        let guess: u32 = guess.trim().parse().expect("Please enter a number.");
    
    
        println!("\nYou guessed: {guess}"); //Print guess
        //println!("The secret number is: {secret_number}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        };

    }




}
