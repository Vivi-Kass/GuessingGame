use std::io; //User input library

fn main() {
    
    println!("Please guess a number");

    let mut guess = String::new(); //mutable empty string so we can change it later

    io::stdin() //stdin processor
        .read_line(&mut guess) //Read line and save to guess. .read_line reads the line. &mut means mutable and argument reference rather than value
        .expect("Failed to read line"); //Excepption

    //io::stdin().read_line(&mut guess).expect("Failed to read line"); an alternative to the above line

    println!("You guessed: {guess}"); //Print guess
    
}
