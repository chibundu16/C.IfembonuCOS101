use std::io;

fn main() {
    println!("");
    println!("RESEARCHERS PUBLICATION INCENTIVE ALLKOCATION");
    println!("");
    println!("Enter your  details" );

  let mut name = String::new();
  let mut papers = String::new();

println!("");
    println!("Enter your name:");
io::stdin().read_line(&mut name).expect("Not a valid string");

println!("Enter number of papers published:");
io::stdin().read_line(&mut papers).expect("Not a valid number");
let _paper:i32 = papers.trim().parse().expect("Not a valid input");


if _paper >5 && _paper <10{
    println!( "Dear Mr/Mrs {}  Your incentive is 800,000",name);
}else if _paper >=3 && _paper <=5{
     println!( "Dear Mr/Mrs {} Your incentive is 500,000",name); 
}else if _paper >=10 {
     println!( "Dear Mr/Mrs {} Your incentive is 1,000,000",name);
 }else if _paper <3{
   println!( "Dear Mr/Mrs {} Your incentive is 100,000",name); 
 }

  
}
