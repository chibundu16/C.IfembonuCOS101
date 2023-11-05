use std::io;


fn main() {
    let mut distance = String::new();
    let mut time = String::new();

    println!("Enter your distance in miles");
    io::stdin().read_line(&mut distance).expect("Not a valid string");
    let a:f32 = distance.trim().parse().expect("Not a valid number");

        println!("Enter your time in hour");
    io::stdin().read_line(&mut time).expect("Not a valid string");
    let b:f32 = time.trim().parse().expect("Not a valid number");


    let x:f32 = (a * 0.621371) / b;

    println!("The speed of the car is {}",x)
}
