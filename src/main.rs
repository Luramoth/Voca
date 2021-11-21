//use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main (){
    println!("fuck torvalds I swear to god I hate git and everyhitng it stands for");

    let number = rand::thread_rng().gen_range(1, 10);

    loop {
        println!("now guess a number from 1-10 for no reason lol, psst, its {} btw", number);

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess:u32  = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("you guessed: {}", guess);
    
        match guess.cmp(&number) {
            Ordering::Greater => println!("{}", " and it was too big dummy".red()),
            Ordering::Less => println!("{}", " and it was too small idiot".red()),
            Ordering::Equal => {
                println!("{}", " and it was correct".green());
                break;
            },
        }
    }
}
