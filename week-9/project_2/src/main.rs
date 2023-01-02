 

fn main() {
    let student_name = ["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"]; 
    let matric_number = ["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"]; 
    let department = ["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = [300,100,200,200,100];
    
    let all = [student_name,matric_number,department,level]; 
    println!("PAU  Student Information System"); 

    for i in all.iter() 
    {

        print!("The student name is {:?}.He/she department is {:?} in level {:?}.The matric number given by the school is {:?}. ",
            student_name,department,matric_number ,level);
    }
}
