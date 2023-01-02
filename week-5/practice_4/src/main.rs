use std::io;

fn main() {
    let mut name = String::new();
    let mut age = String::new();
    
    println!("\nEnter your name:"); 
    io::stdin().read_line(&mut name).expect("Failed to read input"); 

    println!("\n Enter your age:"); 
    io::stdin().read_line(&mut age).expect("Not a valid string"); 
    let age:i32 = age.trim().parse().expect("Not a valid number"); 

    if age >=18 {
        println!("Here's the invitation code to the party {}",name ) ;
    } else {
        
      println!("\nYour too young to come to this party");
    }
}
