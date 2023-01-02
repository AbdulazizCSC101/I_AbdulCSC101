use std::io; 

fn main() {
    let mut input1 = String::new(); 
    let mut input2 = String::new(); 
    println!("Enter lower bound : "); 
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let lower_bound:i32 = input1.trim().parse().expect("Not a valid number"); 

    println!("Enter Upper bound : " ); 
    io::stdin().read_line(&mut input2).expect("Not a valid String"); 
    let upper_bound:i32 = input2.trim().parse().expect("Not a valid number") ;

    for x in lower_bound..upper_bound{
        println!("Count level is {}",x );
    }
}
