use std::io;

fn checker() {
    let mut input = String::new(); 
    println!("Enter character : "); 
   io::stdin().read_line(&mut input).expect("Failure"); 
   let ch:i32 = input.trim().parse().expect("Wrong input"); 

   if ch >=0  && ch <= 9 
   {
    println!("Character '{}' is a digit", ch );
   }
   else {
   println!("Character'{}' is not a digit",ch );
  }
}


fn main() {
    println!("Welcome! This program checks whether a character
        variable contins a digit or not"); 
    
    checker();
}
