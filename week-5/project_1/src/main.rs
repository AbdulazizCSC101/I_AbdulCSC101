use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new(); 
    let mut input3 = String::new(); 

    println!("Enter a:"); 
    io::stdin().read_line(&mut input1).expect("Not a valid String"); 
    let a:i32 = input1.trim().parse().expect("Not a valid number"); 

    println!("Enter b:"); 
    io::stdin().read_line(&mut input2).expect("Not a valid String"); 
    let b:i32 = input2.trim().parse().expect("Not a valid number"); 

    println!("Enter c :"); 
    io::stdin().read_line(&mut input3).expect("Not a valid string"); 
    let c:i32 = input3.trim().parse().expect("Not a valid String"); 

    let discriminant:i32 = b^2 - 4 * a * c;  
    println!("The discriminant is {}", discriminant );
    if discriminant > 0 
    {
        println!("They have two distinct roots");
    }
    else if discriminant == 0 
    {
        println!("it has only one real root");
    }
     else 
     {
        println!("They are no real roots");
     }
}

