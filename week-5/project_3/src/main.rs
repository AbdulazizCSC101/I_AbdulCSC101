
fn main() {
    
   //print hello program
println!("Welcome to Abdulinyo Fast food Delivery"); 
//print out the menu 
println!("Menu                            Price
P = Poundo Yam/Edinkaiko Soup       - N3,200
F = Fried Rice & Chicken            - N3,000
A = Amala & Ewedu Soup             - N2,500
E = Eba & Egusi Soup               - N2,000
W = White Rice & Stew              - N2,500
"); 
//    Put in the price of Pounded yam, fried rice, amala and ewedu soup, eba and egusi soup, white rice and stew 
let price_of_p = 3_200.0; 
let price_of_f = 3_000.0; 
let price_of_a = 2_500.0; 
let price_of_e = 2_000.0; 
let price_of_w = 2_500.0;

//    Input the quantity of pounded yam,fried rice, amala and ewedu soup, eba and egusi soup, white rice and stew 
println!("How many quantities of Pounded yam do you want?"); 
let mut input1 = String::new(); 
std::io::stdin().read_line(&mut input1).expect("hfd"); 
let qty_p:f32 = input1.trim().parse().expect("Failed");  

let mut input2 = String::new();
println!("How many quantities of  Fried Rice do you want ?"); 
std::io::stdin().read_line(&mut input2).expect("jjk"); 
let  qty_f:f32 = input2.trim().parse().expect("Failed");


let mut input3 = String::new(); 
println!("How many Quantities of Amala and Ewedu soup do you want ?");
std::io::stdin().read_line(&mut input3).expect("hfd"); 
let qty_a:f32 = input3.trim().parse().expect("Failed");  

let mut input4 = String::new(); 
println!("How many quantities of eba and egusi soup do you want");
std::io::stdin().read_line(&mut input4).expect("hfd"); 
let qty_e:f32 = input4.trim().parse().expect("Failed");  

let mut input5 = String::new(); 
println!("How many quantities of white rice and stew do you want");
std::io::stdin().read_line(&mut input5).expect("hfd"); 
let qty_w:f32 = input5.trim().parse().expect("Failed"); 

//  Let bill = price of pounded yam * quantity of pounded yam +price of fried rice * quantity of fried rice 
//+ price of amala and ewedu soup + price of eba and egusi soup * quantity of eba and egusi soup +
// price of white rice and stew * quantity of white rice and stew  

let bill :f32 =  ( price_of_p * qty_p) + (price_of_f * qty_f) + (price_of_a * qty_a) + (price_of_e * qty_e) + (price_of_w * qty_w ); 
println!("The bill is {}", bill);
//  Put an if statement when bill is greater than 10000;  
if bill > 10_000.0 
{
    let final_bill:f32= bill * (1.0 - 0.05);  
    println!("Your bill is greater than 10,000 so you have a 5% disount. 
            Your bill is :\n {}", final_bill);
}

}
