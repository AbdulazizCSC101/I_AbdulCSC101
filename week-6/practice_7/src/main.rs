fn main() {
    let arr1:[i32;4] = [10,20,30,40]; 
    println!("\n Array with data type"); 
    println!("array is {:?}",arr1 ); 
    println!("array size is :{}",arr1.len() ); 

    let arr2 = [10.4,20.7,30.4,40.9,51.2,72.2];
    println!("\n Array without data type"); 
    println!("Array is {:?}",arr2 ); 
    println!("array size is : {}",arr2.len() ); 


    let arr3:[i32;8] = [-1;8];
    println!("\n Array with default values"); 
    println!("array size is {:?}", arr3 ); 
    println!("array size is : {}", arr3.len());
}