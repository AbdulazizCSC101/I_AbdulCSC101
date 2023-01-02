use std::io::Write;

fn main() {
 let larger = "33 Export, Desperados, Goldberg, Gulder, Heineken, Star"; 
 let stout = "Legend, Turbo king, Non Alcoholic";
 let non_alcoholic = "Maltina, Amstel Malta, Malta Gold,Fayrouz";   
  
  let mut beverages = std::fs::File::create("Nigeria Beweries Plc.txt").expect("Write failed"); 
  beverages.write_all("Welcome to Nigeria beweries Company ! 
                   \nTheir rich porfolio which make them no 1 in the world are :".as_bytes()).expect("Write failed"); 
  beverages.write_all("Larger :".as_bytes()).expect("wrte failed");
  beverages.write_all(larger.as_bytes()).expect("Variable couldnt comprehend"); 
  
  beverages.write_all("Stout :".as_bytes()).expect("Write failed") ;
  beverages.write_all(stout.as_bytes()).expect("Expectation failed"); 
  
  beverages.write_all(" Non alcoholic : ".as_bytes()).expect("write failed"); 
  beverages.write_all(non_alcoholic.as_bytes()).expect("immutable"); 
  
  println!("Data has been transferred to the file");

}
