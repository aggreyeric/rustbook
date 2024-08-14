
use rand::Rng;
use std::cmp::Ordering;
pub mod t;
use rand;

pub fn chapter2(){
    let secret = rand::thread_rng().gen_range(1..=10);
     
    loop {
        let mut guess_buffer = String::new();

        std::io::stdin().read_line(&mut guess_buffer).expect("Failed to read line");

        let guess: i32 = guess_buffer.clone().trim().parse().expect("Please type a number!");

        match guess.cmp(&secret) {
            Ordering::Less => println!("LESS {}", secret),
            Ordering::Greater => println!("GREATER {}", secret),
            Ordering::Equal => {
                println!("EQUAL");
                break;
            }   
            
        };
        
    }
 
}