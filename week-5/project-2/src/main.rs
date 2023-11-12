
use std::io;

fn main() {
 let mut age = String::new();
 let  mut exp =String::new();

 println!("Enter your age");
 io::stdin().read_line(&mut age).expect("not a valid input");
 let age:i32 = age.trim().parse().expect("not a valid number");

  println!("are you experienced(yes/no)");
 io::stdin().read_line(&mut exp).expect("not a valid input");
 

let mut isexp = false;

if exp.trim() == "yes"{
    isexp = true;
} 
if !isexp{
    println!("Your incentive is 100,000");
}else if age >=40 {
    println!("Your incentive is 1,560,00");
}else if age>=30 {
println!("Your incentive is 1,480,000");
}else{
    println!("Your incentive is 1,300,000");
   }
}
