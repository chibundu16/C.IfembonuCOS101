// rust program to determine age pass

use std::io;

fn main() {
 let mut input1 = String::new();
 let mut input2 = String::new();

 println!("Enter yoour name: " );
 io::stdin().read_line(&mut input1).expect("Not a valid string");

  println!("Enter yoour age: " );
 io::stdin().read_line(&mut input2).expect("Not a valid string");
  let age:i32 = input2.trim().parse().expect("Not a valid number");

  if age >= 18 {
    println!("welcome to the party {}", input1)
  } else {
    println!("oops, you are not of age to enter this party {}", input1);
  }
}
