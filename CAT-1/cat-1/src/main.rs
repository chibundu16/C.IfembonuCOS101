use std::io;

fn main() 
{
    println!("These are the villages we are considering ; 1)Akapbom, 2)Ngbauju, 3)Atabrikang, 4) Okorobilum, 5) Emeremen");
    println!("Enter your  details" );

  let mut name = String::new();
  let mut imdob = String::new();
  let mut ages = String::new();
  let mut email = String::new();
  let mut number = String::new();
  let mut numberofsiblings = String::new();
  let mut numberofchildren = String::new();
  let mut diagnosis = String::new();
  let mut residence = String::new();

  const COST:f32 = 1_200_000.00 ;
  const COST_2:f32 = 550_000.00 ;
  const COST_3:f32 = 1_500_000.00 ;
  const COST_4:f32 = 800_000.00 ;
  const COST_5:f32 = 400_000.00 ;

println!("Enter your name:");
io::stdin().read_line(&mut name).expect("Not a valid string");


println!("Enter your dob:");
io::stdin().read_line(&mut imdob).expect("Not a valid integer");
 

println!("Enter your age:");
io::stdin().read_line(&mut ages).expect("Not a valid number");
let age:i32 = ages.trim().parse().expect("Not a valid input");

println!("Enter your Email:");
io::stdin().read_line(&mut email ).expect("Not a valid string");


println!("Enter your phone number:");
io::stdin().read_line(&mut number).expect("Not a valid number");
let number:i32 = number.trim().parse().expect("Not a valid input");

println!("Enter number of siblings:");
io::stdin().read_line(&mut numberofsiblings).expect("Not a valid number");
let siblings:i32 = numberofsiblings.trim().parse().expect("Not a valid input");

println!("Enter number of children:");
io::stdin().read_line(&mut numberofchildren).expect("Not a valid number");
let children:i32 = numberofchildren.trim().parse().expect("Not a valid input");

println!("Enter your medical diagnosis:");
io::stdin().read_line(&mut diagnosis).expect("Not a valid string");


println!("Enter your residence:");
io::stdin().read_line(&mut residence).expect("Not a valid string");




if age >50  && children >4 && residence.trim() == "Akapbom"  {
        let discount:f32 = (COST as f32)-((COST as f32)*(20.0/100.0));
        println!("You qualify for a discount of 20%");
       
        println!("");
        println!("Name: {} ",name);
        println!("Dob: {}", imdob);
        println!("Age: {}", ages);
        println!("phone number: {}", number);
        println!("Email: {}", email);
        println!("Number of siblings: {}", numberofsiblings);
        println!("Number of children: {}", numberofchildren);
        println!("Medical diagnosis: {}", diagnosis);
        println!("Residence: {}", residence);
        println!("Your price is {}",discount );


    }else if age == 50 && children >4 && residence.trim() == "Ngbauju" {
               let discount:f32 = (COST_2 as f32)-((COST_2 as f32)*(5.0/100.0));
        println!("You qualify for a discount of 5%");
       
        println!("");
        println!("Name: {} ",name);
        println!("Dob: {}", imdob);
        println!("Age: {}", ages);
        println!("phone number: {}", number);
        println!("Email: {}", email);
        println!("Number of siblings: {}", numberofsiblings);
        println!("Number of children: {}", numberofchildren);
        println!("Medical diagnosis: {}", diagnosis);
        println!("Residence: {}", residence);
        println!("Your price is {}",discount ); 

    }else if  children >3 && siblings >3 && diagnosis.trim() == "CKD" && residence.trim() == "Atabrikang"{
               let discount:f32 = (COST_3 as f32)-((COST_3 as f32)*(15.0/100.0));
        println!("You qualify for a discount of 15%");
       
        println!("");
        println!("Name: {} ",name);
        println!("Dob: {}", imdob);
        println!("Age: {}", ages);
        println!("phone number: {}", number);
        println!("Email: {}", email);
        println!("Number of siblings: {}", numberofsiblings);
        println!("Number of children: {}", numberofchildren);
        println!("Medical diagnosis: {}", diagnosis);
        println!("Residence: {}", residence);
        println!("Your price is {}",discount ); 

    }else if age > 28 && age <45 && children >4 && diagnosis.trim() == "Diabetes" {
               let discount:f32 = (COST_4 as f32)-((COST_4 as f32)*(10.0/100.0));
        println!("You qualify for a discount of 10%");
       
        println!("");
        println!("Name: {} ",name);
        println!("Dob: {}", imdob);
        println!("Age: {}", ages);
        println!("phone number: {}", number);
        println!("Email: {}", email);
        println!("Number of siblings: {}", numberofsiblings);
        println!("Number of children: {}", numberofchildren);
        println!("Medical diagnosis: {}", diagnosis);
        println!("Residence: {}", residence);
        println!("Your price is {}",discount ); 

    }else if age > 58 && children >5 && siblings > 5 && diagnosis.trim() =="Athritis" && residence.trim() == "Emerenem" {
               let discount:f32 = (COST_5 as f32)-((COST_5 as f32)*(10.0/100.0));
        println!("You qualify for a discount of 5%");
       
        println!("");
        println!("Name: {} ",name);
        println!("Dob: {}", imdob);
        println!("Age: {}", ages);
        println!("phone number: {}", number);
        println!("Email: {}", email);
        println!("Number of siblings: {}", numberofsiblings);
        println!("Number of children: {}", numberofchildren);
        println!("Medical diagnosis: {}", diagnosis);
        println!("Residence: {}", residence);
        println!("Your price is {}",discount ); 
    }




}



