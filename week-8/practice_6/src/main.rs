fn main() {
    let data_tuple: (&str, f32, u8) = ("Rust", 3.14, 100); 
    println!(" Tuple contents = {:?}", data_tuple ); 

    let no_datatype_tuple = ("Rust", "fun", 100); 
    println!("Tuple contents = {:?}", no_datatype_tuple ); 


    println!("Value at index 0 = {}", data_tuple.0 ); 

    println!("Value of Index 1 ={}", data_tuple.1 ); 

    println!("Value at Index 2 ={}",data_tuple.2 );
}
