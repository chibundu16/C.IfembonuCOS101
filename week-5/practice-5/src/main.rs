use std::io;

fn main() {
    println!("Enter a number");
    let mut input = String::new();
      io::stdin().read_line(&mut input).expect("Failed to read input");
let mut num:i32 = input.trim().parse().expect("Failed to input");


while num < 100 {
    println!("inside the loop value is {}",num);
    num+=2;
}
println!("outside loop number value is {}",num );
}
