use std::io;  
// Create a function
fn trapezium() {
    //input their values
    println!("Calculate the area of trapezium");
    
    println!("Enter A : "); 
    let mut base1 = String::new(); 
    io::stdin().read_line(&mut base1).expect("Failed to read"); 
    let a:f32 = base1.trim().parse().expect("Invalid"); 

    println!("Enter B: "); 
    let mut base2 = String::new(); 
    io::stdin().read_line(&mut base2).expect("Failed to read"); 
    let b:f32 = base2.trim().parse().expect("Invalid"); 

    println!("Enter height : "); 
    let mut eno = String::new(); 
    io::stdin().read_line(&mut eno).expect("Failed to read"); 
    let h:f32 = eno.trim().parse().expect("Invalid"); 

    let t:f32 = 1.0 / 2.0 * (a + b) * h ;
// Calculate trapezium
  println!("Area of trapezium is {}",t );

}  

fn rhombus() {
     // input their values
     println!("Calculate the area of rhombus");
     
     println!("Enter diagonal 1: "); 
     let mut input1 = String::new(); 
     io::stdin().read_line(&mut input1).expect("Failed"); 
     let c:f32 = input1.trim().parse().expect("Failed");  

     println!("Enter diagonal 2: "); 
     let mut input2 = String::new(); 
     io::stdin().read_line(&mut input2).expect("Failed"); 
     let d:f32 = input2.trim().parse().expect("Failed"); 
// Calculate rhombus
     let rhombus:f32 = 1.0 / 2.0 * c * d ; 



     println!("Area of the rhombus is {}",rhombus ); 
}   


fn parallelogram() {
    // input their values
    println!("Calculate the area of parallelogram");
    
    println!("Enter Base :");  
    let mut input3 = String::new(); 
     io::stdin().read_line(&mut input3).expect("Failed"); 
     let base:f64 = input3.trim().parse().expect("Failed"); 

     println!("Enter altitude : ");  
     let mut input4 = String::new(); 
     io::stdin().read_line(&mut input4).expect("Failed"); 
     let altitude:f64 = input4.trim().parse().expect("Failed"); 
// Calculate area of parallelogram
     let par:f64 = base * altitude;  

    println!("Area of parallelogram is {}",par ); 


}


fn cube() {
    //input their values
    println!("Calculate the area of the cube" ); 

    println!("Enter length of side : "); 
    let mut input5 = String::new(); 
     io::stdin().read_line(&mut input5).expect("Failed"); 
     let l:i32 = input5.trim().parse().expect("Failed"); 
     //calculate the area of the cube
     let cube:i32 = 6 * i32::pow(l,2);  

     println!("Area of the cube is {}",cube );

} 



fn cylinder () {
    //input their values    
    println!("Calculate the volume of the cylinder"); 

    let pi:f32 = 3.14 ; 


    println!("Enter radius  : "); 
    let mut input6 = String::new(); 
     io::stdin().read_line(&mut input6).expect("Failed"); 
     let r:f32 = input6.trim().parse().expect("Failed"); 

     println!("Enter height : "); 
     let mut input7 = String::new(); 
     io::stdin().read_line(&mut input7).expect("Failed"); 
     let h:f32 = input7.trim().parse().expect("Failed"); 
    // calculate the volume of the cylinder
     let cylinder:f32 = pi * f32::powf(r,2.0) * h; 

     println!("Volume of the cylinder is {}",cylinder ); 


} 


fn main() {
    trapezium(); 

    rhombus(); 

    parallelogram(); 

    cube(); 

    cylinder();


}