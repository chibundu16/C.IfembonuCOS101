use std::io;


fn main() {
 
 println!("Enter your  details" );

  let mut name = String::new();
  let mut department = String::new();
  let mut state = String::new();
  let mut email = String::new();
  let mut grade = String::new();
  let mut class = String::new();
  let mut rep = String::new();


  println!("Enter your name:");
io::stdin().read_line(&mut name).expect("Not a valid string");


println!("What department are you in ?:");
io::stdin().read_line(&mut department).expect("Not a valid integer");
 

println!("Enter your state of origin:");
io::stdin().read_line(&mut state).expect("Not a valid number");

println!("Enter your Email:");
io::stdin().read_line(&mut email ).expect("Not a valid string");


println!("Enter your currrent CGPA:");
io::stdin().read_line(&mut grade).expect("Not a valid number");
let _cgpa:f32 = grade.trim().parse().expect("Not a valid input");

println!("what level are you in ?:");
io::stdin().read_line(&mut class).expect("Not a valid number");
let _level:i32 = class.trim().parse().expect("Not a valid input");

println!("Are you a class rep ? (yes/no):");
io::stdin().read_line(&mut rep ).expect("Not a valid string");

let mut irep = true;

if rep.trim() == "yes" {irep = true};

if irep && _level >100 && _cgpa > 4.0 {

       println!("");
        println!("Name: {} ",name);
        println!("Department: {}", department);
        println!("State of Origin: {}", state);
        println!("Email: {}", email);
         println!("You have met all required qualifications therefore you are elligible to vote");


     }else{
        println!("SORRY YOU ARE NOT ELLIGIBLE TO VOTE");
     }

}
