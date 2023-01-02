use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEmployees incentive!"); 
    println!("Do u have experience" );
    println!("\nPick 1 for yes ;pick 2 for no : ");
     let mut input1 = String::new(); 
     io::stdin().read_line(&mut input1).expect("gh");
     let input3:u32 = input1.trim().parse().expect("df");
     let input4 = 1; 
     
     
     let mut bools = false; 
       if  input3 == input4{
        bools = true;
     }
      
     
  

     
    println!("\nEnter age : "); 
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let age:u32 = input.trim().parse().expect("Not a valid number");  


    if  input3 > 2 
    {
        println!("\nStart Again remember Pick 1 for yes ;pick 2 for no ");
    }



     else if bools && age >=40 
    {
        println!("Your incentive is 1_560_000 ");
    }
    else if  bools && age >=30 && age < 40 
    {
        println!("Your incentive is 1_480_000 "); 
    }
     else if bools && age < 28 
     {
        println!("Your incentive is 1_300_000 per month");
     }
     else    
     {
        println!("Your incentive is 100,000");
     }
}
     