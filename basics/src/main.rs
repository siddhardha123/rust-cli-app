//imports
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let mut guess = String::new();

   loop {
    io::stdin().read_line(&mut guess).expect("failed to read line");
    let secret = rand::thread_rng().gen_range(1,101);
    let guess: u32 =  guess.trim().parse().expect("parsing error");
    match guess.cmp(&secret) {
        Ordering::Less => println!("less than"),
        Ordering::Greater => println!("more than"),
        Ordering::Equal => {println!("you win");
        break;
       },
    }
   }

}


